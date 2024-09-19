//! HoloPKU hello module (healthy check).

use log::trace;
use tonic::{Request, Response, Status};

use crate::codegen::hello::hello_server::Hello;
use crate::codegen::hello::{HelloRequest, HelloResponse};

#[derive(Debug)]
pub struct HelloService {}

#[tonic::async_trait]
impl Hello for HelloService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let req = request.into_inner();
        trace!("SayHello got request: {req:#?}");

        let response = HelloResponse {
            message: "Hello, world!".into(),
        };
        Ok(Response::new(response))
    }
}
