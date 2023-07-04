use reqwest::blocking::get;
use serde_json::Value;
use std::env;
use std::path::Path;

fn main() {
    println!("Reading the executable filename...");
    let os: &str = std::env::consts::OS; // 读取当前系统名称
    println!("Running on {os} OS"); // 打印当前系统名称

    // 读取当前可执行文件名称
    let args: Vec<String> = env::args().collect();
    let filename: String = Path::new(&args[0])
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    println!("Executable filename: {filename}"); // 打印当前可执行文件名称

    let mut parts = filename.split(';');

    let id: &str = parts.next().unwrap();
    let passwd: &str = parts.next().unwrap();

    println!("id: {id}, passwd: {passwd}");

    let url_login = format!(
        "http://10.0.254.125:801/eportal/portal/login?&user_account={id}&user_password={passwd}"
    );

    let response = get(&url_login).unwrap(); // 发送请求

    let body = response.text().unwrap(); // 获取响应内容
    let json_data = body
        .trim_start_matches("jsonpReturn(")
        .trim_end_matches(");"); // 去除多余的字符
    let parsed_data: Value = serde_json::from_str(json_data).unwrap(); // 解析 JSON 数据
    let msg_value = parsed_data["msg"].as_str().unwrap_or(""); // 获取 msg 字段的值

    println!("json msg: {msg_value}");
}
