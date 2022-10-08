use filter_api::filter_client::FilterClient;
use filter_api::FilterRequest;

pub mod filter_api {
    tonic::include_proto!("jsfilter");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = FilterClient::connect("http://127.0.0.1:50051").await?;

    let request = tonic::Request::new(FilterRequest {
        js: "function filter(payload){ return payload.a==='x'}".into(),
        payload: "{\"a\":\"x\"}".into(),
    });

    let response = client.filter(request).await?;

    println!("RESPONSE={:?}", response);

    let request2 = tonic::Request::new(FilterRequest {
        js: "function filter(payload){ return payload.a==='x'}".into(),
        payload: "{\"a\":\"y\"}".into(),
    });

    let response2 = client.filter(request2).await?;

    println!("RESPONSE={:?}", response2);

    Ok(())
}