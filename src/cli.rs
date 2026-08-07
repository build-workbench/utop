//! Command-line argument parsing without external dependencies.

use crate::model::SortKey;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Config {
    pub(crate) sort: SortKey,
    pub(crate) desc: bool,
    pub(crate) delay_ms: u64,
    pub(crate) filter: String,
    pub(crate) tree: bool,
}

/// Outcome of argument parsing: run with a config, print help, or print
/// version. Kept as a value so the parser stays pure (no exit/print inside).
#[derive(Debug)]
pub(crate) enum Action {
    Run(Config),
    Help,
    Version,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            sort: SortKey::Cpu,
            desc: true,
            delay_ms: 500,
            filter: String::new(),
            tree: false,
        }
    }
}

const USAGE: &str = "\
utop — a lightweight htop clone

Usage: utop [OPTIONS]

Options:
  -h, --help           Print this help and exit
  -s, --sort <KEY>     Initial sort key: cpu | mem | pid | name [default: cpu]
  -a, --asc            Start in ascending order [default: descending]
  -d, --delay <MS>     Refresh interval in milliseconds, clamped to 100..=5000
                       [default: 500]
  -f, --filter <STR>   Initial process filter (matches name or PID)
  -t, --tree           Start in tree view
  -V, --version        Print version and exit
";

/// Parses `std::env::args`, printing help/version or exiting on bad input.
pub(crate) fn parse() -> Config {
    match parse_args(std::env::args().skip(1)) {
        Ok(Action::Run(config)) => config,
        Ok(Action::Help) => {
            print!("{USAGE}");
            std::process::exit(0);
        }
        Ok(Action::Version) => {
            println!("utop {}", env!("CARGO_PKG_VERSION"));
            std::process::exit(0);
        }
        Err(err) => {
            eprintln!("utop: {err}\n\n{USAGE}");
            std::process::exit(2);
        }
    }
}

/// Pure parser; never prints or exits.
pub(crate) fn parse_args<I: IntoIterator<Item = String>>(args: I) -> Result<Action, String> {
    let mut args = args.into_iter();
    let mut config = Config::default();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => return Ok(Action::Help),
            "-V" | "--version" => return Ok(Action::Version),
            "-s" | "--sort" => {
                let value = args.next().ok_or("--sort requires a value")?;
                config.sort = match value.as_str() {
                    "cpu" => SortKey::Cpu,
                    "mem" => SortKey::Mem,
                    "pid" => SortKey::Pid,
                    "name" => SortKey::Name,
                    other => return Err(format!("invalid sort key '{other}'")),
                };
            }
            "-a" | "--asc" => config.desc = false,
            "-d" | "--delay" => {
                let value = args.next().ok_or("--delay requires a value")?;
                let ms: u64 = value
                    .parse()
                    .map_err(|_| format!("invalid delay '{value}'"))?;
                config.delay_ms = ms.clamp(100, 5000);
            }
            "-f" | "--filter" => {
                config.filter = args.next().ok_or("--filter requires a value")?;
            }
            "-t" | "--tree" => config.tree = true,
            other => return Err(format!("unknown argument '{other}'")),
        }
    }
    Ok(Action::Run(config))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    fn cfg(items: &[&str]) -> Config {
        match parse_args(args(items)).unwrap() {
            Action::Run(config) => config,
            other => panic!("expected Run, got {other:?}"),
        }
    }

    #[test]
    fn empty_args_yield_defaults() {
        assert_eq!(cfg(&[]), Config::default());
    }

    #[test]
    fn sort_and_asc_flags() {
        let config = cfg(&["--sort", "mem", "--asc"]);
        assert_eq!(config.sort, SortKey::Mem);
        assert!(!config.desc);
    }

    #[test]
    fn delay_is_clamped_to_bounds() {
        assert_eq!(cfg(&["--delay", "99999"]).delay_ms, 5000);
        assert_eq!(cfg(&["-d", "5"]).delay_ms, 100);
    }

    #[test]
    fn filter_and_tree_flags() {
        let config = cfg(&["-f", "rust", "-t"]);
        assert_eq!(config.filter, "rust");
        assert!(config.tree);
    }

    #[test]
    fn unknown_argument_is_an_error() {
        assert!(parse_args(args(&["--bogus"])).is_err());
    }

    #[test]
    fn missing_value_is_an_error() {
        assert!(parse_args(args(&["--sort"])).is_err());
    }

    #[test]
    fn help_returns_help() {
        assert!(matches!(parse_args(args(&["-h"])).unwrap(), Action::Help));
    }

    #[test]
    fn version_returns_version() {
        assert!(matches!(
            parse_args(args(&["--version"])).unwrap(),
            Action::Version
        ));
    }
}
