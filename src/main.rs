use libhtmlfilter;
use openai_api_rust::*;
use openai_api_rust::chat::*;
use std::fs;
use std::io::Write;
use std::time::SystemTime;
use toml;
use dotenv::dotenv;
use std::env;
use std::path::Path;

fn main() {
    dotenv().ok();
    // 解析命令行参数
    let args: Vec<String> = env::args().collect();
    let url = args.iter().position(|r| r == "--url")
        .map(|p| args.get(p + 1))
        .flatten()
        .expect("URL is required");
    let tags: Vec<&str> = args.iter().position(|r| r == "--tag")
        .map(|p| args.get(p + 1))
        .flatten()
        .map(|s| s.split(',').collect())
        .unwrap_or_else(Vec::new);
    let classes: Vec<&str> = args.iter().position(|r| r == "--class")
        .map(|p| args.get(p + 1))
        .flatten()
        .map(|s| s.split(',').collect())
        .unwrap_or_else(Vec::new);

    // 调用过滤函数
    let filtered_html = libhtmlfilter::get_filtered_html_fullurl_removeref(url, &tags, &classes);

    // 打印过滤后的 HTML
    //println!("Filtered HTML:\n{}", filtered_html);

    // 读取.env文件中的auth token
    let auth_token = env::var("AUTH_TOKEN").expect("AUTH_TOKEN not found in .env file");

    // 读取config.toml文件中的模板和提示词片段
    let config_content = fs::read_to_string("config.toml").expect("Unable to read config.toml");
    let config: toml::Value = toml::from_str(&config_content).expect("Unable to parse config.toml");

    // 获取 general 部分
    let general = config.get("general")
        .expect("General section not found in config.toml")
        .as_table()
        .expect("General section is not a table");

    // 获取模板和提示词片段
    let template = general.get("template")
        .expect("Template not found in config.toml")
        .as_str()
        .expect("Template is not a string");
    let prompt_fragment = general.get("prompt")
        .expect("Prompt fragment not found in config.toml")
        .as_str()
        .expect("Prompt fragment is not a string");

    // 拼凑完整的提示词
    let full_prompt = format!("{} {} {}", prompt_fragment, template, filtered_html);

    // 调用 llm 函数
    let mut response = llm(&auth_token, &full_prompt);

    // 打印 LLM 返回的信息
    //println!("LLM Response:\n{}", response);

    // 检查并移除 ```markdown 和 ``` 行
    if response.starts_with("```markdown") && response.ends_with("```") {
        response = response.trim_start_matches("```markdown\n").trim_end_matches("```").to_string();
    }

    // 创建 output 目录（如果不存在）
    let output_dir = Path::new("output");
    if !output_dir.exists() {
        fs::create_dir_all(output_dir).expect("Unable to create output directory");
    }

    // 将返回的信息存储在工作目录下的 output 文件夹中
    let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let output_file_name = format!("output/{}-{}.md", url.replace("https://", "").replace("/", "-"), timestamp);
    let mut output_file = fs::File::create(output_file_name).expect("Unable to create output file");
    output_file.write_all(response.as_bytes()).expect("Unable to write to output file");
}

fn llm(auth_token: &str, prompt: &str) -> String {
    let auth = Auth::new(auth_token);
    let openai = OpenAI::new(auth, "https://api.deepseek.com/");
    let body = ChatBody {
        model: "deepseek-chat".to_string(),
        max_tokens: Some(4096),
        temperature: Some(0_f32),
        top_p: Some(0.9_f32),
        n: Some(1),
        stream: Some(false),
        stop: None,
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
        messages: vec![Message { role: Role::User, content: prompt.to_string() }],
    };
    let rs = openai.chat_completion_create(&body);
    match rs {
        Ok(response) => {
            let choice = response.choices;
            let message = &choice[0].message.as_ref().unwrap();
            message.content.clone()
        },
        Err(e) => {
            eprintln!("Error calling API: {:?}", e);
            String::new()
        }
    }
}
