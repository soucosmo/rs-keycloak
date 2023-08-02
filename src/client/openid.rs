use super::urls::OPENID_URL;
use minreq::{post, Error};
use crate::types::Token;

#[derive(Debug)]
pub struct OpenID {
    server_url: String,
    client_id: String,
    client_secret: String,
    pub token: Token,
}

impl OpenID {
    pub fn login_with_client(server_url: &str, realm_name: &str, client_id: &str, client_secret: &str) -> Result<Self, Error> {
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
                    "client_id={}&client_secret={}&grant_type=client_credentials",
                    client_id,
                    client_secret,
                )
            ).send();

        match res {
            Ok(e) => {
                if e.status_code != 200 {
                    return Err(Error::Other("Unauthorized"));
                }

                let token = Token::Client(e.clone().json().unwrap());

                Ok(Self {
                    server_url: server_url.to_string(),
                    client_id: client_id.to_string(),
                    client_secret: client_secret.to_string(),
                    token: token,
                })
            },
            Err(e) => Err(e),
        }
    }

    pub fn login_with_password(server_url: &str, realm_name: &str, client_id: &str, client_secret: &str, username: &str, password: &str) -> Result<Self, Error> {
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
                    "scope=openid&client_id={}&client_secret={}&grant_type=password&username={}&password={}",
                    client_id,
                    client_secret,
                    username,
                    password,
                )
            ).send();

        match res {
            Ok(e) => {
                if e.status_code != 200 {
                    return Err(Error::Other("Unauthorized"));
                }

                let token = Token::Password(e.clone().json().unwrap());

                Ok(Self {
                    server_url: server_url.to_string(),
                    client_id: client_id.to_string(),
                    client_secret: client_secret.to_string(),
                    token: token,
                })
            },
            Err(e) => Err(e),
        }
    }

    pub fn get_token_type(&self) -> String {
        match &self.token {
            Token::Client(token) => token.token_type.to_string(),
            Token::Password(token) => token.token_type.to_string(),
        }
    }

    pub fn get_access_token(&self) -> String {
        match &self.token {
            Token::Client(token) => token.access_token.to_string(),
            Token::Password(token) => token.access_token.to_string(),
        }
    }

    pub fn get_refresh_token(&self) -> Option<String> {
        match &self.token {
            Token::Client(_) => None,
            Token::Password(token) => Some(token.refresh_token.to_string()),
        }
    }

    pub fn get_expires_in(&self) -> u64 {
        match &self.token {
            Token::Client(token) => token.expires_in,
            Token::Password(token) => token.expires_in,
        }
    }

    pub fn get_refresh_expires_in(&self) -> u64 {
        match &self.token {
            Token::Client(token) => token.refresh_expires_in,
            Token::Password(token) => token.refresh_expires_in,
        }
    }

    pub fn get_scopes(&self) -> Vec<&str> {
        match &self.token {
            Token::Client(token) => token.scope.split(' ').collect::<Vec<&str>>(),
            Token::Password(token) => token.scope.split(' ').collect::<Vec<&str>>(),
        }
    }
}
