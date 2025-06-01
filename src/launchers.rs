use std::process::{Command, Stdio};

pub fn launch_openrgb(profile: &str) {
    /* Launches openrgb and waits for it to exit
    *  so no zombie process is created
    */
    let _ = Command::new("openrgb")
        .args(["--profile", profile])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start OpenRGB")
        .wait()
        .expect("Failed to wait on OpenRGB");
}
