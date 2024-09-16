use holopku::codegen::auth::auth_client::AuthClient;
use holopku::codegen::auth::{GetUserRequest, LoginProvider, LoginRequest, RegisterRequest};
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = AuthClient::connect("http://[::1]:8080").await?;

    println!("*** AUTHENTICATION CLIENT ***");
    println!("Try IAAA login without registration");
    let response = client
        .login(Request::new(LoginRequest {
            auth_provider: LoginProvider::Iaaa.into(),
            iaaa_token: "some-token".into(),
            username: "2200088888".into(),
            password: "mypassword".into(),
            ip_address: Some("my-ip-address".into()),
        }))
        .await;
    println!("RESPONSE = {:?}", response);

    println!("Try password login without registration");
    let response = client
        .login(Request::new(LoginRequest {
            auth_provider: LoginProvider::Password.into(),
            iaaa_token: "".into(),
            username: "laughoutloud".into(),
            password: "mypassword".into(),
            ip_address: None,
        }))
        .await;
    println!("RESPONSE = {:?}", response);

    println!("Try password registration");
    let response = client
        .register(Request::new(RegisterRequest {
            auth_provider: LoginProvider::Password.into(),
            username: "laughoutloud".into(),
            password: "mypassword".into(),
            email: "lol@example.com".into(),
        }))
        .await;
    println!("RESPONSE = {:?}", response);

    println!("Try password login again after registration");
    let response = client
        .login(Request::new(LoginRequest {
            auth_provider: LoginProvider::Password.into(),
            iaaa_token: "".into(),
            username: "laughoutloud".into(),
            password: "mypassword".into(),
            ip_address: None,
        }))
        .await;
    println!("RESPONSE = {:?}", response);

    println!("Try password registration once again, should fail");
    let response = client
        .register(Request::new(RegisterRequest {
            auth_provider: LoginProvider::Password.into(),
            username: "laughoutloud".into(),
            password: "mypassword".into(),
            email: "lol@example.com".into(),
        }))
        .await;
    println!("RESPONSE = {:?}", response);

    Ok(())
}
