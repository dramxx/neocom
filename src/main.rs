mod cli;
mod commands;
mod display;
mod esi;
mod zkill;

use anyhow::Result;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = cli::parse();
    match args.command {
        cli::Command::Travel(cmd) => commands::travel::run(cmd),
        cli::Command::Price(cmd) => commands::price::run(cmd),
        cli::Command::Intel(cmd) => commands::intel::run(cmd),
        cli::Command::System(cmd) => commands::system::run(cmd),
        cli::Command::Wh(cmd) => commands::wh::run(cmd),
        cli::Command::Status => commands::status::run(),
    }
}
