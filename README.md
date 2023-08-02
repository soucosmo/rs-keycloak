# RS-KEYCLOAK
Fornece uma camada de autenticação para keycloak usando rust.

## Instalação
Adicione ao projeto usando ```cargo add rs-keycloak```

## Fazendo login com o client
```rust
use client::OpenID;

fn main() {
    let server_url = "https://seuhost.com.br";
    let client_id = "client";
    let realm_name = "realm";
    let client_secret = "client_secret";

    let open_id = OpenID::login_with_client(
        server_url,
        realm_name,
        client_id,
        client_secret,
    ).unwrap();

    println!("{}", open_id.get_access_token()); // veja outros métodos disponíveis na struct OpenID
}
```

## Fazendo login com usuário e senha
```rust
use client::OpenID;

fn main() {
    let server_url = "https://seuhost.com.br";
    let client_id = "client";
    let realm_name = "realm";
    let client_secret = "client_secret";
    let username = "meuusuario";
    let password = "minhasenhasecreta";

    let open_id = OpenID::login_with_password(
        server_url,
        realm_name,
        client_id,
        client_secret,
        username,
        password,
    ).unwrap();

    println!("{}", open_id.get_access_token()); // veja outros métodos disponíveis na struct OpenID
}
```