// use std::io::Error;

// use self::auth::auth::GithubAuthenticator;
use dotenv::dotenv;
// use std::env;
use std::error::Error;

// use self::config::config::{Cfg, CfgManager}; // config lol
mod auth;
mod util;
// mod config;

// use util::util::open_browser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    // match open_browser("http://google.com") {
    //     Ok(_) => println!("opened browser"),
    //     Err(e) => println!("error: {:?}", e),
    // }

    let client_id = std::env::var("client_id").expect("client_id must be set");
    let client_secret = std::env::var("client_secret").expect("client_secret must be set");
    println!("client_id: {:?}", client_id);
    println!("client_secret: {:?}", client_secret);

    let auth = auth::oauth_async::authenticate(client_id, client_secret).await;
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
