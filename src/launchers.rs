use std::process::{Command, Stdio};

pub fn launch_openrgb(profile: &str) {
    let _ = Command::new("openrgb")
        .args(["--profile", profile])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to load OpenRGB profile");
}
