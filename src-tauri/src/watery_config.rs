use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};
use std::path::Path;
use std::sync::Arc;

use log::info;
use parking_lot::RwLock;
use semver::Version;
use tauri::Url;

use serde::{Deserialize, Serialize};

use crate::WateryResult;

const DEFAULT_DATA_VERSION: Version = Version::new(0, 0, 1);

#[derive(Debug, Deserialize, Serialize)]
pub struct WateryConfig {
    data_version: Version,
    pub proxy: Option<Url>,
}

impl Default for WateryConfig {
    fn default() -> Self {
        WateryConfig {
            data_version: DEFAULT_DATA_VERSION,
            proxy: None,
        }
    }
}

impl WateryConfig {
    pub fn get_ver(&self) -> &Version {
        &self.data_version
    }

    pub fn read_from_file<P: AsRef<Path>>(file_path: P) -> WateryResult<Self> {
        // 尝试打开文件
        let file = OpenOptions::new().read(true).open(&file_path);

        // 如果文件不存在，则创建文件并写入默认配置
        let config = match file {
            Ok(file) => {
                // 文件存在，读取并解析
                let reader = BufReader::new(file);
                let config: WateryConfig = serde_json::from_reader(reader)?;
                config
            }
            Err(_) => {
                // 文件不存在或打开失败，创建文件并写入默认配置
                info!("File not found, creating a new one with default configuration.");
                let default = WateryConfig::default();

                let mut new_file = File::create(&file_path)?;
                let json = serde_json::to_string_pretty(&default)?;
                new_file.write_all(json.as_bytes())?;
                default
            }
        };
        Ok(config)
    }

    pub fn dump_to_file<P: AsRef<Path>>(&self, file_path: P) -> WateryResult<()> {
        // 尝试打开文件
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .open(&file_path);

        // 如果文件不存在，则创建文件并写入默认配置
        match file {
            Ok(mut file) => {
                let json = serde_json::to_string_pretty(&self)?;
                file.write_all(json.as_bytes())?;
            }
            Err(_) => {
                info!("File not found, creating a new one with default configuration.");
                let mut new_file = File::create(&file_path)?;
                let json = serde_json::to_string_pretty(&self)?;
                new_file.write_all(json.as_bytes())?;
            }
        };
        Ok(())
    }
}

pub type WateryConfigWrap = Arc<RwLock<WateryConfig>>;
