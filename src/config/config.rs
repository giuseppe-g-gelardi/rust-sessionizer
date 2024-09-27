use serde::{Deserialize, Serialize};
use serde_json::json;

use std::fs;
use std::io::Error;

use dirs;
use std::env;
use std::path::PathBuf;

/*
 * NOTE:
 * consider putting the UserInfo struct in here
 *
 * ex: {
 * pub struct UserInfo {
 *    pub username: String,
 *    pub email: String,
 * }
 *
 * pub struct Cfg {
 *   pub user_info: UserInfo,
 *   pub access_token: String,
 *   pub editor: String,
 *   pub alias: Option<String>,
 *   pub tmux: bool,
 * }
 * 
 */

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Cfg {
    pub username: String,
    pub email: String,
    pub access_token: String,
    pub editor: String,
    pub alias: Option<String>,
    pub tmux: bool,
}

#[derive(Debug, Clone)]
pub struct CfgManager {
    pub config_file_name: String, // config_file_name: Option<PathBuf>,
    pub config: Cfg,
}

impl CfgManager {
    pub fn new_cfg_manager() -> Self {
        Self {
            config_file_name: "cfg.json".to_string(), // self.get_config_file_location(),
            config: Cfg {
                username: "".to_string(),
                email: "".to_string(),
                access_token: "".to_string(),
                editor: "vscode".to_string(),
                alias: None,
                tmux: false,
            },
        }
    }

    pub fn generate_default_config_file(&self) -> Result<Cfg, Error> {
        let config_exists = self.verify_config_exists();
        println!("config_exists: {:?}", config_exists);
        match config_exists {
            // true => self.get_config(0),
            true => self.get_config(0),
            false => self.write_config(&self.config),
        }
    }

    pub fn get_config(&self, recursive_depth: u8) -> Result<Cfg, Error> {
        if recursive_depth > 5 {
            return Err(Error::new(std::io::ErrorKind::NotFound, "File not found"));
        }

        match self.read_config_file() {
            Ok(config) => Ok(config),
            Err(_) => {
                self.generate_default_config_file()?;
                self.get_config(0)
            }
        }
    }

    pub fn write_config(&self, config: &Cfg) -> Result<Cfg, Error> {
        match self.save_config_to_file(&config) {
            Err(_) => {
                return Err(Error::new(
                    std::io::ErrorKind::NotFound,
                    "Error writing to or saving file",
                ));
            }
            _ => self.get_config(0),
        }
    }

    fn save_config_to_file(&self, config: &Cfg) -> Result<(), Error> {
        let json_cfg = json!(&config);
        let json_str = serde_json::to_string_pretty(&json_cfg).unwrap();

        match std::fs::write(&self.config_file_name, json_str) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::new(
                std::io::ErrorKind::NotFound,
                "Error writing to or saving file:, {}".to_string() + &e.to_string(),
            )),
        }
    }

    fn read_config_file(&self) -> Result<Cfg, Error> {
        let content = fs::read_to_string(&self.config_file_name)?;
        let config = serde_json::from_str::<Cfg>(&content)?;
        Ok(config)
    }

    pub fn verify_config_exists(&self) -> bool {
        PathBuf::from(self.config_file_name.to_string()).exists()
    }

    fn _get_config_file_location(self) -> Option<PathBuf> {
        // - for linux and mac it is ~/.config/cfg.json -- subject to change
        // - for windows it is %APPDATA%\local\cfg.json ???
        let home_dir = dirs::home_dir()?; // Get user's home directory
        let config_dir = match std::env::consts::OS {
            "windows" => {
                let appdata = env::var("APPDATA").ok()?;
                PathBuf::from(appdata).join("local") // eww...
            }
            "linux" | "macos" => home_dir.join(".config"),
            _ => {
                eprintln!("Unsupported operating system");
                return None;
            }
        };

        Some(config_dir.join("cfg.json"))
    }
}

// ************************************************************************** //
// ************************************************************************** //
// ************************************************************************** //
// ************************************************************************** //
// ************************************************************************** //
