use tonic::{transport::Server, Request, Response, Status};


pub mod enclave {
    include!(concat!(env!("OUT_DIR"), concat!("/gen/enclave.v1.rs")));
}

use enclave::{ActualRequest, ActualResponse};
use crate::enclave::enclave_unary_service_server::{EnclaveUnaryService, EnclaveUnaryServiceServer};

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl EnclaveUnaryService for MyGreeter {
    async fn actual(
        &self,
        request: Request<ActualRequest>,
    ) -> Result<Response<ActualResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = ActualResponse {
            stuff: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(EnclaveUnaryServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
