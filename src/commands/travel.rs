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
        let gate_kills = if gate_ids.is_empty() {
            vec![]
        } else {
            match zkill.get_gate_kills(hop.system_id, &gate_ids, args.hours) {
                Ok(kills) => kills,
                Err(e) => {
                    eprintln!("Warning: could not fetch kills for {}: {}", hop.name, e);
                    vec![]
                }
            }
        };

        // Calculate total kills for this system
        let total_kills: u32 = gate_kills.iter().map(|g| g.kill_count as u32).sum();

        if total_kills >= 5 {
            hot_count += 1;
        }

        let bar_width = 10;
        let filled = ((total_kills as f64 / 20.0) * bar_width as f64).min(bar_width as f64) as usize;
        let bar = format!("{}{}", "█".repeat(filled), "░".repeat(bar_width - filled));

        // Show gate-specific details if there are kills
        let detail = if total_kills > 0 && !gate_kills.is_empty() {
            // Find gate with most kills
            let worst_gate = gate_kills
                .iter()
                .max_by_key(|g| g.kill_count);

            if let Some(g) = worst_gate {
                if g.kill_count > 0 {
                    // Get gate info to show destination system
                    if let Ok(gate_info) = esi.get_stargate_info(g.gate_id) {
                        // Get destination system name
                        let dest_name = if let Some(dest) = gate_info.destination {
                            // Try to resolve system_id to name, fall back to gate name
                            esi.resolve_system(dest.system_id)
                                .unwrap_or_else(|_| {
                                    gate_info.name
                                        .strip_prefix("Stargate (")
                                        .and_then(|s| s.strip_suffix(')'))
                                        .unwrap_or(&gate_info.name)
                                        .to_string()
                                })
                        } else {
                            // Parse gate name (e.g., "1NZV-7" from "Stargate (1NZV-7)")
                            gate_info.name
                                .strip_prefix("Stargate (")
                                .and_then(|s| s.strip_suffix(')'))
                                .unwrap_or(&gate_info.name)
                                .to_string()
                        };

                        let kill_text = if g.kill_count == 1 { "kill" } else { "kills" };
                        format!("{} {} at {} gate", g.kill_count, kill_text, dest_name)
                    } else {
                        let kill_text = if g.kill_count == 1 { "kill" } else { "kills" };
                        format!("{} {} at gate", g.kill_count, kill_text)
                    }
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        if detail.is_empty() {
            println!(
                "{:<12} [{:.2}]  {:<10}  {}/{}h  {}",
                hop.name,
                hop.security,
                bar,
                total_kills,
                args.hours,
                danger_label(total_kills)
            );
        } else {
            println!(
                "{:<12} [{:.2}]  {:<10}  {}/{}h  {}  {}",
                hop.name,
                hop.security,
                bar,
                total_kills,
                args.hours,
                danger_label(total_kills),
                detail
            );
        }
    }

    println!();
    if hot_count > 0 {
        println!("Danger: {} hot systems.", hot_count);
    }
    Ok(())
}
