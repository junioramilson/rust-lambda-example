use lambda_http::{run, service_fn, Body, Error, Request, Response, aws_lambda_events::serde_json::json};

async fn handler(event: Request) -> Result<Response<Body>, Error> {
    println!("Event: {:?}", event);

    let resp_body = json!({
        "message": String::from("test")
    });

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(resp_body.to_string().into())
        .map_err(Box::new)?;

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_level(true)
        .init();

    run(service_fn(handler)).await
}
