use tonic::{transport::Server, Request, Response, Status};

use simple::simple_service_server::{SimpleService, SimpleServiceServer};
use simple::{Address, Location};

pub mod simple {
    tonic::include_proto!("simple"); 
}

const SERVER_ADDRESS: &str = "0.0.0.0:50051";
const SERVER_NAME: &str = "Rust Server";

#[derive(Debug, Default)]
pub struct SS{}

#[tonic::async_trait]
impl SimpleService for SS {
    async fn where_am_i(
        &self,
        request: Request<Address>, 
    ) -> Result<Response<Location>, Status> {

        let res = Location{
            name:request.into_inner().name,
            server_name: SERVER_NAME.to_owned(),
            timestamp: chrono::Utc::now().to_string(),
        };

        Ok(Response::new(res))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ss = SS::default();

    println!("server is listening on {}", SERVER_ADDRESS);

    Server::builder()
        .add_service(SimpleServiceServer::new(ss))
        .serve(SERVER_ADDRESS.parse()?)
        .await?;

    Ok(())
}