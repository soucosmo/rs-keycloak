use super::{TokenClient, TokenPassword};
use serde_json::Value;


#[derive(Debug)]
pub enum Token {
    Client(TokenClient),
    Password(TokenPassword),
    Introspect(Value),
}
