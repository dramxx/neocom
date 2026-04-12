use crate::cli::SystemArgs;
use crate::display::danger_label;
use anyhow::{Context, Result};

pub fn run(args: SystemArgs) -> Result<()> {
    let esi = crate::esi::EsiClient::new();
    let zkill = crate::zkill::ZkillClient::new();

    // Resolve system name to ID
    let resolved = esi
        .resolve_names(&[&args.name])?
        .pop()
        .context("Unknown system")?;

    let system_id = resolved.id;
    let system_name = resolved.name;

    // Get system info from ESI
    let sys_info = esi
        .get_system_info(system_id)
        .with_context(|| "Failed to get system info")?;

    // Get kill data from zKillboard
    let kills = zkill
        .get_system_kills(system_id, 24)
        .map(|k| k.kill_count.unwrap_or(0) as u32)
        .unwrap_or(0);

    let kills_7d = zkill
        .get_system_kills(system_id, 168)
        .map(|k| k.kill_count.unwrap_or(0) as u32)
        .unwrap_or(0);

    println!("System:    {}", system_name);
    println!("Security:  {:.1}", sys_info.security);

    if let Some(region) = sys_info.region {
        println!("Region ID: {}", region);
    }

    println!();
    println!("Kills 24h:   {}  {}", kills, danger_label(kills));
    println!("Kills 7d:   {}", kills_7d);

    // Check for notorious systems
    let notorious = ["Jita", "Perimeter", "Urlen", "Ahbazon", "Oulley"];
    if notorious
        .iter()
        .any(|s| s.eq_ignore_ascii_case(&system_name))
    {
        println!();
        println!("⚠ NOTORIOUS: Known high-trap system. Caution at gates.");
    }

    Ok(())
}
