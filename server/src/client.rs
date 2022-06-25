use todo::todo_client::TodoClient;

pub mod todo {
    tonic::include_proto!("todo");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TodoClient::connect("http://0.0.0.0:50051").await?;

    let request = tonic::Request::new(());

    let response = client.get_todos(request).await?;

    println!("RESPONSE={:?}", response.into_inner().todos);
}