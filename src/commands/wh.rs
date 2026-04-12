use crate::cli::{WhArgs, WhType};
use anyhow::Result;

pub fn run(args: WhArgs) -> Result<()> {
    let class = args.class.to_lowercase();
    let wh_type = args.wh_type.clone();

    match class.as_str() {
        "c1" => show_c1(&wh_type),
        "c2" => show_c2(&wh_type),
        "c3" => show_c3(&wh_type),
        "c4" => show_c4(&wh_type),
        "c5" => show_c5(&wh_type),
        "c6" => show_c6(&wh_type),
        _ => Err(anyhow::anyhow!(
            "Unknown wormhole class: {}. Use c1-c6.",
            class
        )),
    }
}

fn show_c1(_wh_type: &WhType) -> Result<()> {
    println!("Class 1 Wormhole Sites");
    println!("{}", "=".repeat(50));

    println!("COMBAT ANOMALIES");
    println!("  Unknown            ~150 DPS    easy");
    println!("  Forgotten         ~200 DPS    easy");

    println!();
    println!("RELIC / DATA SITES");
    println!("  Data Site         ~400 DPS");
    println!("  Relic Site       ~500 DPS");

    println!();
    println!("STATICS: → C2, → HS");
    Ok(())
}

fn show_c2(_wh_type: &WhType) -> Result<()> {
    println!("Class 2 Wormhole Sites");
    println!("{}", "=".repeat(50));

    println!("COMBAT ANOMALIES");
    println!("  Guardian          ~300 DPS");
    println!("  Core Scanner    ~350 DPS");

    println!();
    println!("RELIC / DATA SITES");
    println!("  Digital        ~500 DPS");
    println!("  Semantic      ~600 DPS");

    println!();
    println!("GAS SITES");
    println!("  K162           C50, C60");
    println!("  Temporal       C70");

    println!();
    println!("STATICS: → C1, → C3, → HS");
    Ok(())
}

fn show_c3(_wh_type: &WhType) -> Result<()> {
    println!("Class 3 Wormhole Sites");
    println!("{}", "=".repeat(50));

    println!("COMBAT ANOMALIES");
    println!("  Perimeter Checkpoint    ~650 DPS   easy");
    println!("  Perimeter Hangar      ~650 DPS   easy");
    println!("  Sleeper Data Sanct.   ~700 DPS   easy");

    println!();
    println!("RELIC / DATA SITES");
    println!("  Forgotten Frontier Recursive   1396 DPS  ⚠ hardest C3");
    println!("  Forgotten Frontier Quarantine  608 DPS");
    println!("  Unsecured Frontier         608 DPS");

    println!();
    println!("GAS SITES");
    println!("  Minor Perimeter Reservoir   C50+C60   ~25M/load");
    println!("  Sizable Perimeter Reservoir C50+C72   ~30M/load");

    println!();
    println!("STATICS: → Lowsec, → C1");
    Ok(())
}

fn show_c4(_wh_type: &WhType) -> Result<()> {
    println!("Class 4 Wormhole Sites");
    println!("{}", "=".repeat(50));

    println!("COMBAT ANOMALIES");
    println!("  Sanctum           ~800 DPS");
    println!("  Forgotten      ~1000 DPS");

    println!();
    println!("RELIC / DATA SITES");
    println!("  Front. Archive   ~900 DPS");
    println!("  Front. Vault  ~1100 DPS");

    println!();
    println!("STATICS: → C3, → C5, → HS");
    Ok(())
}

fn show_c5(_wh_type: &WhType) -> Result<()> {
    println!("Class 5 Wormhole Sites");
    println!("{}", "=".repeat(50));

    println!("COMBAT ANOMALIES");
    println!("  Stronghold       ~1200 DPS");
    println!("  Incursion       ~1500 DPS");

    println!();
    println!("RELIC / DATA SITES");
    println!("  Front. Archive   ~1500 DPS");
    println!("  Magnetometric   ~2000 DPS");

    println!();
    println!("STATICS: → C4, → C6, → LS");
    Ok(())
}

fn show_c6(_wh_type: &WhType) -> Result<()> {
    println!("Class 6 Wormhole Sites");
    println!("{}", "=".repeat(50));

    println!("COMBAT ANOMALIES");
    println!("  Tyrannos         ~2000 DPS");
    println!("  Titan's Forge    ~2500 DPS");

    println!();
    println!("RELIC / DATA SITES");
    println!("  Citadel         ~2500 DPS");
    println!("  Overseer        ~3000 DPS");

    println!();
    println!("STATICS: → C5, → NS");
    Ok(())
}
