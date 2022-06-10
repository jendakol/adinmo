use err_context::AnyError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRes {
    pub foo: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub json: JsonRes,
}

// TODO some error handling would be fine
// TODO some logging would be fine too
#[tokio::main(flavor = "current_thread")] // this is just enough for this task
async fn main() -> Result<(), AnyError> {
    let client = reqwest::ClientBuilder::default().build()?;

    let resp = client.post("https://httpbin.org/post") // TODO make it constant
        .json(&JsonRes { foo: "bar".to_owned() }) // this adds content-type header
        .send()
        .await?;

    let headers = &resp.headers().iter().map(|(name, value)| format!("{} -> {:?}", name, value)).collect::<Vec<_>>();
    println!("Headers: {:?}", headers);

    let _resp_json = resp.json::<Response>().await?.json;

    // it was not requested to print it, just parse it ;-)
    // println!("Received: {:?}", &_resp_json);

    Ok(())
}
