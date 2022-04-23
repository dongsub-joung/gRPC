use tonic::{transport::Server, Request, Response, Status};
use hello::say_server::{Say, SayServer};
use hello::{SayResponse, SayRequest};

mod hello;

#[derive(Default)]
pub struct MySay {}

#[tonic::async_trait]
impl Say for MySay {
    async fn send(&self, request: Request<SayRequest>) -> Result<Response<SayResponse>, Status>{
        Ok(Response::new(SayResponse{
            message:format!("hello {}", request.get_ref().name),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let addr= "[::1]:50051".parse().unwrap();
    let say= MySay::default();
    println!("server listeing on {}", addr);

    Server::builder()
        .add_service(SayServer::new(say))
        .server(addr)
        .await?;
    Ok(())
}
