use tonic::{transport::Server, Request, Response, Status};

use item_detail::search_service_server::{SearchService, SearchServiceServer};
use item_detail::{ItemRequest, ItemResponse};

pub mod item_detail {
    tonic::include_proto!("itemdetail");
}

#[derive(Debug, Default)]
pub struct MyService {}

#[tonic::async_trait]
impl SearchService for MyService {
    async fn search_item(
        &self,
        request: Request<ItemRequest>,
    ) -> Result<Response<ItemResponse>, Status> {
        println!("Got a request: {:?}", request);
        let inner_req = request.into_inner();

        let reply = match inner_req.item_number {
            1 => ItemResponse {
                name: "tv".into(),
                price: 100,
                currency: "Pounds".into(),
            },
            2 => ItemResponse {
                name: "bayblade".into(),
                price: 10,
                currency: "Pounds".into(),
            },
            3 => ItemResponse {
                name: "flower".into(),
                price: 3,
                currency: "Pounds".into(),
            },
            _ => ItemResponse {
                name: "pencil".into(),
                price: 13,
                currency: "Pounds".into(),
            },
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let item_service = MyService::default();

    Server::builder()
        .add_service(SearchServiceServer::new(item_service))
        .serve(addr)
        .await?;

    Ok(())
}
