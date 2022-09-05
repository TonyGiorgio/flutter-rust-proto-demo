use demo_rpc::demo::demo_server::DemoServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting demo server");

    let addr = "0.0.0.0:8880".parse()?;

    let demo_service = proto_demo::DemoService {};
    let svc = DemoServer::new(demo_service);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
