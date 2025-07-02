use clap::Parser;
use MiniGrep;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The string to search for
    query: String,

    /// The file to search in
    filename: String,

    /// Case insensitive search
    #[arg(long)]
    case_insensitive: bool,
}

fn main() {
    let cli = Cli::parse();

    let config = MiniGrep::Config {
        query: cli.query,
        filename: cli.filename,
        case_sensitive: !cli.case_insensitive,
    };

    if let Err(e) = MiniGrep::run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}