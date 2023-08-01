use minreq::post;

pub struct KeyckoakOpenID {
    server_url: String,
    realm_name: String,
    client_id: String,
    client_secret: String,
    verify: bool,
}

impl KeyckoakOpenID {
    pub fn new(&self, server_url: &str, realm_name: &str, client_id: &str, client_secret: &str, verify: bool) {
        
    }
}

