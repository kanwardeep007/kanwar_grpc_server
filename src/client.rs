use item_detail::search_service_client::SearchServiceClient;
use tonic::{transport::Server, Request, Response, Status};

use item_detail::search_service_server::SearchService;
use item_detail::{ItemRequest, ItemResponse};

pub mod item_detail {
    tonic::include_proto!("itemdetail");
}

#[derive(Debug, Default)]
pub struct MyServiceClient {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SearchServiceClient::connect("http://[::1]:50051").await?;

    let req = ItemRequest { item_number: 2 };
    dbg!(req.item_number);
    let res = client.search_item(req).await;
    dbg!(res);
    Ok(())
}
