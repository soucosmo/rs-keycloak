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

## Fazendo logout com refresh_token
```rust
use client::OpenID;

fn main() {
    let server_url = "https://seuhost.com.br";
    let client_id = "client";
    let realm_name = "realm";
    let client_secret = "client_secret";
    let refresh_token = "refresh token";

    let open_id = OpenID::logout(
        server_url,
        realm_name,
        client_id,
        client_secret
        refresh_token,
    );

    if open_id.is_ok() {
        println!("Logout bem sucedido");
    } else {
        println!("Erro ao deslogar");
    }

}
```

## Fazendo o introspect do token e validando roles
```rust
use client::OpenID;

fn main() {
    let server_url = "https://seuhost.com.br";
    let client_id = "client";
    let realm_name = "realm";
    let client_secret = "client_secret";
    let access_token = "eyJhbGciO.....H38Xw";

    let open_id = OpenID::introspect(
        server_url,
        realm_name,
        client_id,
        client_secret,
        access_token,
    ).unwrap();
    
    // Obtendo todas as roles
    let roles = open_id.get_roles();
    
    // printando todas as roles
    println!("{:?}", roles);
    
    // validando se o token possui
    // Será verdadeiro se existir ALGUMA das roles informadas
    let uma_ou_outra = open_id.has_any_roles(&["cobrancas.boleto", "user"]);

    println!("Alguma persmissão existe? {:?}", uma_ou_outra);
    
    // validando se o token possui
    // Será verdadeiro se existir ALGUMA das roles informadas
    let todas =    open_id.has_all_roles(&["cobrancas.boleto", "uma_authorization"]);

    println!("Todas as permissões existem? {:?}", todas);
}
```