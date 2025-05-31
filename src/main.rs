use std::{
    env,
    error::Error,
    thread,
    time::Duration,
};

use colored::Colorize;
use rgb_utils::{
    config::{set_mouse_id, set_profile},
    constants, input, usb_handler::check_usbs,
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
            "Not enough arguments found.\nUsage: <option> <arg1> [arg2]"
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

        println!("Listening for USB events...");

        // Tracks if something has happened with the usbs
        let mut usb_event: bool = false;

        // What value to check for with the `usb_event` bool
        let mut listen_state = false;
        // Loop to listen for events

        loop {
            // If an event has happened
            if usb_event {
                // If the events are over
                if usb_event == listen_state {
                    thread::sleep(Duration::from_millis(500));
                    check_usbs();
                    listen_state = false;
                    usb_event = false;
                }
            }

            // Poll for events
            let event = match monitor.iter().next() {
                Some(event) => {
                    usb_event = true;
                    event
                }
                None => {
                    listen_state = true;
                    thread::sleep(Duration::from_millis(10));
                    continue;
                }
            };

            println!("{}", event.action().unwrap().to_string_lossy());
        }
    });

    // Keep the main function alive
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

