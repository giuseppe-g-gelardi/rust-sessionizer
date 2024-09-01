use serde::Deserialize;
use serde_json::json;

// use serde_json::from_str;
//
use std::fs;
use std::io::Error;

#[derive(Debug, Deserialize)]
pub struct Config { // should specify if this is json?
    pub access_token: String,
    pub editor: String,        // vscode, neovim, vim, emacs, etc.
    pub alias: Option<String>, // user defined alias
    pub tmux: bool,
}

struct ConfigManager {
    config_file_name: String,
    default_config: Config,
}

impl ConfigManager {
    pub fn new() -> Self {
        // should name this new_config_manager to not confuse with
        // Config::new()???
        let config_file_name = "config.json".to_string(); // getConfigFileLocation()
        let mut default_config = Config::new();

        default_config.access_token = "".to_string();
        default_config.editor = "vscode".to_string();
        default_config.alias = None;
        default_config.tmux = false;

        Self {
            config_file_name,
            default_config,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self {
            access_token: "".to_string(),
            editor: "".to_string(),
            alias: None,
            tmux: false,
        }
    }

    pub fn get_config(recursive_depth: u8) {}

    pub fn to_json(&self) -> serde_json::Value {
        json!({
            "access_token": self.access_token,
            "editor": self.editor,
            "alias": self.alias,
            "tmux": self.tmux,
        })
    }

    pub fn save_to_file(&self, file_name: &str) -> std::io::Result<()> {
        let json_cfg = self.to_json();
        let json_str = serde_json::to_string_pretty(&json_cfg).unwrap();
        std::fs::write(file_name, json_str)
    }

    pub fn create_config(&self, file_name: &String) -> Result<(), Error> {
        // let cfg = Config::new();
        self.save_to_file(file_name)?;
        Ok(())
    }

    pub fn update_config_file(&mut self, file_name: &String) {
        // let mut cfg = Config::new();

        self.access_token = "asdfasdf".to_string();
        self.editor = "VSCode".to_string();
        self.alias = None;
        self.tmux = true;

        self.save_to_file(&file_name).unwrap();
    }

    pub fn verify_access_token(&self, file_name: &String) -> Result<(), Error> {
        // this next line is pissing me off. I don't know what to do with it
        // keeps crashing the program instead of returning an error and moving
        // on with the rest of the program to circumvent the absence of the file
        // it is looking for.
        let text = fs::read_to_string(file_name);
        println!("text: {:#?}", &text);

        // not sure what to do in here

        // let cfg: Config = from_str(&text)?;
        // println!("access_token: {}", self.access_token);

        if self.access_token != "" {
            println!("access_token: {}", self.access_token);
        } else {
            println!("no access token")
        }

        Ok(())
    }
}
