use std::error::Error;

mod auth;
mod config;
mod env;
mod tui;

use auth::auth::authenticate;
use config::config::{Cfg, CfgManager};
use env::env::load_env;

use tui::tui::welcome_dialog;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let env = load_env();

    let config_manager = CfgManager::new_cfg_manager();
    let config = &config_manager.get_config(1)?;
    let config_exists = &config_manager.verify_config_exists();

    if config.access_token == "".to_string() && *config_exists == true {
        println!("not authenicated");
        let auth = authenticate(env.client_id, env.client_secret).await; // write the access_token (auth) to the config file

        let _ = &config_manager.write_config(&Cfg {
            access_token: auth.to_string(),
            ..config.clone()
        });
    } else {
        println!("You are already authenticated... starting TUI!");
        println!("access_token: {:?}", config.access_token);
    }

    // let get_config = &config_manager.get_config(1);
    // println!("get_config: {:?}", get_config);

    println!("welcome dialog: {:?}", welcome_dialog());

    Ok(())
}

// NOTE: struct update syntax
//
// #[derive(Clone, Debug)]
// struct Doc {
//     id: usize,
//     a: String,
// }
//
// fn main() {
//     let d = Doc {id: 1, a: "foo".to_string()};
//     let d2 = Doc {
//         id: 2,
//         ..d.clone()
//     };
//
//     println!("{d:?} {d2:?}");
// }
