use reqwest::blocking::Client;
use serde_json::Value;
use std::{env, path, thread, time};

const FAILURE_WAIT_TIME: u64 = 5; // 请求失败后等待的时间，单位：秒

fn main() {
    // 打印架构和系统
    let arch: &str = env::consts::ARCH;
    let system: &str = env::consts::OS;
    println!(
        "Running on \x1b[0;30;43m {}-{} \x1b[0m system",
        arch, system
    );

    // 打印可执行文件名
    println!("Reading the executable filename...");
    let filename = get_filename();
    println!("Executable filename: \x1b[0;30;47m{}\x1b[0m", filename);

    // 提取学号和密码
    let (id, passwd) = extract_id_and_password(&filename);
    println!("id: \x1b[0;37;45m{}\x1b[0m", id);
    println!("passwd: \x1b[0;30;46m{}\x1b[0m", passwd);

    let url_http: String = format!(
        "http://10.0.254.125:801/eportal/portal/login?&user_account={id}&user_password={passwd}"
    );
    let _url_https: String = format!(
        "https://auth.cqnu.edu.cn:802/eportal/portal/login?&user_account={id}&user_password={passwd}"
    );
    // println!("request url: \x1b[0;37;44m{}\x1b[0m", url_http);

    // Windows下 等待物理连接建立
    #[cfg(target_os = "windows")]
    {
        println!("Waiting for physical connection...");
        thread::sleep(time::Duration::from_secs(7));
    }

    let client = Client::builder()
        .no_proxy()
        .build()
        .unwrap();

    // 尝试请求10次，直到请求成功
    for _ in 0..10 {
        match client.get(&url_http).send() {
            Ok(response) => {
                println!(
                    "request status code: \x1b[0;37;42m{}\x1b[0m",
                    response.status()
                );
                if response.status() != 200 {
                    println!("Request failed. Retrying...");
                    thread::sleep(time::Duration::from_secs(FAILURE_WAIT_TIME));
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
                thread::sleep(time::Duration::from_secs(2));
                return; // 请求成功，退出程序
            }
            Err(err) => {
                println!("An error occurred during the request: {}", err);
                thread::sleep(time::Duration::from_secs(FAILURE_WAIT_TIME));
            }
        }
    }
    println!("Exceeded maximum number of attempts. Request failed.")
}

fn get_filename() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = path::Path::new(&args[0]) // 获取可执行文件名
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    // windows系统下，去掉.exe后缀
    #[cfg(target_os = "windows")]
    let filename = {
        path::Path::new(&filename)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
    };
    filename
}

fn extract_id_and_password(filename: &str) -> (&str, &str) {
    let mut parts = filename.split(';');
    let id = parts.next().expect(
        "\x1b[0;37;41m Please make sure your id and password are separated by ';'. \x1b[0m ",
    );
    let passwd = parts.next().expect(
        "\x1b[0;37;41m Please make sure your id and password are separated by ';'. \x1b[0m ",
    );
    if id.len() != 13 || !id.chars().all(char::is_numeric) {
        panic!("ID must be a 13-digit number.");
    }
    (id, passwd)
}

fn extract_json_data(response_text: &str) -> Value {
    let start_index = response_text.find('(').unwrap() + 1;
    let end_index = response_text.rfind(')').unwrap();
    let json_data = &response_text[start_index..end_index];

    match serde_json::from_str(json_data) {
        Ok(data) => data,
        Err(err) => panic!(
            "Failed to parse JSON: {},\n Please contact the developer with a bug report",
            err
        ),
    }
}
