use reqwest::Error;
use serde_json::Value;
use std::{env, env::consts};

fn main() {
    println!("Reading the executable filename...");
    let arch = consts::ARCH;
    let system = consts::OS;
    println!("Running on \x1b[5;30;43m {}-{} \x1b[0m OS",arch, system);

    let filename = get_filename(&system);
    println!("Executable filename: \x1b[0;30;47m{}\x1b[0m", filename);

    let (id, passwd) = extract_id_and_password(&filename);
    println!("id: \x1b[0;37;45m{}\x1b[0m", id);
    println!("passwd: \x1b[0;30;46m{}\x1b[0m", passwd);

    let url_login = format!(
        "http://10.0.254.125:801/eportal/portal/login?&user_account={id}&user_password={passwd}"
    );
    println!("request url: \x1b[0;37;44m{}\x1b[0m", url_login);

    for _ in 0..10 {
        match make_request(&url_login) {
            Ok(response) => {
                let flag = response.status();
                println!("request status code: \x1b[0;37;42m {} \x1b[0m", flag);

                let response_text = response.text().unwrap();
                println!("response text: {}", response_text);
                let start_index = response_text.find('(').unwrap() + 1;
                let end_index = response_text.rfind(')').unwrap();
                let json_data = &response_text[start_index..end_index];
                let data: Value = serde_json::from_str(json_data).unwrap();
                println!("json.msg: \x1b[0;37;41m{}\x1b[0m", data["msg"].as_str().unwrap());

                if flag.is_success() {
                    break;
                }
            }
            Err(err) => {
                println!("An error occurred during the request: {err}");
                std::thread::sleep(std::time::Duration::from_secs(3));
            }
        }
    }
}

fn get_filename(system: &str) -> String {
    let args: Vec<String> = env::args().collect();
    let filename = std::path::Path::new(&args[0])
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    let filename = if system == "windows" {
        std::path::Path::new(&filename)
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

fn make_request(url: &str) -> Result<reqwest::blocking::Response, Error> {
    reqwest::blocking::get(url)
}
