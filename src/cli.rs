use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "neocom",
    version,
    about = "EVE Online CLI toolkit. Market prices, route safety, pilot intel, and more."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Fetch route and show danger rating per hop
    Travel(TravelArgs),
    /// Fetch market prices for items
    Price(PriceArgs),
    /// Pilot intel and kill history
    Intel(IntelArgs),
    /// System overview
    System(SystemArgs),
    /// Wormhole sites for a class
    Wh(WhArgs),
    /// EVE server status
    Status,
    /// Generate shell completion scripts
    #[clap(hide = true)]
    Generate(GenerateArgs),
}

/// Generate shell completion scripts
#[derive(Args)]
pub struct GenerateArgs {
    /// Shell type: bash, zsh, fish, powershell, elvish
    pub shell: String,
}

#[derive(Args, Clone)]
pub struct TravelArgs {
    pub origin: String,
    pub destination: String,
    #[arg(long, default_value = "1")]
    pub hours: u32,
    #[arg(long, default_value = "shortest")]
    pub route: RouteFlag,
}

#[derive(Args, Clone)]
pub struct PriceArgs {
    pub item: Option<String>,
    pub quantity: Option<u32>,
    #[arg(long, short)]
    pub file: Option<String>,
    #[arg(long)]
    pub buy: bool,
    #[arg(long)]
    pub sell: bool,
    #[arg(long, default_value = "10000002")]
    pub region: String,
}

#[derive(Args)]
pub struct IntelArgs {
    pub pilot: String,
}

#[derive(Args)]
pub struct SystemArgs {
    pub name: String,
}

#[derive(Args)]
pub struct WhArgs {
    pub class: String,
    #[arg(long, default_value = "all")]
    pub wh_type: WhType,
}

#[derive(clap::ValueEnum, Clone, Default, Debug)]
pub enum RouteFlag {
    /// Shortest route (default)
    #[default]
    Shortest,
    /// Prefer high-sec systems
    Safest,
    /// Only high-sec systems
    Secure,
    /// Prefer low-sec systems
    Insecure,
}

#[derive(clap::ValueEnum, Clone, Default, Debug)]
pub enum WhType {
    #[default]
    All,
    Combat,
    Relic,
    Gas,
}

pub fn parse() -> Cli {
    Cli::parse()
}
