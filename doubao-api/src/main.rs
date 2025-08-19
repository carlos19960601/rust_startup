use std::{
    fmt::Debug,
    sync::{Arc, OnceLock},
};

use anyhow::Result;
use reqwest::{Url, cookie, header};
use serde::{Deserialize, Serialize};
use uuid::{Context, Timestamp};

const DEFAULT_ASSISTANT_ID: &'static str = "497858";

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBody {
    messages: Vec<Message>,

    completion_option: CompletionOption,

    evaluate_option: EvaluateOption,

    conversation_id: String,

    local_conversation_id: String,

    local_message_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct EvaluateOption {
    web_ab_params: String,
}

#[derive(Serialize, Deserialize, Debug)]
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
            headers.insert("accept-language", "zh-CN,zh;q=0.6".parse().unwrap());
            headers.insert("agw-js-conv", "str, str".parse().unwrap());
            headers.insert("content-type", "application/json".parse().unwrap());
            headers.insert("origin", "https://www.doubao.com".parse().unwrap());
            headers.insert("referer", "https://www.doubao.com/chat/".parse().unwrap());
            headers.insert("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36".parse().unwrap());
            headers.insert("x-flow-trace", "04-001dde355be4e46300186dd2ea30f981-0010ab9dccf935eb-01".parse().unwrap());

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
            ("device_id", "7486134420028048948"), // TODO 研究怎么获取
            ("device_platform", "web"),
            ("language", "zh"),
            ("pc_version", "2.32.0"),        // TODO 研究怎么获取
            ("pkg_type", "release_version"), // TODO 研究怎么获取
            ("real_aid", "497858"),
            ("region", "CN"),
            ("samantha_web", "1"),
            ("sys_region", "CN"),
            ("tea_uuid", "7486134441724741158"), // TODO 研究怎么获取
            ("use-olympus-account", "1"),
            ("version_code", "20800"),
            ("web_id", "7486134441724741158"),
            (
                "msToken",
                "FqDLr1W1n339W2uu2ieMoPmEoQEmsyLhQfUJSl09A3qnR3UOu24Qog_sRtvCm87kwl1Zm_eFZU8A7bvz1Qm1HOkGGEe_ALqHzAMsL4pShExvR1eN56S2ZeseG-tASj7yMnVYy7RT5p_BQ9IqwUulfNr7WO_tz2EhchteUm9ohiUp8g%3D%3D",
            ),
            (
                "a_bogus",
                "Yv0jkw7EQZRbFpAGmKkJtXK53quANB8y9TT2WqBRtAcEcwFb-Cl1wqtGcxof1X1Hvbhik%2FP7Er0HDfnbTW4rUlQpLmpfuzhjrT55nysLZHh2YBkZrNRqeSUFq7ztU8TPmQ5eE%2F85WGsrZE5WnH9klpMHL%2FljBbDZFN-GV2tCP9u4BAScd7FdYFw19k3OQB%2F3s9R%3D",
            ),
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

    // 生成 UUID v1 需要一个上下文（用于处理时钟序列）和一个时间戳
    let context = Context::new(0); // 上下文编号，通常从 0 开始
    let ts = Timestamp::now(context);

    let mut local_message_id_buf = [0u8; 6];
    rand::fill(&mut local_message_id_buf);

    let uuid = uuid::Uuid::new_v1(ts, &local_message_id_buf);
    let local_message_id = uuid.to_string();

    let message = Message {
        content: r#"{"text": "天空为什么是蓝色的？"}"#.to_string(),
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
        local_message_id: local_message_id, // 在页面搜索 createLocalMessageId
    };

    println!("request_body: {:?}", request_body);
    println!("url: {}", url.to_string());
    println!("headers: {:?}", get_headers());

    // let response = client
    //     .post(url)
    //     .header(header::COOKIE, "odin_tt=d5991763f8a9ed3d9087b956151f5fd5d74280f6962ac16d748e2fe242c1074a297d8ff19b66cf36857269a477920361a00fc9f64d4ea3d7dcd8a4ba56cdd5fb; uid_tt=dcb265b51787f32882218cbad85507dc; uid_tt_ss=dcb265b51787f32882218cbad85507dc; sid_tt=f47821dcbe77b6fad1a14d525895981c; sessionid=f47821dcbe77b6fad1a14d525895981c; sessionid_ss=f47821dcbe77b6fad1a14d525895981c; is_staff_user=false; i18next=zh; flow_user_country=CN; sid_guard=f47821dcbe77b6fad1a14d525895981c%7C1754484569%7C5184000%7CSun%2C+05-Oct-2025+12%3A49%3A29+GMT; sid_ucp_v1=1.0.0-KDAxZWE2MjNjMTIxZjAwMjYxYmU1NDBhMmQ4ZDkzODk2Y2Q2YzZhNzEKIAiE5NCF8sy5BBDZns3EBhjCsR4gDDDTl8izBjgHQPQHGgJobCIgZjQ3ODIxZGNiZTc3YjZmYWQxYTE0ZDUyNTg5NTk4MWM; ssid_ucp_v1=1.0.0-KDAxZWE2MjNjMTIxZjAwMjYxYmU1NDBhMmQ4ZDkzODk2Y2Q2YzZhNzEKIAiE5NCF8sy5BBDZns3EBhjCsR4gDDDTl8izBjgHQPQHGgJobCIgZjQ3ODIxZGNiZTc3YjZmYWQxYTE0ZDUyNTg5NTk4MWM; gd_random=eyJtYXRjaCI6dHJ1ZSwicGVyY2VudCI6MC45OTE3MjQzMDg0OTMxMDl9.w8VXQI43p/B9x5P3LY1Cly/KPI+j/mHjcpWA+O8HU0w=; ttwid=1%7Cb5SwlwhismUUkt0qHjhQvoDHT1HOH1HKtIwON-y-InY%7C1755393545%7C50156d53ed2261c1ef7a45ca06bf78173d812361be2887eca665b25178c6ff26; tt_scid=OBl7W6OP1JfSEt57sdhuuipRmcTeClxxDZZt7Nw.7e0CTl3yetTuPxf0iIQt8Jvicfea; flow_ssr_sidebar_expand=0; session_tlb_tag=sttt%7C15%7C9Hgh3L53tvrRoU1SWJWYHP_________YSd9vNg_mImpQFTL5YxI-LUcQomahv2pwEvISB0NEkNU%3D; msToken=LOYwsl7Xy_qfBmgWZrCCnaRb_zrWn_b0NJpGnJVHSF7VLvoX9tHiBM5QNLOfSXT4euvIaoCx2-CuJ6WVKYJXS3igHdAzQkEXXZcN3uX724zdhI024PjOjgkSYfl98eomZbUk; passport_fe_beating_status=true")
    //     .headers(get_headers())
    //     .json(&request_body)
    //     .send()
    //     .await?;

    // println!("status: {}", response.status());

    // println!("response: {}", response.text().await?);

    Ok(())
}
