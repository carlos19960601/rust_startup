use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt;

#[derive(Debug, Serialize, Deserialize)]
struct Model {
    name: String,
    size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct ListModelsRequest {
    models: Vec<Model>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GenerateCompletionRequest {
    model: String,
    prompt: String,
    stream: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GenerateCompletionResponse {
    model: String,
    response: String,
    done: bool,
}

async fn list_models(client: &Client) -> Result<ListModelsRequest, Error> {
    let response = client
        .get("http://218.216.70.114:11434/api/tags")
        .send()
        .await?;
    Ok(response.json().await?)
}

async fn generate_completion(
    client: &Client,
    request: &GenerateCompletionRequest,
) -> Result<GenerateCompletionResponse, Error> {
    let response = client
        .post("http://218.216.70.114:11434/api/generate")
        .json(&request)
        .send()
        .await?;
    let result = response.json::<GenerateCompletionResponse>().await?;
    Ok(result)
}

async fn generate_completion_stream(
    client: &Client,
    request: &mut GenerateCompletionRequest,
) -> Result<(), Error> {
    request.stream = Some(true);
    let response = client
        .post("http://218.216.70.114:11434/api/generate")
        .json(&request)
        .send()
        .await?;

    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        match item {
            Ok(bytes) => {
                println!("{}", String::from_utf8_lossy(&bytes));
            }
            Err(err) => {
                println!("err: {}", err)
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = reqwest::Client::new();

    let models = list_models(&client).await?;
    println!("{:#?}", models);

    let mut generate_completion_request = GenerateCompletionRequest {
        model: "qwen3:latest".to_string(),
        prompt: "Why is the sky blue?".to_string(),
        stream: Some(false),
    };
    let completion_response = generate_completion(&client, &generate_completion_request).await?;

    println!("{}", completion_response.response);

    generate_completion_stream(&client, &mut generate_completion_request).await?;
    Ok(())
}
