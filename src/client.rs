use holopku::codegen::auth::auth_client::AuthClient;
use holopku::codegen::auth::{GetUserRequest, LoginProvider, LoginRequest, RegisterRequest};
use holopku::codegen::food_post::FoodPost;
use holopku::codegen::forum::forum_client::ForumClient;
use holopku::codegen::post::Post;
// use holopku::codegen::forum::CreatePostRequest;
use holopku::AUTHORIZATION_KEY;
use hyper::client::conn::http1;
use hyper::header::HeaderValue;
use hyper_util::rt::TokioExecutor;
use tonic::metadata::{MetadataKey, MetadataValue};
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
    // println!("User token = {token:#?}");
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
        .create_food_post({
            let mut create_post = holopku::codegen::forum::CreateFoodPostRequest {
                post: Some(FoodPost {
                    post: Some(Post {
                        id: 1,
                        title: "first post--test".into(),
                        user_id: user_id,
                        content: "this is the first post to test".into(),
                        likes: 0,
                        favorates: 0,
                        created_at: 0,
                        updated_at: None,
                        comments: vec![],
                        images: vec![],
                        post_type: holopku::codegen::post::PostType::Foodpost.into(),
                    }),
                    food_place: holopku::codegen::food_post::Place::JiaYuan.into(),
                    score: 0,
                }),
            }
            .into_request();
            let metadata = create_post.metadata_mut();
            metadata.append_bin(AUTHORIZATION_KEY, MetadataValue::from_bytes(&token));
            create_post
        })
        .await;
    println!("RESPONSE = {:?}", response);

    println!("Try GetPost request");
    let the_new_post_id = response?.into_inner().post_id;
    // try get it
    let response = forum_client
        .get_food_post({
            let mut get_post = holopku::codegen::forum::GetPostRequest {
                post_id: the_new_post_id,
            }
            .into_request();
            let metadata = get_post.metadata_mut();
            metadata.append_bin(AUTHORIZATION_KEY, MetadataValue::from_bytes(&token));
            get_post
        })
        .await;
    println!("RESPONSE = {:?}", response);

    println!("Try Favorate request");
    let response = forum_client
        .favorate({
            let mut favorate_post = holopku::codegen::forum::FavorateRequest {
                user_id,
                post_id: the_new_post_id,
            }
            .into_request();
            let metadata = favorate_post.metadata_mut();
            metadata.append_bin(AUTHORIZATION_KEY, MetadataValue::from_bytes(&token));
            favorate_post
        })
        .await;
    println!("RESPONSE = {:?}", response);

    println!("Try GetPost request again");
    // try get it
    let response = forum_client
        .get_food_post({
            let mut get_post = holopku::codegen::forum::GetPostRequest {
                post_id: the_new_post_id,
            }
            .into_request();
            let metadata = get_post.metadata_mut();
            metadata.append_bin(AUTHORIZATION_KEY, MetadataValue::from_bytes(&token));
            get_post
        })
        .await;
    println!("RESPONSE = {:?}", response);

    println!("Try UnFavorate request");
    let response = forum_client
        .unfavorate({
            let mut favorate_post = holopku::codegen::forum::UnfavorateRequest {
                user_id,
                post_id: the_new_post_id,
            }
            .into_request();
            let metadata = favorate_post.metadata_mut();
            metadata.append_bin(AUTHORIZATION_KEY, MetadataValue::from_bytes(&token));
            favorate_post
        })
        .await;
    println!("RESPONSE = {:?}", response);

    println!("Try GetPost request again");
    // try get it
    let response = forum_client
        .get_food_post({
            let mut get_post = holopku::codegen::forum::GetPostRequest {
                post_id: the_new_post_id,
            }
            .into_request();
            let metadata = get_post.metadata_mut();
            metadata.append_bin(AUTHORIZATION_KEY, MetadataValue::from_bytes(&token));
            get_post
        })
        .await;
    println!("RESPONSE = {:?}", response);

    // delete it
    println!("Try DeletePost request");
    let response = forum_client
        .delete_post({
            let mut delete_post = holopku::codegen::forum::DeletePostRequest {
                post_id: the_new_post_id,
                user_id: user_id,
            }
            .into_request();
            let metadata = delete_post.metadata_mut();

            metadata.append_bin(AUTHORIZATION_KEY, MetadataValue::from_bytes(&token));
            delete_post
        })
        .await;
    println!("RESPONSE = {:?}", response);

    Ok(())
}
