use crate::{input, json_handler::{read_config_json, write_config}};

/// Set the profile loaded by the program
pub fn set_profile() -> Result<(), String> {
    let profile = input!("Enter Profile Name (e.g., Gaming): ");
    if profile.trim().is_empty() {
        return Err("Profile name cannot be empty.".to_string());
    }

    write_config("profile", profile);
    Ok(())
}

pub fn get_profile() -> String {
    read_config_json("profile")
}

/// Set the id of the mouse
pub fn set_mouse_id() -> Result<(), String> {
    let id = input!("Enter Mouse ID as Hexadecimal (e.g., 42a3): ");
    if id.trim().is_empty() {
        return Err("Mouse ID cannot be empty.".to_string());
    }

    write_config("mouseID", id);
    Ok(())
}

pub fn get_mouse_id() -> String {
    read_config_json("mouseID")
}