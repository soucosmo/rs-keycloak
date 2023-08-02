use super::{TokenClient, TokenPassword};

#[derive(Debug)]
pub enum Token {
    Client(TokenClient),
    Password(TokenPassword),
}
