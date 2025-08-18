use anyhow::Result;
use serde_json::Value;

fn main() -> Result<()> {
    let json_str = r#"{
        "conversation_id": "1234567890",
        "completion_option": {
            "is_regen": false,
            "with_suggest": true,
            "need_create_conversation": false,
            "launch_stage": 1,
            "is_replace": false,
            "is_delete": false,
            "message_from": 1,
            "use_deep_think": false
        }
    }"#;

    // 2) 转成 JSON Schema
    let value: Value = serde_json::from_str(json_str)?;
    let schema: schemars::schema::RootSchema = schemars::schema_for_value!(value);

    println!("schema: {}", serde_json::to_string_pretty(&schema)?);

    // 3) 根据 Schema 生成 Rust struct 代码
    let settings = typify::TypeSpaceSettings::default();
    let mut ts = typify::TypeSpace::new(&settings);
    ts.add_root_schema(schema)?;

    // 4) 输出
    println!("----- Rust struct -----");
    let syn_file = syn::parse2::<syn::File>(ts.to_stream())?;
    let contents = prettyplease::unparse(&syn_file);
    println!("{}", contents);

    Ok(())
}
