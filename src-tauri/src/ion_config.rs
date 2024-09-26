use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};
use std::path::Path;
use std::sync::Arc;

use log::info;
use parking_lot::RwLock;
use semver::Version;
use tauri::Url;

use serde::{Deserialize, Serialize};

use crate::IonResult;
use crate::DEFAULT_DATA_VERSION;

#[derive(Debug, Deserialize, Serialize)]
pub struct IonConfig {
    data_version: Version,
    pub proxy: Option<Url>,
}

impl Default for IonConfig {
    fn default() -> Self {
        IonConfig {
            data_version: DEFAULT_DATA_VERSION,
            proxy: None,
        }
    }
}

impl IonConfig {
    pub fn get_ver(&self) -> &Version {
        &self.data_version
    }

    pub fn read_from_file<P: AsRef<Path>>(file_path: P) -> IonResult<Self> {
        // 尝试打开文件
        let file = OpenOptions::new().read(true).open(&file_path);

        // 如果文件不存在，则创建文件并写入默认配置
        let config = match file {
            Ok(file) => {
                // 文件存在，读取并解析
                let reader = BufReader::new(file);
                let config: IonConfig = serde_json::from_reader(reader)?;
                config
            }
            Err(_) => {
                // 文件不存在或打开失败，创建文件并写入默认配置
                info!("File not found, creating a new one with default configuration.");
                let default = IonConfig::default();

                let mut new_file = File::create(&file_path)?;
                let json = serde_json::to_string_pretty(&default)?;
                new_file.write_all(json.as_bytes())?;
                default
            }
        };
        Ok(config)
    }

    pub fn dump_to_file<P: AsRef<Path>>(&self, file_path: P) -> IonResult<()> {
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

pub type IonConfigWrap = Arc<RwLock<IonConfig>>;
