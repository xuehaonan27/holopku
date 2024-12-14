use crate::codegen::auth::auth_client::AuthClient;
use crate::codegen::auth::{GetUserRequest, LoginProvider, LoginRequest, RegisterRequest};
use crate::codegen::food_post::FoodPost;
use crate::codegen::forum::forum_client::ForumClient;
use crate::codegen::post::Post;
use tokio::runtime::Runtime;

use crate::AUTHORIZATION_KEY;
use hyper::client::conn::http1;
use hyper::header::HeaderValue;
use hyper_util::rt::TokioExecutor;
use tonic::metadata::{MetadataKey, MetadataValue};
use tonic::{IntoRequest, Request};
use tonic_web::GrpcWebClientLayer;

/// 前提：数据库中没有名为test_user_ne的用户
#[test]
fn login_without_register() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new().expect("Failed to create runtime");
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let mut auth_client = rt.block_on(AuthClient::connect("http://[::1]:8080"))?;

    println!("Try password login without registration");
    let response = auth_client.login(Request::new(LoginRequest {
        auth_provider: LoginProvider::Password.into(),
        iaaa_token: "".into(),
        username: "test_user_ne".into(),
        password: "mypassword".into(),
        ip_address: None,
    }));
    let response = rt.block_on(response);
    println!("RESPONSE = {:?}", response);

    assert!(response.is_err());
    Ok(())
}

/// 前提：数据库中没有名为test_user_note的用户
#[test]
fn register() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new().expect("Failed to create runtime");
    //log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let mut auth_client = rt.block_on(AuthClient::connect("http://[::1]:8080"))?;

    println!("Try password registration");
    let response = auth_client.register(Request::new(RegisterRequest {
        auth_provider: LoginProvider::Password.into(),
        username: "test_user_note".into(),
        password: "mypassword".into(),
        email: "lol@example.com".into(),
    }));
    let response = rt.block_on(response);
    println!("RESPONSE = {:?}", response);

    assert!(response.is_ok());
    assert!(response.unwrap().into_inner().success);
    Ok(())
}

/// 前提：数据库中有名为test_user的用户
#[test]
fn login() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new().expect("Failed to create runtime");
    //log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let mut auth_client = rt.block_on(AuthClient::connect("http://[::1]:8080"))?;

    println!("Try password login again after registration");
    let response = auth_client.login(Request::new(LoginRequest {
        auth_provider: LoginProvider::Password.into(),
        iaaa_token: "".into(),
        username: "test_user".into(),
        password: "mypassword".into(),
        ip_address: None,
    }));
    let response = rt.block_on(response);
    println!("RESPONSE = {:?}", response);

    assert!(response.is_ok());
    assert!(response.unwrap().into_inner().success);
    Ok(())
}

/// 前提：数据库中有名为test_user的用户
#[test]
fn register_again() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new().expect("Failed to create runtime");
    //log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let mut auth_client = rt.block_on(AuthClient::connect("http://[::1]:8080"))?;

    println!("Try password registration once again, should fail");
    let response = auth_client.register(Request::new(RegisterRequest {
        auth_provider: LoginProvider::Password.into(),
        username: "test_user".into(),
        password: "mypassword".into(),
        email: "lol@example.com".into(),
    }));
    let response = rt.block_on(response);
    println!("RESPONSE = {:?}", response);

    assert!(response.is_err());
    Ok(())
}

/// 前提：数据库中有名字为test_user的用户
#[test]
fn send_post_and_delete() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new().expect("Failed to create runtime");
    //log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let mut auth_client = rt.block_on(AuthClient::connect("http://[::1]:8080"))?;
    let mut forum_client = rt.block_on(ForumClient::connect("http://[::1]:8080"))?;
    println!("Try password login again after registration");
    let response = auth_client.login(Request::new(LoginRequest {
        auth_provider: LoginProvider::Password.into(),
        iaaa_token: "".into(),
        username: "test_user".into(),
        password: "mypassword".into(),
        ip_address: None,
    }));
    let response = rt.block_on(response);
    println!("RESPONSE = {:?}", response);

    let response = response.unwrap().into_inner();
    let token = response.token;
    // println!("User token = {token:#?}");
    let user_id = response.user.unwrap().id;
    println!("Try CreatePost request");
    let response = forum_client.create_food_post({
        let mut create_post = crate::codegen::forum::CreateFoodPostRequest {
            post: Some(FoodPost {
                post: Some(Post {
                    id: 1,
                    title: "first post--test".into(),
                    user_id: 1,
                    content: "this is the first post to test".into(),
                    likes: 0,
                    favorates: 0,
                    created_at: 0,
                    updated_at: None,
                    comments: vec![],
                    images: vec![],
                    post_type: crate::codegen::post::PostType::Foodpost.into(),
                }),
                food_place: crate::codegen::food_post::Place::JiaYuan.into(),
                score: 0,
            }),
        }
        .into_request();
        let metadata = create_post.metadata_mut();
        metadata.append_bin(AUTHORIZATION_KEY, MetadataValue::from_bytes(&token));
        create_post
    });
    let response = rt.block_on(response);
    println!("RESPONSE = {:?}", response);

    assert!(response.is_ok());

    println!("Try GetPost request");
    let the_new_post_id = response?.into_inner().post_id;
    // try get it
    let response = forum_client.get_food_post({
        let mut get_post = crate::codegen::forum::GetPostRequest {
            post_id: the_new_post_id,
        }
        .into_request();
        let metadata = get_post.metadata_mut();
        metadata.append_bin(AUTHORIZATION_KEY, MetadataValue::from_bytes(&token));
        get_post
    });
    let response = rt.block_on(response);
    println!("RESPONSE = {:?}", response);
    assert!(response.is_ok());

    println!("Try Favorate request");
    let response = forum_client.favorate({
        let mut favorate_post = crate::codegen::forum::FavorateRequest {
            user_id,
            post_id: the_new_post_id,
        }
        .into_request();
        let metadata = favorate_post.metadata_mut();
        metadata.append_bin(AUTHORIZATION_KEY, MetadataValue::from_bytes(&token));
        favorate_post
    });
    let response = rt.block_on(response);
    println!("RESPONSE = {:?}", response);
    assert!(response.is_ok());

    println!("Try GetPost request again");
    // try get it
    let response = forum_client.get_food_post({
        let mut get_post = crate::codegen::forum::GetPostRequest {
            post_id: the_new_post_id,
        }
        .into_request();
        let metadata = get_post.metadata_mut();
        metadata.append_bin(AUTHORIZATION_KEY, MetadataValue::from_bytes(&token));
        get_post
    });
    let response = rt.block_on(response);
    println!("RESPONSE = {:?}", response);
    assert!(response.is_ok());

    println!("Try UnFavorate request");
    let response = forum_client.unfavorate({
        let mut favorate_post = crate::codegen::forum::UnfavorateRequest {
            user_id,
            post_id: the_new_post_id,
        }
        .into_request();
        let metadata = favorate_post.metadata_mut();
        metadata.append_bin(AUTHORIZATION_KEY, MetadataValue::from_bytes(&token));
        favorate_post
    });
    let response = rt.block_on(response);
    println!("RESPONSE = {:?}", response);
    assert!(response.is_ok());

    println!("Try GetPost request again");
    // try get it
    let response = forum_client.get_food_post({
        let mut get_post = crate::codegen::forum::GetPostRequest {
            post_id: the_new_post_id,
        }
        .into_request();
        let metadata = get_post.metadata_mut();
        metadata.append_bin(AUTHORIZATION_KEY, MetadataValue::from_bytes(&token));
        get_post
    });
    let response = rt.block_on(response);
    println!("RESPONSE = {:?}", response);
    assert!(response.is_ok());

    // delete it
    println!("Try DeletePost request");
    let response = forum_client.delete_post({
        let mut delete_post = crate::codegen::forum::DeletePostRequest {
            post_id: the_new_post_id,
            user_id: user_id,
        }
        .into_request();
        let metadata = delete_post.metadata_mut();

        metadata.append_bin(AUTHORIZATION_KEY, MetadataValue::from_bytes(&token));
        delete_post
    });
    let response = rt.block_on(response);
    println!("RESPONSE = {:?}", response);
    assert!(response.is_ok());

    // get user
    println!("Try GetUser request");
    let response = auth_client.get_user({
        let mut delete_post =
            crate::codegen::auth::GetUserRequest { user_id: user_id }.into_request();
        let metadata = delete_post.metadata_mut();

        metadata.append_bin(AUTHORIZATION_KEY, MetadataValue::from_bytes(&token));
        delete_post
    });
    let response = rt.block_on(response);
    println!("RESPONSE = {:?}", response);
    assert!(response.is_ok());

    Ok(())
}
