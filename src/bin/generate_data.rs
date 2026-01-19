use csv::Writer;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Generating large dataset for capital allocation optimizer...");

    let num_investments = 1000;
    let alternatives_per_investment = 4;
    let total_alternatives = num_investments * alternatives_per_investment;

    let file = File::create("assets_large.csv")?;
    let mut writer = Writer::from_writer(file);

    writer.write_record([
        "Asset_ID",
        "Alternative_ID",
        "Cost_USD",
        "PoF_Post_Action",
        "CoF_Total_USD",
        "Safety_Risk_Level",
    ])?;

    let investment_types = vec![
        "IT_SYSTEM",
        "DATACENTER",
        "CLOUD_MIGRATION",
        "SOFTWARE_LICENSE",
        "BUILDING_RENOVATION",
        "FACILITY_EXPANSION",
        "OFFICE_UPGRADE",
        "WAREHOUSE",
        "MARKET_EXPANSION",
        "PRODUCT_LAUNCH",
        "RND_PROJECT",
        "MANUFACTURING_LINE",
        "ROAD_UPGRADE",
        "BRIDGE_REPAIR",
        "POWER_GRID",
        "WATER_SYSTEM",
        "TRAINING_PROGRAM",
        "CYBERSECURITY",
        "ERP_SYSTEM",
        "CRM_PLATFORM",
    ];

    let alternatives = [
        "Defer",
        "Pilot_Program",
        "Partial_Implementation",
        "Full_Implementation",
    ];

    let safety_levels = ["Negligible", "Low", "Medium", "High", "Critical"];

    let mut seed = 42u64;

    for inv_num in 0..num_investments {
        let investment_type = &investment_types[inv_num % investment_types.len()];
        let asset_id = format!("{}_{:04}", investment_type, inv_num + 1);

        seed = (seed * 1103515245 + 12345) & 0x7fffffff;
        let base_cof = 100000.0 + (seed % 5000000) as f64;

        seed = (seed * 1103515245 + 12345) & 0x7fffffff;
        let base_safety_idx = (seed % 5) as usize;

        for (alt_idx, alternative) in alternatives.iter().enumerate() {
            let cost = match alt_idx {
                0 => 0.0,
                1 => {
                    seed = (seed * 1103515245 + 12345) & 0x7fffffff;
                    5000.0 + (seed % 15000) as f64
                }
                2 => {
                    seed = (seed * 1103515245 + 12345) & 0x7fffffff;
                    20000.0 + (seed % 80000) as f64
                }
                3 => {
                    seed = (seed * 1103515245 + 12345) & 0x7fffffff;
                    100000.0 + (seed % 400000) as f64
                }
                _ => 0.0,
            };

            let pof = match alt_idx {
                0 => {
                    seed = (seed * 1103515245 + 12345) & 0x7fffffff;
                    0.15 + (seed % 30) as f64 / 100.0
                }
                1 => {
                    seed = (seed * 1103515245 + 12345) & 0x7fffffff;
                    0.10 + (seed % 20) as f64 / 100.0
                }
                2 => {
                    seed = (seed * 1103515245 + 12345) & 0x7fffffff;
                    0.04 + (seed % 12) as f64 / 100.0
                }
                3 => {
                    seed = (seed * 1103515245 + 12345) & 0x7fffffff;
                    0.01 + (seed % 5) as f64 / 100.0
                }
                _ => 0.5,
            };

            let safety_idx = if alt_idx == 0 {
                base_safety_idx
            } else {
                (base_safety_idx as i32 - alt_idx as i32).max(0) as usize
            };
            let safety_level = &safety_levels[safety_idx];

            writer.write_record([
                &asset_id,
                *alternative,
                &format!("{:.2}", cost),
                &format!("{:.4}", pof),
                &format!("{:.2}", base_cof),
                safety_level,
            ])?;
        }

        if (inv_num + 1) % 100 == 0 {
            println!(
                "Generated {} investments ({} alternatives)...",
                inv_num + 1,
                (inv_num + 1) * alternatives_per_investment
            );
        }
    }

    writer.flush()?;

    println!(
        "\n✓ Successfully generated {} alternatives for {} investments",
        total_alternatives, num_investments
    );
    println!("✓ File saved as: assets_large.csv");

    let metadata = std::fs::metadata("assets_large.csv")?;
    let size_mb = metadata.len() as f64 / (1024.0 * 1024.0);
    println!("✓ File size: {:.2} MB", size_mb);

    Ok(())
}
