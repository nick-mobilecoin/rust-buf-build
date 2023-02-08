use tonic::{transport::Server, Request, Response, Status};


pub mod attest {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), concat!("/gen/attest.v1.rs")));
    }
}
pub mod enclave {
    pub mod what {
        include!(concat!(env!("OUT_DIR"), concat!("/gen/enclave.v1.rs")));
    }
}

use enclave::what::{ActualRequest, ActualResponse};
use crate::enclave::what::enclave_unary_service_server::{EnclaveUnaryService, EnclaveUnaryServiceServer};

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
            stuff: format!("Hello {}!", String::from_utf8(request.into_inner().stuff).unwrap()).into(),
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
