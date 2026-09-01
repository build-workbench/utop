//! utop — a lightweight htop clone. Thin binary entry point.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = utop::cli::parse();
    utop::run(&config)
}
