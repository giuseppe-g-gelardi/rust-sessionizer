use dotenv::dotenv;

#[derive(Debug)]
pub struct Env {
    pub client_id: String,
    pub client_secret: String,
}
pub fn load_env() -> Env {
    dotenv().ok();

    Env {
        client_id: std::env::var("client_id").expect("client_id must be set"),
        client_secret: std::env::var("client_secret").expect("client_secret must be set"),
    }
}
