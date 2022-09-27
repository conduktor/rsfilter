use boa_engine::property::Attribute;
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
        // println!("Got a request from {:?}", request.remote_addr());

        // Instantiate the execution context
        let mut context = Context::default();
        context.register_global_property("json_value", request.get_ref().payload.clone(), Attribute::READONLY);
        let js_code = format!(r#"
            let filter = {};
            let value = JSON.parse(json_value);
            filter(value);
        "#,request.get_ref().js);
    

        match context.eval(js_code) {
            Ok(res) => {
                // println!("{}", res.to_string(&mut context).unwrap());
                let reply = filter_api::FilterResponse {
                    payload: res.to_string(&mut context).unwrap().to_string(),
                };
                return Ok(Response::new(reply))
            }
            Err(e) => {
                // Pretty print the error
                // eprintln!("Uncaught {}", e.display());
                let reply = filter_api::FilterResponse {
                    payload: "💥 ERROR SYSTEM 💥".into(),
                };
                return Ok(Response::new(reply))
            }
        };
        

        
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse().unwrap();
    let greeter = JsFilter::default();

    println!("JsFilterServer listening on {}", addr);

    Server::builder()
        .add_service(FilterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Filter,JsFilter, filter_api::{FilterRequest, FilterResponse}};


    #[tokio::test]
    async fn test_filter() {
        let filter = JsFilter::default();
        let request = tonic::Request::new(FilterRequest {
            js: "(payload) => payload.a==='x'".into(),
            payload: "{\"a\":\"x\"}".into(),
        });
        let res = filter.filter(request).await;

        let expected = FilterResponse{
            payload: "{\"a\":\"x\"}".into(),
        };

        assert_eq!(res.unwrap().get_ref().payload, expected.payload);
    }
  
 #[tokio::test]
    async fn test_not_filter() {
        let filter = JsFilter::default();
        let request = tonic::Request::new(FilterRequest {
            js: "(payload) => payload.a==='x'".into(),
            payload: "{\"a\":\"y\"}".into(),
        });
        let res = filter.filter(request).await;

        let expected = FilterResponse{
            payload: "false".into(),
        };

        assert_eq!(res.unwrap().get_ref().payload, expected.payload);
    }
}