use reqwest::blocking;
use serde_json::Value;
use std::{env, path, thread, time};

fn main() {
    let arch: &str = env::consts::ARCH;
    let system: &str = env::consts::OS;
    println!("Running on \x1b[0;30;43m {}-{} \x1b[0m system", arch, system); // 打印架构和系统
    
    println!("Reading the executable filename...");
    let filename = get_filename(&system);
    println!("Executable filename: \x1b[0;30;47m{}\x1b[0m", filename); // 打印可执行文件名

    let (id, passwd) = extract_id_and_password(&filename); // 提取学号和密码
    println!("id: \x1b[0;37;45m{}\x1b[0m", id);
    println!("passwd: \x1b[0;30;46m{}\x1b[0m", passwd);

    let url_login: String = format!(
        "http://10.0.254.125:801/eportal/portal/login?&user_account={id}&user_password={passwd}"
    );
    println!("request url: \x1b[0;37;44m{}\x1b[0m", url_login);

    // 尝试请求10次，直到请求成功
    for _ in 0..10 {
        match blocking::get(&url_login) {
            Ok(response) => {
                println!(
                    "request status code: \x1b[0;37;42m{}\x1b[0m",
                    response.status()
                );
                if response.status() != 200 {
                    println!("Request failed. Retrying...");
                    thread::sleep(time::Duration::from_secs(3));
                    continue;
                } else {
                    println!("Request succeeded. Congratulations!");
                }

                let response_text = response.text().unwrap();
                let data: Value = extract_json_data(&response_text);
                println!(
                    "json.msg: \x1b[0;37;41m{}\x1b[0m",
                    data["msg"].as_str().unwrap()
                );
                return; // 请求成功，退出程序
            }
            Err(err) => {
                println!("An error occurred during the request: {}", err);
                thread::sleep(time::Duration::from_secs(3));
            }
        }
    }
    println!("Exceeded maximum number of attempts. Request failed.")
}

fn get_filename(system: &str) -> String {
    let args: Vec<String> = env::args().collect();
    let filename = path::Path::new(&args[0]) // 获取可执行文件名
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    // windows系统下，去掉.exe后缀
    let filename = if system == "windows" {
        path::Path::new(&filename)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
    } else {
        filename
    };
    filename
}

fn extract_id_and_password(filename: &str) -> (&str, &str) {
    let mut parts = filename.split(';');
    let id = parts.next().unwrap();
    let passwd = parts.next().unwrap();
    if id.len() != 13 || !id.chars().all(char::is_numeric) {
        panic!("ID must be a 13-digit number.");
    }
    (id, passwd)
}

fn extract_json_data(response_text: &str) -> Value {
    let start_index = response_text.find('(').unwrap() + 1;
    let end_index = response_text.rfind(')').unwrap();
    let json_data = &response_text[start_index..end_index];

    serde_json::from_str(json_data).unwrap()
}
