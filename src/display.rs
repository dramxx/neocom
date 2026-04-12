pub fn danger_label(kills: u32) -> &'static str {
    if kills == 0 {
        "clear"
    } else if kills <= 4 {
        "moderate"
    } else if kills >= 16 {
        "very dangerous"
    } else {
        "dangerous"
    }
}

pub fn threat_label(score: u8) -> &'static str {
    match score {
        0 => "probably fine",
        1 => "caution",
        2 => "dangerous",
        3 => "THREAT: HIGH",
        _ => "no data",
    }
}

pub fn isk_format_sell(amount: f64) -> String {
    if amount >= 1_000_000.0 {
        let m = (amount / 1_000_000.0 * 10.0).round() / 10.0;
        format!("{:.1}M", m)
    } else if amount >= 1_000.0 {
        let k = (amount / 1_000.0).round();
        format!("{}K", k as i64)
    } else {
        format!("{}", amount as i64)
    }
}

pub fn isk_format(amount: f64) -> String {
    isk_format_sell(amount) + " ISK"
}

pub fn show_status() -> anyhow::Result<()> {
    let client = crate::esi::EsiClient::new();
    let status = client.get_status()?;
    println!("Tranquility ONLINE");
    println!("Players: {}", status.players);
    println!("Version: {}", status.version);
    Ok(())
}
