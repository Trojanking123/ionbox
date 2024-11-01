use semver::Version;

pub const DB_FILE: &str = "ionbox.db";
pub const CONFIG_PATH: &str = "ionbox.json";
pub const LOG_PATH: &str = "ionbox.log";
pub const LOCAL_ADDR: &str = "127.0.0.1:50911";
pub const DEFAULT_DATA_VERSION: Version = Version::new(0, 0, 1);
