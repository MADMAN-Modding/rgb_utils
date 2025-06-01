use std::{
    env,
    error::Error,
    thread,
    time::Duration,
};

use colored::Colorize;
use rgb_utils::{
    config::{get_mouse_id, get_profile, set_mouse_id, set_profile},
    constants, input,
    launchers::launch_openrgb, usb_handler::check_usbs,
};

use tokio::task;
use udev::MonitorBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    constants::setup();

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let result: Result<String, String> = match args[1].as_str() {
            "-c" | "--config" => config(),
            "-l" | "--listen" => listen().await,
            "-d" | "--daemon" => daemon().await,
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
        println!(
            "{}",
            "Not enough arguments found.\nUsage: <option (e.g.) -c, -l, -d>"
                .red()
                .bold()
        );
    }

    Ok(())
}

/// Configure the mouse id or profile to be loaded for openrgb
///
/// # Returns
/// * `Ok(String)` - Blank Message
/// * `Err(String)` - The Error the function had
fn config() -> Result<String, String> {
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

async fn listen() -> Result<String, String> {
    // Spawn a task to listen for USB events
    task::spawn_blocking(move || {
        // Create a monitor for USB devices
        let monitor = MonitorBuilder::new()
            .unwrap()
            .match_subsystem("usb")
            .unwrap()
            .listen()
            .unwrap();

        // Tracks how many times the id has been seen
        let mut launch: u8 = 0;

        let mut eventing = false;

        // Loop to listen for events
        loop {
            // 6 is the amount of events given off by the events
            if launch == 6 && !eventing {
                launch_openrgb(&get_profile());
                launch = 0;
            }

            // Poll for events
            let event = match monitor.iter().next() {
                Some(event) => {
                    eventing = true;
                    event
                }
                None => {
                    eventing = false;
                    thread::sleep(Duration::from_millis(10));
                    continue;
                }
            };

            for prop in event.properties() {
                let value = prop.value().to_str().unwrap();

                if value == get_mouse_id() {
                    launch += 1;
                }
            }
        }
    });

    // Keep the main function alive
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

async fn daemon() -> Result<String, String> {
    check_usbs();

    let _ = listen().await;

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}