use std::sync::OnceLock;

use anyhow::Result;
use reqwest::{Url, header};
use serde::{Deserialize, Serialize};

const DEFAULT_ASSISTANT_ID: &'static str = "497858";

#[derive(Serialize, Deserialize)]
pub struct RequestBody {
    messages: Vec<Message>,

    completion_option: CompletionOption,

    evaluate_option: EvaluateOption,

    conversation_id: String,

    local_conversation_id: String,

    local_message_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct CompletionOption {
    is_regen: bool,

    with_suggest: bool,

    need_create_conversation: bool,

    launch_stage: i64,

    is_replace: bool,

    is_delete: bool,

    message_from: i64,

    use_deep_think: bool,

    use_auto_cot: bool,

    resend_for_regen: bool,

    event_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct EvaluateOption {
    web_ab_params: String,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    content: String,

    content_type: i64,

    attachments: Vec<Option<serde_json::Value>>,

    references: Vec<Option<serde_json::Value>>,
}

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

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::new();

    let url = Url::parse_with_params(
        "https://www.doubao.com/samantha/chat/completion",
        &[
            ("aid", DEFAULT_ASSISTANT_ID),
            ("device_id", ""),
            ("device_platform", "web"),
            ("language", "zh"),
            ("region", "CN"),
            ("sys_region", "CN"),
        ],
    )?;

    let mut local_message_buf = [0u8; 16];
    rand::fill(&mut local_message_buf);
    let local_message_str = local_message_buf
        .iter()
        .map(|b| format!("{}", b))
        .collect::<String>()
        .chars()
        .take(16)
        .collect::<String>();

    let uuid = uuid::Uuid::new_v1();

    let message = Message {
        content: "天空为什么是蓝色的?".to_string(),
        content_type: 2001,
        attachments: vec![],
        references: vec![],
    };

    let completion_option = CompletionOption {
        is_regen: false,
        with_suggest: true,
        need_create_conversation: true, // conversation 第一次请求的时候设置为 true
        launch_stage: 1,
        is_replace: false,
        is_delete: false,
        message_from: 0,
        use_deep_think: false,
        use_auto_cot: true,
        resend_for_regen: false,
        event_id: "0".to_string(),
    };

    let evaluate_option = EvaluateOption {
        web_ab_params: "".to_string(),
    };

    let request_body = RequestBody {
        messages: vec![message],
        completion_option: completion_option,
        evaluate_option: evaluate_option,
        conversation_id: "0".to_string(),
        local_conversation_id: format!("local_{}", local_message_str), // 在页面搜索 local_$
        local_message_id: "".to_string(), // 在页面搜索 createLocalMessageId
    };

    let response = client
        .post(url)
        .header(header::REFERER, "https://www.doubao.com/chat/")
        .header("Agw-js-conv", "str, str")
        .headers(get_headers())
        .json(&request_body)
        .send()
        .await?;

    println!("status: {}", response.status());

    println!("response: {}", response.text().await?);

    Ok(())
}
