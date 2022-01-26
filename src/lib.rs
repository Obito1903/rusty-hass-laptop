use convert_case::{Case, Casing};
use directories::{self};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{BufReader, Write},
    ops::Not,
    path::Path,
};
use sys_info;

pub mod hass;
pub mod sensors;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct Config {
    #[serde(skip)]
    pub config_file: Option<String>,
    pub auth_token: Option<String>,
    pub webhook_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_secret: Option<String>,
    pub hass_address: Option<String>,
    pub device_id: String,
    pub app_id: String,
    pub app_name: String,
    pub app_version: String,
    pub device_name: String,
    pub manufacturer: String,
    pub model: String,
    pub os_name: String,
    pub os_version: String,
    pub support_encryption: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            config_file: None,
            auth_token: None,
            webhook_id: None,
            webhook_secret: None,
            hass_address: None,
            device_id: format!(
                "{}_{}",
                sys_info::hostname().unwrap(),
                sys_info::os_type().unwrap()
            ),
            app_id: env!("CARGO_PKG_NAME").parse().unwrap(),
            app_name: env!("CARGO_PKG_NAME")
                .parse()
                .unwrap_or(String::from(""))
                .to_case(Case::Title),
            app_version: env!("CARGO_PKG_VERSION").parse().unwrap(),
            device_name: sys_info::hostname().unwrap(),
            manufacturer: String::from("Unknown"),
            model: String::from("Unknown"),
            os_name: sys_info::os_type().unwrap(),
            os_version: sys_info::os_release().unwrap(),
            support_encryption: false,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let project_dir = directories::ProjectDirs::from("fr", "obito1903", env!("CARGO_PKG_NAME"));
        if project_dir.is_none() {
            Config::default()
        } else {
            let config_path = project_dir
                .as_ref()
                .unwrap()
                .config_dir()
                .join("config.json");
            if config_path.exists() {
                Config::load_from_file(config_path.to_str().unwrap().parse().unwrap()).unwrap()
            } else {
                let config = Config::default();
                config.save_to_file().unwrap();
                config
            }
        }
    }

    pub fn load_from_file(path: String) -> Result<Self, &'static str> {
        if Path::new(&path).exists() {
            let file = File::open(&path).unwrap();
            let reader = BufReader::new(file);
            let mut config: Config = serde_json::from_reader(reader).unwrap();
            config.config_file = Some(path);
            Ok(config)
        } else {
            Err("Config file not found")
        }
    }

    pub fn save_to_file(&self) -> Result<(), &'static str> {
        let config_path = directories::ProjectDirs::from("fr", "obito1903", env!("CARGO_PKG_NAME"))
            .unwrap()
            .config_dir()
            .join("config.json");
        if config_path.parent().unwrap().exists().not() {
            fs::create_dir_all(&config_path.parent().unwrap()).unwrap();
        }
        let mut file = File::create(&config_path).unwrap();
        file.write_all(serde_json::to_string_pretty(&self).unwrap().as_bytes())
            .unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    #[test]
    fn serialize_test() {
        let config = Config::new();
        println!("serialized = {}", serde_json::to_string(&config).unwrap());
    }
    #[test]
    fn deserialize_test() {
        let reader = String::from(
            r#"{"auth_token":null,"webhook_id":null,"hass_address":null,"device_id":"rodriguess","app_version":"0.1.0","device_name":"rodriguess","manufacturer":"Unknown","model":"Unknown","os_name":"Linux","os_version":"5.14.16-zen1-1-zen"}"#,
        );
        let config: Config = serde_json::from_reader(reader.as_bytes()).unwrap();
        println!("{:?}", config);
    }
    #[test]
    fn save_test() {
        let config = Config::new();
        config.save_to_file().unwrap();
    }
    #[test]
    fn read_test() {
        let config = Config::new();
        println!(
            "serialized = {}",
            serde_json::to_string_pretty(&config).unwrap()
        );
    }
}
