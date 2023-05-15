mod args;
mod meteorology;
mod waybar_json;

use crate::args::Args;
use crate::meteorology::*;
use clap::Parser;
use quick_xml::de::from_str;
use std::fs::File;
use std::io::BufReader;
use waybar_json::Waybar;

async fn get_data(dry_run: Option<String>) -> anyhow::Result<Meteorology> {
    let meteorology: Meteorology = if let Some(data_location) = dry_run {
        let f = File::open(data_location)?;
        let reader = BufReader::new(f);

        quick_xml::de::from_reader(reader)?
    } else {
        const API_URL: &str = "https://dom.org.cy/AWS/OpenData/CyDoM.xml";

        let xml = reqwest::get(API_URL).await?.text().await?;
        from_str(&xml)?
    };

    Ok(meteorology)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    //    println!("args: {args:?}");

    let meteorology = get_data(args.dry_run).await?;

    if args.stations {
        let station_names = meteorology.get_stations_name();
        for (idx, name) in station_names.iter().enumerate() {
            println!("{}. {name}", idx + 1);
        }
    }

    let mut icon = "";
    let mut temperature = 0.0;
    if let Some(station) = args.station {
        let observations = meteorology.get_station_options(station);
        if let Some(observations) = observations {
            icon = waybar_json::get_icon(observations);
            const TEMPERATURE: &str = "Air Temperature (1.2m)";
            temperature = observations.get_value(TEMPERATURE).unwrap();
        }
    }

    let mut waybar = Waybar::default();

    waybar = waybar.set_text(format!("{temperature:.0}\u{00b0} {icon}"));

    let s = serde_json::to_string(&waybar).unwrap();
    println!("{s}");

    Ok(())
}
