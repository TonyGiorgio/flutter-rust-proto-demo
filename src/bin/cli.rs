#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting demo server");

    proto_demo::hello_world_request().await;

    Ok(())
}
