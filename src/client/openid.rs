use minreq::post;


pub struct OpenID {
    server_url: String,
    client_id: String,
    client_secret: String,
    verify: bool,
}

impl OpenID {
    fn url_token(server_url: &str, realm_name: &str) -> String {
        format!("{}/realms/{}/protocol/openid-connect/token", server_url, realm_name)
    }

    pub fn login_with_client(server_url: &str, realm_name: &str, client_id: &str, client_secret: &str, verify: bool) {
        let url = Self::url_token(server_url, realm_name);

        let res = post(url)
            .with_header("Content-Type", "application/x-www-form-urlencoded")
            .with_body(
                format!(
                    "client_id={}&client_secret={}&grant_type=client_credentials",
                    client_id,
                    client_secret,
                )
            ).send();
    }

    pub fn login_with_password(server_url: &str, realm_name: &str, client_id: &str, client_secret: &str, username: &str, password: &str, verify: bool) {
        
    }
}

