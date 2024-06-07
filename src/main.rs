use std::fs::File;
use std::io::{self, Read};
use std::process::Command;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    guid: String,
}

fn main() {
    let mut file = File::open("config.yml").expect("Failed to open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read config file");
    let config: Config = serde_yml::from_str(&contents).expect("Failed to parse config file");

    let guid = config.guid;

    println!("Enter CPU max % value:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();

    let value: u8 = match input.parse() {
        Ok(num) if num >= 1 && num <= 100 => num,
        _ => {
            eprintln!("Please choose a value between 1-100");
            return;
        }
    };

    let value_str = value.to_string();

    let ac_output = Command::new("powercfg")
        .args(&["/setacvalueindex", &guid, "SUB_PROCESSOR", "PROCTHROTTLEMAX", &value_str])
        .output()
        .expect("Failed to execute command for on battery");

    if ac_output.status.success() {
        println!("Successfully set max CPU usage (plugged in) to {}%", &value_str);
    } else {
        eprintln!(
            "Failed to set max CPU usage (plugged in) value: {}",
            String::from_utf8_lossy(&ac_output.stderr)
        );
    }

    let dc_output = Command::new("powercfg")
        .args(&["/setdcvalueindex", &guid, "SUB_PROCESSOR", "PROCTHROTTLEMAX", &value_str])
        .output()
        .expect("Failed to execute command");

    if dc_output.status.success() {
        println!("Successfully set max CPU usage (on battery) to {}%", &value_str);
    } else {
        eprintln!(
            "Failed to set max CPU usage (on battery) value: {}",
            String::from_utf8_lossy(&dc_output.stderr)
        );
    }
}
