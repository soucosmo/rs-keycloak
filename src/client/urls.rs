pub struct OpenIDUrl {
    pub well_known: &'static str,
    pub token: &'static str,
    pub userinfo: &'static str,
    pub introspect: &'static str,
    pub logout : &'static str,
}

pub const OPENID_URL: OpenIDUrl = OpenIDUrl {
    well_known: "realms/{realm-name}/.well-known/openid-configuration",
    token: "realms/{realm-name}/protocol/openid-connect/token",
    userinfo: "realms/{realm-name}/protocol/openid-connect/userinfo",
    introspect: "realms/{realm-name}/protocol/openid-connect/token/introspect",
    logout : "realms/{realm-name}/protocol/openid-connect/logout",
};
