//! `dos2unix-rust` — a minimal CRLF → LF converter.
//!
//! Streaming, O(1) memory, 8KB buffer. Supports in-place file conversion,
//! stdin/stdout piping, a check-only mode, and a quiet mode for scripts.

use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{Context, Result};

const BUFFER_SIZE: usize = 8 * 1024;

/// Exit code returned by `--check` when CRLF is found.
const EXIT_CRLF_FOUND: u8 = 2;

#[derive(Debug)]
struct Cli {
    check: bool,
    quiet: bool,
    files: Vec<PathBuf>,
}

fn print_usage(program: &str) {
    eprintln!(
        "Usage: {program} [OPTIONS] [FILE]...

Convert CRLF (\\r\\n) line endings to LF (\\n).

Options:
  -c, --check     Check for CRLF without modifying; exit {EXIT_CRLF_FOUND} if found.
  -q, --quiet     Suppress informational output.
  -h, --help      Show this help message.
  -v, --version   Show version.

If no FILE is given (or FILE is '-'), reads from stdin and writes to stdout.
Files are converted in place unless --check is set.",
    );
}

fn parse_args(argv: Vec<String>) -> Result<Cli> {
    let mut check = false;
    let mut quiet = false;
    let mut files = Vec::new();
    let program = argv
        .first()
        .map(String::as_str)
        .unwrap_or("dos2unix-rust")
        .to_string();
    let mut iter = argv.into_iter().skip(1);

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print_usage(&program);
                std::process::exit(0);
            }
            "-v" | "--version" => {
                println!("dos2unix-rust {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            "-c" | "--check" => check = true,
            "-q" | "--quiet" => quiet = true,
            "--" => {
                files.extend(iter.by_ref().map(PathBuf::from));
            }
            s if s.starts_with('-') && s.len() > 1 => {
                // Allow combined short flags like -cq.
                for ch in s[1..].chars() {
                    match ch {
                        'c' => check = true,
                        'q' => quiet = true,
                        'h' => {
                            print_usage(&program);
                            std::process::exit(0);
                        }
                        'v' => {
                            println!("dos2unix-rust {}", env!("CARGO_PKG_VERSION"));
                            std::process::exit(0);
                        }
                        other => anyhow::bail!("unknown option '-{other}'"),
                    }
                }
            }
            _ => files.push(PathBuf::from(arg)),
        }
    }

    Ok(Cli {
        check,
        quiet,
        files,
    })
}

fn main() -> Result<ExitCode> {
    let cli = parse_args(std::env::args().collect())?;

    if cli.files.is_empty() {
        let stdin = io::stdin();
        let stdout = io::stdout();
        if cli.check {
            let found = check_stream(stdin.lock())?;
            if found && !cli.quiet {
                eprintln!("stdin: CRLF line endings found");
            }
            return Ok(ExitCode::from(if found { EXIT_CRLF_FOUND } else { 0 }));
        }
        convert_stream(stdin.lock(), stdout.lock(), !cli.quiet)?;
        return Ok(ExitCode::SUCCESS);
    }

    let mut any_found = false;
    for file in &cli.files {
        if cli.check {
            let found = check_file(file).with_context(|| format!("check {}", file.display()))?;
            if found {
                any_found = true;
                if !cli.quiet {
                    eprintln!("{}: CRLF line endings found", file.display());
                }
            } else if !cli.quiet {
                eprintln!("{}: clean", file.display());
            }
        } else {
            convert_file_in_place(file, !cli.quiet)
                .with_context(|| format!("convert {}", file.display()))?;
        }
    }

    if cli.check && any_found {
        Ok(ExitCode::from(EXIT_CRLF_FOUND))
    } else {
        Ok(ExitCode::SUCCESS)
    }
}

/// Detect CRLF in a buffered reader without loading the whole stream.
fn check_stream<R: BufRead>(mut reader: R) -> Result<bool> {
    let mut buf = [0u8; BUFFER_SIZE];
    let mut prev_cr = false;
    loop {
        let n = reader.read(&mut buf).context("read input stream")?;
        if n == 0 {
            break;
        }
        for &b in &buf[..n] {
            if prev_cr && b == b'\n' {
                return Ok(true);
            }
            prev_cr = b == b'\r';
        }
    }
    Ok(false)
}

fn check_file(path: &Path) -> Result<bool> {
    let f = OpenOptions::new()
        .read(true)
        .open(path)
        .with_context(|| format!("open {}", path.display()))?;
    check_stream(BufReader::new(f))
}

/// Convert CRLF → LF from a reader into a writer.
///
/// `announce` controls per-stream diagnostics (used for stdin/stdout mode).
fn convert_stream<R: Read, W: Write>(reader: R, writer: W, announce: bool) -> Result<()> {
    let mut reader = BufReader::with_capacity(BUFFER_SIZE, reader);
    let mut writer = BufWriter::with_capacity(BUFFER_SIZE, writer);
    let mut buf = [0u8; BUFFER_SIZE];
    let mut prev_cr = false;
    let mut converted = 0u64;

    loop {
        let n = reader.read(&mut buf).context("read input")?;
        if n == 0 {
            break;
        }
        let mut out = Vec::with_capacity(n);
        for &b in &buf[..n] {
            if prev_cr && b == b'\n' {
                // Drop the preceding CR (already pushed); emit LF only.
                out.pop();
                out.push(b'\n');
                converted += 1;
            } else {
                out.push(b);
            }
            prev_cr = b == b'\r';
        }
        writer.write_all(&out).context("write output")?;
    }

    // A trailing lone CR is preserved as-is.
    writer.flush().context("flush output")?;

    if announce && converted > 0 {
        eprintln!("converted ({converted} CRLF → LF)");
    }
    Ok(())
}

/// Convert a file in place: write to a temp sibling, then rename atomically.
fn convert_file_in_place(path: &Path, announce: bool) -> Result<()> {
    let src = OpenOptions::new()
        .read(true)
        .open(path)
        .with_context(|| format!("open {}", path.display()))?;

    let tmp_path = temp_sibling(path);

    {
        let dst = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&tmp_path)
            .with_context(|| format!("create {}", tmp_path.display()))?;
        convert_stream(BufReader::new(src), BufWriter::new(dst), false)?;
    }

    std::fs::rename(&tmp_path, path)
        .with_context(|| format!("rename {} -> {}", tmp_path.display(), path.display()))?;

    if announce {
        eprintln!("{}: converted", path.display());
    }
    Ok(())
}

fn temp_sibling(path: &Path) -> PathBuf {
    let mut name = path
        .file_name()
        .map(|n| n.to_os_string())
        .unwrap_or_default();
    name.push(".dos2unix.tmp");
    path.with_file_name(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn detects_crlf_in_stream() {
        let input = Cursor::new(b"a\r\nb\r\nc\n");
        assert!(check_stream(BufReader::new(input)).unwrap());
    }

    #[test]
    fn clean_stream_has_no_crlf() {
        let input = Cursor::new(b"a\nb\nc\n");
        assert!(!check_stream(BufReader::new(input)).unwrap());
    }

    #[test]
    fn lone_cr_is_not_crlf() {
        let input = Cursor::new(b"a\rb\nc\n");
        assert!(!check_stream(BufReader::new(input)).unwrap());
    }

    #[test]
    fn convert_stream_strips_cr_before_lf() {
        let input = Cursor::new(b"a\r\nb\r\nc\n");
        let mut output = Vec::new();
        convert_stream(input, &mut output, false).unwrap();
        assert_eq!(output, b"a\nb\nc\n");
    }

    #[test]
    fn convert_stream_preserves_lone_cr() {
        let input = Cursor::new(b"a\rb\r\nc\n");
        let mut output = Vec::new();
        convert_stream(input, &mut output, false).unwrap();
        assert_eq!(output, b"a\rb\nc\n");
    }

    #[test]
    fn convert_stream_empty_input() {
        let input = Cursor::new(b"");
        let mut output = Vec::new();
        convert_stream(input, &mut output, false).unwrap();
        assert_eq!(output, b"");
    }

    #[test]
    fn parse_flags_combined() {
        let cli = parse_args(vec!["dos2unix-rust".into(), "-cq".into()]).unwrap();
        assert!(cli.check);
        assert!(cli.quiet);
    }

    #[test]
    fn parse_long_flags() {
        let cli = parse_args(vec![
            "dos2unix-rust".into(),
            "--check".into(),
            "--quiet".into(),
        ])
        .unwrap();
        assert!(cli.check);
        assert!(cli.quiet);
    }
}
