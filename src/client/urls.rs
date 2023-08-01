struct OpenIDUrl {
    pub token: &'static str,
    pub userinfo: &'static str,
    pub introspect: &'static str,
    pub logout : &'static str,
}

pub const OPENID: OpenIDUrl = OpenIDUrl {
    url_well_known: "realms/{realm-name}/.well-known/openid-configuration",
    url_token: "realms/{realm-name}/protocol/openid-connect/token",
    url_userinfo: "realms/{realm-name}/protocol/openid-connect/userinfo",
    url_introspect: "realms/{realm-name}/protocol/openid-connect/token/introspect",
    logout : "realms/{realm-name}/protocol/openid-connect/logout",
};
