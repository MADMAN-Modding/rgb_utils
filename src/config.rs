use crate::{input, json_handler::write_config};

/// Set the profile loaded by the program
pub fn set_profile() {
    let profile = input!();

    write_config("profile", profile);
}

/// Set the id of the mouse
pub fn set_mouse_id() {
    let id = input!();

    write_config("mouseID", id);
}