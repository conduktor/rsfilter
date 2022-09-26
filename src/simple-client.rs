use filter_api::filter_client::FilterClient;
use filter_api::FilterRequest;

pub mod filter_api {
    tonic::include_proto!("jsfilter");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = FilterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(FilterRequest {
        js: "js function".into(),
        payload: "awesome payload".into(),
    });

    let response = client.filter(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}