use tonic::{transport::Server, Request, Response, Status};
use todo::todo_server::{TodoServer, Todo};
use todo::{GetTodosResponse};

pub mod todo {
    tonic::include_proto!("todo");
}

#[derive(Debug, Default)]
pub struct TodoService { }

#[tonic::async_trait]
impl Todo for TodoService {
    async fn get_todos(&self, _: Request<()>) -> Result<Response<GetTodosResponse>, Status> {
        let message = GetTodosResponse { todos: vec![] };

        Ok(Response::new(message))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();
    let todo_service = TodoService::default();

    Server::builder()
        .add_service(TodoServer::new(todo_service))
        .serve(addr)
        .await?;

    Ok(())
}
