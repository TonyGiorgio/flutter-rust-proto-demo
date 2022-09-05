use std::pin::Pin;

use futures::Stream;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use tonic::{transport::Server, Request, Response, Status};

use demo_rpc::demo::demo_client::DemoClient;
use demo_rpc::demo::demo_server::{Demo, DemoServer};

use demo_rpc::demo::{HelloRequest, HelloResponse};

pub struct DemoService {}

#[tonic::async_trait]
impl Demo for DemoService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Got a request: {:?}", request);
        Ok(Response::new(HelloResponse {
            message: "hello world".into(),
        }))
    }
}

pub async fn hello_world_request() -> String {
    let addr = "http://0.0.0.0:8880";

    let grpc_client = create_grpc_client(addr.to_string()).await;

    let response = grpc_client
        .clone()
        .say_hello(tonic::Request::new(HelloRequest { name: "me".into() }))
        .await;

    let response = match response {
        Ok(response) => {
            let response = response.into_inner().message;
            println!("received a response: {}", response);
            response
        }
        Err(e) => {
            println!("received an error: {}", e);
            "error".to_string()
        }
    };
    response
}

pub async fn create_grpc_client(addr: String) -> DemoClient<tonic::transport::Channel> {
    let channel = tonic::transport::Channel::from_shared(addr)
        .unwrap() // TODO this should probably not be unwrapped
        .connect()
        .await
        .expect("Can't create a channel");

    DemoClient::new(channel)
}
