use crate::{config::get_profile, launchers::launch_openrgb};
use std::{process::Command, sync::Mutex};

use crate::config::get_mouse_id;

static DEVICES: Mutex<Option<Vec<String>>> = Mutex::new(Some(vec![]));

pub fn check_usbs() {
    println!("Checking...");

    let mut devices_lock = DEVICES.lock().unwrap();

    // Kill any existing openrgb process
    let _ = Command::new("pkill")
        .arg("openrgb")
        .status()
        .expect("Failed to execute pkill");

    let new_devices = get_device_ids();

    if new_devices.contains(&get_mouse_id()) {
        launch_openrgb(&get_profile());
    }

    // Always update the devices
    *devices_lock = Some(new_devices);
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