mod cli;
mod commands;
mod display;
mod esi;
mod systems;
mod zkill;

use anyhow::Result;
use clap::CommandFactory;
use clap_complete::Shell;

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
        cli::Command::Generate(cmd) => {
            use clap_complete::generate;
            let shell = match cmd.shell.as_str() {
                "bash" => Shell::Bash,
                "zsh" => Shell::Zsh,
                "fish" => Shell::Fish,
                "powershell" => Shell::PowerShell,
                "elvish" => Shell::Elvish,
                _ => anyhow::bail!(
                    "Unsupported shell: {}. Use bash, zsh, fish, powershell, or elvish",
                    cmd.shell
                ),
            };
            let mut cmd = cli::Cli::command();
            generate(shell, &mut cmd, "neocom", &mut std::io::stdout());
            return Ok(());
        }
    }
}
