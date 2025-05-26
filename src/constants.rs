use directories::ProjectDirs;
use once_cell::sync::OnceCell;

static PROJ_DIRS: OnceCell<ProjectDirs> = OnceCell::new();
pub const MOUSE_PRODUCT_ID:&str = "0067";

pub fn setup() {
    PROJ_DIRS
        .set(
            ProjectDirs::from(
                "com",
                "MADMAN-Modding",
                "RGB Utils",
            )
            .expect("Failed to create ProjectDirs"),
        )
        .unwrap();

    let _ = std::fs::create_dir_all(
        PROJ_DIRS
            .get()
            .expect("Failed to make config dir")
            .config_dir(),
    );
}


pub fn get_config_dir() -> String {
    let proj_dir = PROJ_DIRS.get().expect("ProjectDirs in not initialized :(");

    proj_dir.config_dir();

    let config_dir = ProjectDirs::config_dir(&proj_dir).to_str().unwrap();

    return config_dir.to_string();
}

pub fn get_config_json_path() -> String {
    format!("{}/config.json", get_config_dir())
}