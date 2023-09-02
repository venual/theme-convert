use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Color {
    key: String,
    label: String,
    hex: String,
    rgb: String,
    on: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Theme {
    colors: Vec<Color>,
    font_base: String,
    font_headings: String,
    text_color_light: String,
    text_color_dark: String,
    rounded_base: String,
    rounded_container: String,
    border_base: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: PathBuf,
    #[arg(short, long)]
    as_script: bool,
}

fn rgb_to_hex(rgb: &str) -> String {
    let values: Vec<u8> = rgb
        .split_whitespace()
        .filter_map(|v| v.parse().ok())
        .collect();

    if values.len() == 3 {
        format!("#{:02X}{:02X}{:02X}", values[0], values[1], values[2])
    } else {
        String::from("#000000")
    }
}

fn main() {
    let args = Args::parse();

    let raw_json = fs::read_to_string(args.file).expect("Failed to read the file");

    let raw_data: HashMap<String, String> = serde_json::from_str(&raw_json).unwrap();

    let color_keys = vec![
        "primary",
        "secondary",
        "tertiary",
        "success",
        "warning",
        "error",
        "surface",
    ];
    let mut colors = Vec::new();

    for key in color_keys.iter() {
        let rgb = raw_data[&format!("--color-{}-500", key)].clone();
        let hex = rgb_to_hex(&rgb);
        let on = raw_data[&format!("--on-{}", key)].clone();

        colors.push(Color {
            key: key.to_string(),
            label: key
                .to_string()
                .to_uppercase()
                .chars()
                .next()
                .unwrap()
                .to_string()
                + &key[1..],
            hex,
            rgb,
            on,
        });
    }

    let theme = Theme {
        colors,
        font_base: raw_data["--theme-font-family-base"].clone(),
        font_headings: raw_data["--theme-font-family-heading"].clone(),
        text_color_light: raw_data["--theme-font-color-base"].clone(),
        text_color_dark: raw_data["--theme-font-color-dark"].clone(),
        rounded_base: raw_data["--theme-rounded-base"].clone(),
        rounded_container: raw_data["--theme-rounded-container"].clone(),
        border_base: raw_data["--theme-border-base"].clone(),
    };

    let converted_json = serde_json::to_string(&theme).unwrap();

    if args.as_script {
        let script = format!(
            "localStorage.setItem('storeThemGenForm', '{}');",
            converted_json
        );
        println!("{}", script);
    } else {
        println!("{}", converted_json);
    }
}
