use holopku::auth::AuthService;
use holopku::codegen::auth::auth_server::AuthServer;
use holopku::codegen::forum::forum_server::ForumServer;
use holopku::db::DBClient;
use holopku::forum::ForumService;
use holopku::middleware::auth_interceptor;
use log::trace;
use std::env;
use std::time::Duration;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load configuration
    dotenvy::dotenv().ok();
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let iaaa_id = env::var("IAAA_ID").expect("Must set IAAA_ID");
    let iaaa_key = env::var("IAAA_KEY").expect("Must set IAAA_KEY");
    let addr = env::var("LISTEN_ADDR").expect("Must set LISTEN_ADDR");
    // let jwt_secret = env::var("JWT_SECRET").expect("Must set JWT_SECRET");

    // establish database connection
    let client = DBClient::connect(&database_url)?;
    let addr = addr.parse().unwrap();
    trace!("Auth server listening on: {}", addr);

    let auth_srv = AuthService {
        client: client.clone(),
        iaaa_id,
        iaaa_key,
    };
    let auth_srv = AuthServer::new(auth_srv);

    let forum_srv = ForumService {
        client: client.clone(),
    };
    let forum_srv = ForumServer::with_interceptor(forum_srv, auth_interceptor);

    Server::builder()
        .accept_http1(true)
        // .timeout(Duration::from_secs(5))
        .add_service(tonic_web::enable(auth_srv))
        .add_service(tonic_web::enable(forum_srv))
        .serve(addr)
        .await?;

    Ok(())
}
