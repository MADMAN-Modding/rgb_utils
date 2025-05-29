use std::{env, process::{Command, Stdio}};

use colored::Colorize;
use rgb_utils::{config::{get_profile, set_mouse_id, set_profile}, constants, input};

fn main() {
    constants::setup();

    let args : Vec<String> = env::args().collect();

    if args.len() > 1 {
        let result: Result<String, String> = match args[1].as_str() {
            "-c" | "--config"  => config(),
            _ => Err("Option Not Found".to_string()),
        };

        if result.is_ok() {
            println!("{}", result.unwrap().green().bold())
        } else {
            eprintln!(
                "Failed to execute {}\nError: {}",
                args[1].red().bold(),
                result.err().unwrap().red().bold()
            );

            std::process::exit(-1);
        }
    } else {
        println!("{}", "Not enough arguments found.\nUsage: <option> <arg1> [arg2]".red().bold());
    }

    for id in get_device_ids() {
        if id == constants::MOUSE_PRODUCT_ID.to_string() {
            launch_openrgb(get_profile().as_str());

            break;
        }
    }

}

/// Return a `Vec<String>` of each usb product-id as hexadecimal
fn get_device_ids() -> Vec<String> {
    let mut devices: Vec<String> = Vec::new();

    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        devices.push(format!("{:04x}", device_desc.product_id()));
    }

    devices
}

fn launch_openrgb(profile: &str) {
    let _ = Command::new("openrgb")
    .args(["--profile", profile])
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .spawn()
    .expect("Failed to load OpenRGB profile");
}

/// Configure the mouse id or profile to be loaded for openrgb
/// 
/// # Returns
/// * `Ok(String)` - Blank Message
/// * `Err(String)` - The Error the function had
fn config() -> Result<String, String>{
    let result = match input!("MouseID [1]\nProfile [2]").as_str() {
        "1" => set_mouse_id(),
        "2" => set_profile(),
        _ => return Err("Invalid Option".to_string()),   
    };

    if let Err(e) = result {
        return Err(e);
    }

    Ok("".to_string())
}