use simple::simple_service_client::SimpleServiceClient;
use simple::{Address, Location};

pub mod simple {
    tonic::include_proto!("simple");
}

const GOLANG_RPC_ADDRESS: &str = "http://0.0.0.0:50052";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {   
    let mut client = SimpleServiceClient::connect(GOLANG_RPC_ADDRESS).await?;

    let req = Address{
        name: "rust-client".to_owned()
    };

    let response : tonic::Response<Location> = client.where_am_i(req).await?;
    println!("{:?}", response.into_inner());
    Ok(())
}