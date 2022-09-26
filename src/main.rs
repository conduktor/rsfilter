use tonic::{transport::Server, Request, Response, Status};

use filter_api::filter_server::{Filter, FilterServer};
use filter_api::{FilterRequest, FilterResponse};
use boa_engine::Context;


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

        let js_code = "console.log('Hello World from a JS code string!')";

        // Instantiate the execution context
        let mut context = Context::default();
    

        match context.eval(js_code) {
            Ok(res) => {
                println!("{}", res.to_string(&mut context).unwrap());
            }
            Err(e) => {
                // Pretty print the error
                eprintln!("Uncaught {}", e.display());
            }
        };
        

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