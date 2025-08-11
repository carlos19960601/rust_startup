use std::sync::OnceLock;

use reqwest::header;

fn get_headers() -> header::HeaderMap {
    const HEADER_MAP: OnceLock<header::HeaderMap> = OnceLock::new();
    HEADER_MAP
        .get_or_init(|| {
            let mut headers = header::HeaderMap::new();
            headers.insert(header::ACCEPT, "*/*".parse().unwrap());
            headers
        })
        .clone()
}

fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

    let response = client
        .post("https://www.doubao.com/samantha/chat/completion")
        .header(header::REFERER, "https://www.doubao.com/chat/")
        .header("Agw-js-conv", "str, str")
        .headers(get_headers())
        .send()
        .await?;

    Ok(())
}
