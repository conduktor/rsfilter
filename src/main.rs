use tonic::{transport::Server, Request, Response, Status};

use filter_api::filter_server::{Filter, FilterServer};
use filter_api::{FilterRequest, FilterResponse};

pub mod filter_api {
    tonic::include_proto!("jsfilter");
}

#[derive(Default)]
pub struct JsFilter {}

#[tonic::async_trait]
impl Filter for JsFilter {
    async fn filter(
        &self,
        request: Request<FilterRequest>,
    ) -> Result<Response<FilterResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = filter_api::FilterResponse {
            payload: request.into_inner().payload,
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = JsFilter::default();

    println!("JsFilterServer listening on {}", addr);

    Server::builder()
        .add_service(FilterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}