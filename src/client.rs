use holopku::codegen::auth::auth_client::AuthClient;
use holopku::codegen::auth::{GetUserRequest, LoginProvider, LoginRequest, RegisterRequest};
use holopku::codegen::forum::forum_client::ForumClient;
use holopku::codegen::forum::CreatePostRequest;
use holopku::AUTHORIZATION_KEY;
use hyper_util::rt::TokioExecutor;
use tonic::metadata::MetadataValue;
use tonic::{IntoRequest, Request};
use tonic_web::GrpcWebClientLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let mut auth_client = AuthClient::connect("http://[::1]:8080").await?;
    let mut forum_client = ForumClient::connect("http://[::1]:8080").await?;

    // let client = hyper_util::client::legacy::Client::builder(TokioExecutor::new()).build_http();
    // let svc = tower::ServiceBuilder::new()
    //     .layer(GrpcWebClientLayer::new())
    //     .service(client);
    // let mut auth_client = AuthClient::with_origin(svc.clone(), "http://[::1]:8080".try_into()?);
    // let mut forum_client = ForumClient::with_origin(svc.clone(), "http://[::1]:8080".try_into()?);

    println!("*** AUTHENTICATION CLIENT ***");
    println!("Try IAAA login without registration");
    let response = auth_client
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
    let response = auth_client
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
    let response = auth_client
        .register(Request::new(RegisterRequest {
            auth_provider: LoginProvider::Password.into(),
            username: "laughoutloud".into(),
            password: "mypassword".into(),
            email: "lol@example.com".into(),
        }))
        .await;
    println!("RESPONSE = {:?}", response);

    println!("Try password login again after registration");
    let response = auth_client
        .login(Request::new(LoginRequest {
            auth_provider: LoginProvider::Password.into(),
            iaaa_token: "".into(),
            username: "laughoutloud".into(),
            password: "mypassword".into(),
            ip_address: None,
        }))
        .await;
    println!("RESPONSE = {:?}", response);

    let response = response.unwrap().into_inner();
    let token = response.token;
    println!("User token = {token:#?}");
    let user_id = response.user.unwrap().id;

    println!("Try password registration once again, should fail");
    let response = auth_client
        .register(Request::new(RegisterRequest {
            auth_provider: LoginProvider::Password.into(),
            username: "laughoutloud".into(),
            password: "mypassword".into(),
            email: "lol@example.com".into(),
        }))
        .await;
    println!("RESPONSE = {:?}", response);

    println!("*** FORUM CLIENT ***");
    println!("Try CreatePost request");

    let response = forum_client
        .create_post({
            let mut create_post = CreatePostRequest {
                user_id,
                title: "NewPostTitle".into(),
                content: "This is my new post!".into(),
                images: vec![],
            }
            .into_request();
            let metadata = create_post.metadata_mut();
            metadata.append_bin(AUTHORIZATION_KEY, MetadataValue::from_bytes(&token));
            create_post
        })
        .await;
    println!("RESPONSE = {:?}", response);
    Ok(())
}
