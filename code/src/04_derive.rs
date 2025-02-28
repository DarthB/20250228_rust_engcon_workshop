use serde::{Deserialize, Serialize};
use std::fs;

// this time we derive the implementation of fn default():
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct DistillationColumn {
    trays: i32,
    feed_place: i32,
    reflux_ratio: f32,

    // often macros add more customizations with attributes:
    #[serde(rename = "d2f")]
    distiliate_to_feed_ratio: f32,
}

fn export_json() -> Result<(), Box<dyn std::error::Error>> {
    let output_data = DistillationColumn::default();

    let json = serde_json::to_string_pretty(&output_data)?;
    let filename = "output.json";
    println!("JSON written to {}:\n{}", filename, json);
    fs::write(filename, json)?;
    Ok(())
}

fn main() {
    export_json().unwrap();
}
