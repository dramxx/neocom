use crate::cli::IntelArgs;
use crate::display::threat_label;
use anyhow::{Context, Result};

pub fn run(args: IntelArgs) -> Result<()> {
    let esi = crate::esi::EsiClient::new();
    let zkill = crate::zkill::ZkillClient::new();

    let resolved = esi
        .resolve_names(&[&args.pilot])?
        .pop()
        .with_context(|| "Unknown pilot")?;

    let char_id = resolved.id;

    let info = zkill
        .get_character_info(char_id)
        .with_context(|| "No data for pilot")?;

    let details = info.info.as_ref();
    let kills = info.kills.unwrap_or(0);
    let losses = info.losses.unwrap_or(0);

    let kd = if losses > 0 {
        kills as f64 / losses as f64
    } else {
        kills as f64
    };

    let threat = if kills >= 20 && kd >= 2.0 {
        3
    } else if kills >= 10 && kd >= 1.0 {
        2
    } else if kills >= 5 {
        1
    } else {
        0
    };

    println!("Pilot:     {}", args.pilot);
    if let Some(d) = details {
        if let Some(corp) = d.corp_name.as_ref() {
            println!("Corp:      {}", corp);
        }
        if let Some(alliance) = d.alliance_name.as_ref() {
            println!("Alliance:  {}", alliance);
        }
    }

    println!();
    println!("All-time kills:    {}", kills);
    println!("All-time losses:  {}", losses);
    println!("K/D ratio:        {:.1}", kd);
    println!();
    println!(
        "{} {}",
        threat_label(threat),
        if threat >= 2 {
            "- experienced hunter"
        } else {
            ""
        }
    );

    Ok(())
}
