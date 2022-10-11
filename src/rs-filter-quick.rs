use futures::Stream;
use std::pin::Pin;
use std::sync::Arc;

use log::{debug, error, info};
use tonic::{transport::Server, Request, Response, Status};

use filter_api::filter_server::{Filter, FilterServer};
use filter_api::{
    CreateFilterRequest, CreateFilterResponse, FilterRequest, FilterResponse,
    IsMatchingFilterRequest, IsMatchingFilterResponse,
};

use rand::seq::SliceRandom;

use hirofa_utils::js_utils::{
    adapters::JsRealmAdapter,
    facades::{JsRuntimeBuilder, JsRuntimeFacade},
    Script,
};
use quickjs_runtime::{
    builder::QuickJsRuntimeBuilder, facades::QuickJsRuntimeFacade, quickjs_utils::primitives,
    quickjsrealmadapter::QuickJsRealmAdapter,
};

pub mod filter_api {
    tonic::include_proto!("jsfilter");
}

pub struct JsFilter {
    quick_js_rts: Vec<Arc<QuickJsRuntimeFacade>>,
}

#[tonic::async_trait]
impl Filter for JsFilter {
    async fn filter(
        &self,
        request: Request<FilterRequest>,
    ) -> Result<Response<FilterResponse>, Status> {
        debug!("Got a request from {:?}", request.remote_addr());

        let rt = self
            .quick_js_rts
            .choose(&mut rand::thread_rng())
            .expect("unexpected empty runtime vector")
            .clone();

        let function_id = eval_function(request.get_ref().js.clone(), &rt.clone()).unwrap();

        let filter_result =
            run_function(rt.clone(), function_id, request.get_ref().payload.clone()).await;

        match filter_result {
            Ok(res) => {
                let reply = filter_api::FilterResponse {
                    payload: res.to_string(),
                };
                return Ok(Response::new(reply));
            }
            Err(e) => {
                error!("Uncaught {}", e);
                return Err(Status::internal(e));
            }
        };
    }

    async fn create_filter(
        &self,
        request: Request<CreateFilterRequest>,
    ) -> Result<Response<CreateFilterResponse>, Status> {
        debug!("Request {:?}", request);

        info!("runtime count: {}", self.quick_js_rts.len());

        self.quick_js_rts
            .iter()
            .map(
                |rt| match eval_function(request.get_ref().js.clone(), &rt.clone()) {
                    Ok(function_id) => {
                        info!("created filter with id {}", function_id);
                        let reply = CreateFilterResponse { id: function_id };
                        return Ok(Response::new(reply));
                    }
                    Err(e) => {
                        error!("Uncaught {}", e);
                        return Err(Status::internal(e));
                    }
                },
            )
            .collect::<Vec<Result<Response<CreateFilterResponse>, Status>>>()
            .pop()
            .unwrap()
    }

    async fn is_matching_filter(
        &self,
        request: Request<IsMatchingFilterRequest>,
    ) -> Result<Response<IsMatchingFilterResponse>, Status> {
        let rt = self
            .quick_js_rts
            .choose(&mut rand::thread_rng())
            .expect("unexpected empty runtime vector")
            .clone();

        let filter_result = run_function(
            rt,
            request.get_ref().id.clone(),
            request.get_ref().payload.clone(),
        )
        .await;

        match filter_result {
            Ok(res) => {
                // println!("{}", res.to_string(&mut context).unwrap());
                let reply = IsMatchingFilterResponse { is_matching: res };
                return Ok(Response::new(reply));
            }

            Err(e) => {
                error!("Uncaught {}", e);
                return Err(Status::internal(e));
            }
        };
    }

    type continuousFilterStream =
        Pin<Box<dyn Stream<Item = Result<IsMatchingFilterResponse, Status>> + Send + 'static>>;

    async fn continuous_filter(
        &self,
        request: Request<tonic::Streaming<IsMatchingFilterRequest>>,
    ) -> Result<Response<Self::continuousFilterStream>, Status> {
        unimplemented!();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let addr = "127.0.0.1:50051".parse().unwrap();
    let qjs_rt_1 = Arc::new(QuickJsRuntimeBuilder::new().js_build());
    let qjs_rt_2 = Arc::new(QuickJsRuntimeBuilder::new().js_build());
    let qjs_rt_3 = Arc::new(QuickJsRuntimeBuilder::new().js_build());

    let js_filter_server = JsFilter {
        quick_js_rts: vec![qjs_rt_1, qjs_rt_2, qjs_rt_3],
    };

    info!("JsFilterServer listening on {}", addr);

    Server::builder()
        .add_service(FilterServer::new(js_filter_server))
        .serve(addr)
        .await?;

    Ok(())
}

fn eval_function(code: String, rt: &QuickJsRuntimeFacade) -> Result<i32, String> {
    rt.js_loop_realm_sync(None, move |_rt, realm_adapter| unsafe {
        QuickJsRealmAdapter::eval_ctx(
            realm_adapter.context,
            Script::new("filter", &format!("({});", code)),
            None,
        )
        .map_err(|_| "Error while evaluating function".to_string())
        .map(|func| realm_adapter.js_cache_add(&func))
    })
}

async fn run_function(
    rt: Arc<QuickJsRuntimeFacade>,
    function_id: i32,
    payload: String,
) -> Result<bool, String> {
    rt.js_loop_realm_sync(None, move |_rt, realm_adapter| {
        let parsed_payload = realm_adapter.js_json_parse(payload.as_str()).unwrap();
        debug!("payload: {}", payload.as_str());
        realm_adapter
            .js_cache_with(function_id, |func| {
                realm_adapter
                    .js_function_invoke(None, &func, &[&parsed_payload])
                    .and_then(|result| primitives::to_bool(&result))
            })
            .map_err(|_| "Error during execution of filter".to_string())
    })
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        filter_api::{FilterRequest, FilterResponse},
        Filter, JsFilter,
    };

    use hirofa_utils::js_utils::facades::{JsRuntimeBuilder, JsRuntimeFacade};
    use quickjs_runtime::builder::QuickJsRuntimeBuilder;

    #[tokio::test]
    async fn test_filter() {
        let qjs_rt = Arc::new(QuickJsRuntimeBuilder::new().js_build());

        let filter = JsFilter {
            quick_js_rt: qjs_rt,
        };

        let request = tonic::Request::new(FilterRequest {
            js: "function filter(payload){ return payload.a==='x'}".into(),
            payload: "{\"a\":\"x\"}".into(),
        });
        let res = filter.filter(request).await;

        let expected = FilterResponse {
            payload: "true".into(),
        };

        assert_eq!(res.unwrap().get_ref().payload, expected.payload);
    }

    #[tokio::test]
    async fn test_not_filter() {
        let qjs_rt = Arc::new(QuickJsRuntimeBuilder::new().js_build());

        let filter = JsFilter {
            quick_js_rt: qjs_rt,
        };

        let request = tonic::Request::new(FilterRequest {
            js: "function filter(payload){ return payload.a==='x'}".into(),
            payload: "{\"a\":\"y\"}".into(),
        });
        let res = filter.filter(request).await;

        let expected = FilterResponse {
            payload: "false".into(),
        };

        assert_eq!(res.unwrap().get_ref().payload, expected.payload);
    }
}
