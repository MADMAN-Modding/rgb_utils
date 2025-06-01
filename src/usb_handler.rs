use crate::{config::get_profile, launchers::launch_openrgb};

use crate::config::get_mouse_id;

/// Launches openrgb if the device is connected
pub fn check_usbs() {
    if get_device_ids().contains(&get_mouse_id()) {
        launch_openrgb(&get_profile());
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