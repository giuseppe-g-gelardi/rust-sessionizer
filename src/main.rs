use std::error::Error;

mod auth;
mod config;
mod env;

use auth::auth::authenticate;
use config::config::CfgManager;
use env::env::load_env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let env = load_env();
    println!("env: {:?}", env);

    let config_manager = CfgManager::new_cfg_manager();
    let config = &config_manager.get_init_config(1);
    println!("config: {:?}", config);

    let auth = authenticate(env.client_id, env.client_secret).await; // write the access_token (auth) to the config file
    println!("auth: {:?}", auth);

    Ok(())
}


// NOTE: go reference
/*
// instantiate the config manager that gets passed around the app
cm := c.NewCfgManager()
// currently returns a pointer to a UserConfig struct and an error
conf, err := cm.GetConfig(1)
if err != nil {
    log.Errorf("Error: %v", err)
}

// check if the access token is empty
if conf.AccessToken == "" {
    // if the access token is empty, start the auth flow
    err := auth.Authenticate(conf, cm) // this returns a boolean (isAuth true/false) and an error. should probably remove the bool
    if err != nil {
        log.Errorf("Error: %v", err)
    }
} else {
    fmt.Println("You are already authenticated!")
}
// if the access token is not empty, start the cli
cli.InitCli(conf, cm)
*/

// NOTE: prev main lol {
//
// let config_manager = CfgManager::new_cfg_manager();
// let config = &config_manager.get_init_config(1);
//
// println!("config: {:?}", config);
//
// let config_exists = &config_manager.verify_config_exists();
// println!("config_exists: {:?}", config_exists);
//
// let cfg = Cfg {
//     access_token: "banana".to_string(),
//     editor: "emacs, ewwwww".to_string(),
//     alias: Some("sweet alias".to_string()),
//     tmux: true,
// };
//
// let _ = &config_manager.write_config(&cfg); // this is weird to me
// println!("with_cfg: {:?}", &config);
//
// let upated_cfg = &config_manager.get_init_config(4);
// println!("updated_cfg: {:?}", &upated_cfg);
//
// // // TODO: cli.init()
//
// Ok(())
// // ********************************************************************** //
// // ********************************************************************** //
// // ********************************************************************** //
// // ********************************************************************** //
// }
