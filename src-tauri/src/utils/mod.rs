use directories::ProjectDirs;
use std::fs;
use crate::ion_const::DB_FILE;


pub fn set_cfg_dir(file_name: &str) -> std::path::PathBuf {
    let cfg_dir = ProjectDirs::from("", "", "ionbox").unwrap();
    let dir = cfg_dir.config_dir();
    fs::create_dir_all(dir).unwrap();
    let dir = dir.join(file_name);
    dir
}


pub fn get_db_abs_path() -> String {
    let db_path = set_cfg_dir(DB_FILE);
    if !db_path.exists() {
        fs::File::create(&db_path).unwrap();
    }
    let db_path = db_path.to_str().unwrap_or_default().replace("\\", "/");
    let db_url = format!("sqlite:/{}", db_path);
    dbg!(&db_url);
    db_url
}