use oauth2::basic::BasicClient;
use oauth2::{reqwest, DeviceAuthorizationUrl /*, StandardDeviceAuthorizationResponse*/};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use url::Url;

use webbrowser;

pub async fn authenticate(client_id: String, client_secret: String) {
    let github_client_id = ClientId::new(client_id);
    let github_client_secret = ClientSecret::new(client_secret);
    let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
        .expect("Invalid token endpoint URL");
    // let device_auth_url = "https://github.com/login/device/code".to_string();
    let device_auth_url =
        DeviceAuthorizationUrl::new("https://github.com/login/device/code".to_string())
            .expect("Invalid device authorization URL");

    // Set up the config for the Github OAuth2 process.
    let client = BasicClient::new(github_client_id)
        .set_client_secret(github_client_secret)
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_redirect_uri(
            RedirectUrl::new("http://localhost:9000/auth/callback".to_string())
                .expect("Invalid redirect URL"),
        )
        .set_device_authorization_url(device_auth_url);

    let http_client = reqwest::ClientBuilder::new()
        // Following redirects opens the client up to SSRF vulnerabilities.
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        // This example is requesting access to the user's public repos and email.
        .add_scope(Scope::new("public_repo".to_string()))
        .add_scope(Scope::new("user:email".to_string()))
        .url();

    println!("Open this URL in your browser:\n{authorize_url}\n");
    let _ = webbrowser::open(&authorize_url.to_string());

    // open_browser
    let (code, state) = {
        println!("Waiting for Github's authorization...");
        // A very naive implementation of the redirect server.
        let listener = TcpListener::bind("127.0.0.1:9000").await.unwrap();

        loop {
            if let Ok((mut stream, _)) = listener.accept().await {
                let mut reader = BufReader::new(&mut stream);

                let mut request_line = String::new();
                reader.read_line(&mut request_line).await.unwrap();

                let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                let code = url
                    .query_pairs()
                    .find(|(key, _)| key == "code")
                    .map(|(_, code)| AuthorizationCode::new(code.into_owned()))
                    .unwrap();

                let state = url
                    .query_pairs()
                    .find(|(key, _)| key == "state")
                    .map(|(_, state)| CsrfToken::new(state.into_owned()))
                    .unwrap();

                println!("code: {:?}", code);
                println!("state: {:?}", state);

                let message = "Go back to your terminal :)";
                let response = format!(
                    "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                    message.len(),
                    message
                );
                stream.write_all(response.as_bytes()).await.unwrap();

                // The server will terminate itself after collecting the first code.
                break (code, state);
            }
        }
    };

    println!("Github returned the following code:\n{}\n", code.secret());
    println!(
        "Github returned the following state:\n{} (expected `{}`)\n",
        state.secret(),
        csrf_state.secret()
    );

    // Exchange the code with a token.
    let token_res = client.exchange_code(code).request_async(&http_client).await;

    println!("Github returned the following token:\n{token_res:?}\n");

    if let Ok(token) = token_res {
        // NB: Github returns a single comma-separated "scope" parameter instead of multiple
        // space-separated scopes. Github-specific clients can parse this scope into
        // multiple scopes by splitting at the commas. Note that it's not safe for the
        // library to do this by default because RFC 6749 allows scopes to contain commas.
        let scopes = if let Some(scopes_vec) = token.scopes() {
            scopes_vec
                .iter()
                .flat_map(|comma_separated| comma_separated.split(','))
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        println!("Github returned the following scopes:\n{scopes:?}\n");
    }
}
