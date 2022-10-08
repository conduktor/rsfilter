use filter_api::filter_client::FilterClient;
use filter_api::FilterRequest;
use filter_api::IsMatchingFilterRequest;
use filter_api::CreateFilterRequest;

pub mod filter_api {
    tonic::include_proto!("jsfilter");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = FilterClient::connect("http://127.0.0.1:50051").await?;


    let create_filter = tonic::Request::new(CreateFilterRequest {
        js: "function filter(payload){ return payload.a==='x'}".into(),
    });

    let id = client.create_filter(create_filter).await?;

    println!("RESPONSE={:?}", id);

    let request = tonic::Request::new(IsMatchingFilterRequest {
        id: id.into_inner().id,
        payload: "{\"a\":\"x\"}".into(),
    });

    let response = client.is_matching_filter(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}