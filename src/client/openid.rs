use crate::types::{TokenClient, TokenPassword};
use minreq::{post, Error};
use super::urls::OPENID_URL;
pub struct OpenID {
    server_url: String,
    client_id: String,
    client_secret: String,
    login_client_response: TokenClient,
    login_password_response: TokenPassword,
}

impl OpenID {
    pub fn login_with_client(server_url: &str, realm_name: &str, client_id: &str, client_secret: &str) -> Result<String, Error> {
        let path = OPENID_URL.token.replace("{realm-name}", realm_name);

        let url = format!(
            "{}/{}",
            server_url,
            path,
        );

        let res = post(url)
            .with_header("Content-Type", "application/x-www-form-urlencoded")
            .with_body(
                format!(
                    "client_id={}&client_secrcet={}&grant_type=client_credentials",
                    client_id,
                    client_secret,
                )
            ).send();

        let mut reason_phrase = "".to_string();

        match res {
            Ok(e) => {

                if e.status_code == 200 {
                    let reason_phrase = e.reason_phrase;
                    let x = reason_phrase;

                    Err(Error::InvalidUtf8InResponse(x))
                } else {
                    let token: TokenClient = e.clone().json().unwrap();

                    println!("{}", token.access_token);
                    // Ok(Self {
                    //     server_url.to_string(),
                    //     client_id,
                    //     client_secret,
                    // })
                    Ok("AA".to_string())
                    
                }

            },
            Err(e) => Err(e),
        }
    }

    pub fn login_with_password(server_url: &str, realm_name: &str, client_id: &str, client_secret: &str, username: &str, password: &str) {
        let url = OPENID_URL.token.replace("{realm-name}", realm_name);

        let res = post(url)
            .with_header("Content-Type", "application/x-www-form-urlencoded")
            .with_body(
                format!(
                    "client_id={}&client_secret={}&grant_type=password&username={}&password={}",
                    client_id,
                    client_secret,
                    username,
                    password,
                )
            ).send();
    }
}

