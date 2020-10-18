use app_dirs::AppInfo;


pub const APP_INFO: &AppInfo = &AppInfo {
    name: "glab-cli",
    author: "Surya Elavazhagan"
};

// Author
pub const AUTHOR: &str = "Surya Elavazhagan <surya.elash98@protonmail.com>";

// Default gitlab host
pub const DEFAULT_HOST: &str = "https://gitlab.com";

/// The version of the library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The name of the configuration file.
pub const CONFIG_RC_FILE_NAME: &str = ".glabclirc";