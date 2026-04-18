use crate::cli::TravelArgs;
use crate::display::danger_label;
use anyhow::{Context, Result};

pub fn run(args: TravelArgs) -> Result<()> {
    let esi = crate::esi::EsiClient::new();
    let zkill = crate::zkill::ZkillClient::new();

    let route_flag = match args.route {
        crate::cli::RouteFlag::Shortest => "shortest",
        crate::cli::RouteFlag::Safest => "secure",
        crate::cli::RouteFlag::Secure => "secure",
        crate::cli::RouteFlag::Insecure => "insecure",
    };

    let route = esi
        .get_route(&args.origin, &args.destination, route_flag)
        .with_context(|| "Failed to get route")?;

    let mut hot_count = 0u32;

    println!(
        "Route: {} -> {}  ({} jumps)",
        args.origin,
        args.destination,
        route.len() - 1
    );
    println!(
        "Security: {}",
        route
            .iter()
            .map(|h| if h.security >= 0.5 {
                "HS"
            } else if h.security > 0.0 {
                "LS"
            } else {
                "NS"
            })
            .collect::<Vec<_>>()
            .join(" -> ")
    );
    println!();

    for hop in &route {
        // Get stargate IDs for this system from ESI
        let gate_ids = esi.get_stargate_ids(hop.system_id).unwrap_or_default();
        
        // Get gate-specific kills - only count kills AT the gate (not neighbor gates)
        // This matches eve-gatecheck behavior
        let kills: u32 = if gate_ids.is_empty() {
            0
        } else {
            match zkill.get_gate_kills(hop.system_id, &gate_ids, args.hours) {
                Ok(gate_kills) => {
                    // Count ALL kills at gates in this system (matches "at gates" in eve-gatecheck)
                    gate_kills.iter().map(|g| g.kill_count as u32).sum()
                },
                Err(_) => 0
            }
        };

        if kills >= 5 {
            hot_count += 1;
        }

        let bar_width = 10;
        let filled = ((kills as f64 / 20.0) * bar_width as f64).min(bar_width as f64) as usize;
        let bar = format!("{}{}", "█".repeat(filled), "░".repeat(bar_width - filled));

        println!(
            "{:<12} [{:.1}]  {:<10}  {}/{}h  {}",
            hop.name,
            hop.security,
            bar,
            kills,
            args.hours,
            danger_label(kills)
        );
    }

    println!();
    if hot_count > 0 {
        println!("Danger: {} hot systems.", hot_count);
    }
    Ok(())
}
