use crate::cli::PriceArgs;
use crate::display::isk_format;
use anyhow::{Context, Result};
use comfy_table::{Attribute, Cell, Table};

pub fn run(args: PriceArgs) -> Result<()> {
    let esi = crate::esi::EsiClient::new();
    let region_id: i64 = args.region.parse().context("Invalid region ID")?;

    if let Some(file_path) = &args.file {
        return run_file(esi, file_path, region_id, args.buy, args.sell);
    }

    let item = args.item.as_ref().context("Item name required")?;
    let quantity = args.quantity.unwrap_or(1);

    let resolved = esi.resolve_names(&[item])?.pop().context("Unknown item")?;

    let orders = esi
        .get_market_orders(resolved.id, region_id)
        .with_context(|| "Failed to fetch orders")?;

    let mut best_sell = f64::MAX;
    let mut best_buy = f64::MIN;

    for order in &orders {
        if order.is_buy {
            best_buy = best_buy.max(order.price);
        } else {
            best_sell = best_sell.min(order.price);
        }
    }

    println!("{}  x{}", item, quantity);

    if !args.buy && best_sell < f64::MAX {
        let total = best_sell * quantity as f64;
        println!(
            "  Sell:  {}  →  {} total",
            isk_format(best_sell),
            isk_format(total)
        );
    }

    if !args.sell && best_buy > 0.0 {
        let total = best_buy * quantity as f64;
        println!(
            "  Buy:   {}  →  {} total",
            isk_format(best_buy),
            isk_format(total)
        );
    }

    Ok(())
}

fn run_file(
    esi: crate::esi::EsiClient,
    file_path: &str,
    region_id: i64,
    _buy_only: bool,
    _sell_only: bool,
) -> Result<()> {
    let content =
        std::fs::read_to_string(file_path).with_context(|| format!("Cannot read {}", file_path))?;

    let mut items = Vec::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 2 {
            let name = parts[0].to_string();
            let qty: u32 = parts[1].trim().parse().unwrap_or(1);
            items.push((name, qty));
        }
    }

    let mut table = Table::new();
    table.set_header(vec![
        Cell::new("Item").add_attribute(Attribute::Bold),
        Cell::new("Qty"),
        Cell::new("Sell/unit").add_attribute(Attribute::Bold),
        Cell::new("Sell total").add_attribute(Attribute::Bold),
        Cell::new("Buy/unit").add_attribute(Attribute::Bold),
        Cell::new("Buy total").add_attribute(Attribute::Bold),
    ]);

    let mut total_sell = 0.0;
    let mut total_buy = 0.0;

    for (name, qty) in &items {
        let resolved = match esi.resolve_names(&[name])?.pop() {
            Some(r) => r,
            None => {
                eprintln!("Unknown item: {}", name);
                continue;
            }
        };

        let orders = esi
            .get_market_orders(resolved.id, region_id)
            .with_context(|| format!("Failed to fetch orders for {}", name))?;

        let mut best_sell = f64::MAX;
        let mut best_buy = f64::MIN;

        for order in orders {
            if order.is_buy {
                best_buy = best_buy.max(order.price);
            } else {
                best_sell = best_sell.min(order.price);
            }
        }

        let sell_per = if best_sell < f64::MAX { best_sell } else { 0.0 };
        let buy_per = if best_buy > 0.0 { best_buy } else { 0.0 };

        let sell_total = sell_per * *qty as f64;
        let buy_total = buy_per * *qty as f64;

        total_sell += sell_total;
        total_buy += buy_total;

        table.add_row(vec![
            Cell::new(name),
            Cell::new(qty.to_string()),
            Cell::new(isk_format(sell_per)),
            Cell::new(isk_format(sell_total)),
            Cell::new(isk_format(buy_per)),
            Cell::new(isk_format(buy_total)),
        ]);
    }

    table.add_row(vec![
        Cell::new("").add_attribute(Attribute::Bold),
        Cell::new("").add_attribute(Attribute::Bold),
        Cell::new("").add_attribute(Attribute::Bold),
        Cell::new(isk_format(total_sell)).add_attribute(Attribute::Bold),
        Cell::new("").add_attribute(Attribute::Bold),
        Cell::new(isk_format(total_buy)).add_attribute(Attribute::Bold),
    ]);

    println!("{}", table);
    Ok(())
}
