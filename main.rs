use std::process::Command;
use std::io;

fn main() {
    println!("Enter value for max processor %:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("wow u fucked up the input somehow");
    let input = input.trim();

    let value: u8 = match input.parse() {
        Ok(num) if num >= 1 && num <= 100 => num,
        _ => {
            eprintln!("r u retarded, choose between 1-100");
            return;
        }
    };

    // change guid to your own, you can get it with "powercfg /L" in cmd
    // im too lazy to make it obtain the current active one so yeah
    let guid = "3ff9831b-6f80-4830-8178-736cd4229e7b";
  
    let value_str = value.to_string();

    let output = Command::new("powercfg")
        .args(&["/setdcvalueindex", guid, "SUB_PROCESSOR", "PROCTHROTTLEMAX", &value_str])
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("success!");
    } else {
        eprintln!(
            "error: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
