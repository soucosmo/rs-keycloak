use serde_json::{Value, json};
use super::urls::OPENID_URL;
use minreq::{post, Error};
use crate::types::Token;

#[derive(Debug)]
pub struct OpenID {
    server_url: String,
    client_id: String,
    client_secret: String,
    pub token: Token,
    roles: Vec<String>,
}

impl OpenID {
    fn get_realm_roles(token: &Token) -> Vec<String>{
        match token {
            Token::Introspect(e) => {
                match e.get("realm_access") {
                    Some(ee) => {
                        let token_roles = ee.get("roles")
                        .unwrap()
                        .as_array()
                        .unwrap();

                        token_roles.iter()
                        .map(|i| i.to_string().trim_matches('\"').to_string())
                        .collect::<Vec<String>>()
                    },
                    None => vec![],
                }
            },
            _ => vec![]
        }
    }

    pub fn introspect(server_url: &str, realm_name: &str, client_id: &str, client_secret: &str, access_token: &str) -> Result<Self, Error> {
        let path = OPENID_URL.introspect.replace("{realm-name}", realm_name);

        let url = format!(
            "{}/{}",
            server_url,
            path,
        );

        let res = post(url)
            .with_header("Content-Type", "application/x-www-form-urlencoded")
            .with_body(
                format!(
                    "client_id={}&client_secret={}&token={}",
                    client_id,
                    client_secret,
                    access_token,
                )
            ).send();

        match res {
            Ok(e) => {
                if e.status_code != 200 {
                    return Err(Error::Other("Unauthorized"));
                }

                let token = Token::Introspect(e.clone().json().unwrap());

                let realm_roles = Self::get_realm_roles(&token);

                Ok(Self {
                    server_url: server_url.to_string(),
                    client_id: client_id.to_string(),
                    client_secret: client_secret.to_string(),
                    token: token,
                    roles: realm_roles,
                })
            },
            Err(e) => Err(e),
        }
    }

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
                    roles: vec![],
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
                    roles: vec![],
                })
            },
            Err(e) => Err(e),
        }
    }

    pub fn logout(server_url: &str, realm_name: &str, client_id: &str, client_secret: &str, refresh_token: &str) -> Result<(), Error> {
        let path = OPENID_URL.logout.replace("{realm-name}", realm_name);

        let url = format!(
            "{}/{}",
            server_url,
            path,
        );

        let res = post(url)
            .with_header("Content-Type", "application/x-www-form-urlencoded")
            .with_body(
                format!(
                    "client_id={}&client_secret={}&refresh_token={}",
                    client_id,
                    client_secret,
                    refresh_token,
                )
            ).send();

        match res {
            Ok(e) => {
                if e.status_code != 204 {
                    return Err(Error::Other("Unauthorized"));
                }

                Ok(())
            },
            Err(e) => Err(e),
        }
    }

    pub fn get_token_type(&self) -> String {
        match &self.token {
            Token::Client(token) => token.token_type.to_string(),
            Token::Password(token) => token.token_type.to_string(),
            _ => "".to_string(),
        }
    }

    pub fn get_access_token(&self) -> String {
        match &self.token {
            Token::Client(token) => token.access_token.to_string(),
            Token::Password(token) => token.access_token.to_string(),
            _ => "".to_string(),
        }
    }

    pub fn get_refresh_token(&self) -> Option<String> {
        match &self.token {
            Token::Client(_) => None,
            Token::Password(token) => Some(token.refresh_token.to_string()),
            _ => None,
        }
    }

    pub fn get_expires_in(&self) -> u64 {
        match &self.token {
            Token::Client(token) => token.expires_in,
            Token::Password(token) => token.expires_in,
            _ => 0,
        }
    }

    pub fn get_refresh_expires_in(&self) -> u64 {
        match &self.token {
            Token::Client(token) => token.refresh_expires_in,
            Token::Password(token) => token.refresh_expires_in,
            _ => 0,
        }
    }

    pub fn get_scopes(&self) -> Vec<&str> {
        match &self.token {
            Token::Client(token) => token.scope.split(' ').collect::<Vec<&str>>(),
            Token::Password(token) => token.scope.split(' ').collect::<Vec<&str>>(),
            _ => vec![],
        }
    }

    pub fn get_roles(&self) -> &Vec<String> {
        &self.roles
    }

    pub fn has_any_roles(&self, roles: &[&str]) -> bool {
        for role in &self.roles {
            if roles.contains(&role.as_str()) {
                return true;
            }
        }

        false
    }

    pub fn has_all_roles(&self, roles: &[&str]) -> bool {
        let roles_count = roles.len();

        let mut roles_found: usize = 0;

        for role in &self.roles {
            if roles.contains(&role.as_str()) {
                roles_found += 1;
            }
        }

        roles_count == roles_found
    }

    pub fn get_decoded_token(&self) -> Value {
        match &self.token {
            Token::Introspect(e) => e.clone(),
            _ => json!({}),
        }
    }
}
