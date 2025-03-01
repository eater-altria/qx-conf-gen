use crossterm::{
    cursor::MoveLeft, // ExecutableCommand
    event::{self, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use futures::future::join_all;
use regex::Regex;
use reqwest::{self};
use std::{
    collections::HashMap,
    fs::{self, remove_file, OpenOptions},
    future::Future,
    io::{self, stdout, Read, Write},
    path,
};

pub fn init_conf() {
    let path = "qx.conf";
    if path::Path::new(path).exists() {
        remove_file(path).unwrap()
    }
}

pub fn get_node_name_from_node(node_info: String) -> String {
    // 创建一个正则表达式来匹配 tag= 后面的内容
    let re = Regex::new(r"tag=([^,]+)").unwrap();

    // 查找匹配并打印结果
    if let Some(caps) = re.captures(&node_info) {
        if let Some(matched) = caps.get(1) {
            return matched.as_str().to_string();
        } else {
            return String::new();
        }
    } else {
        return String::new();
    }
}

pub fn get_node_names(node_list: String) -> String {
    let node_list_vec: Vec<&str> = node_list.split("\n").collect();
    let mut node_names = String::from("proxy,direct,reject,自动选择,");
    for node in node_list_vec {
        let name = get_node_name_from_node(node.to_string());
        if name.len() > 0 {
            node_names.push_str(&name);
            node_names.push_str(",");
        }
    }
    return node_names;
}

pub async fn read_node_list<T: AsRef<str>>(path: T, is_url: bool) -> String {
    let node_list: String = if !is_url {
        let mut file = fs::File::open(path.as_ref()).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    } else {
        let client = reqwest::Client::new();
        let contents = match client.get(path.as_ref()).send().await {
            Ok(resp) => {
                let res: String = match resp.text().await {
                    Ok(str) => str,
                    Err(_) => String::from(""),
                };
                res
            }
            Err(_) => String::from(""),
        };
        contents
    };
    return node_list;
}

pub async fn read_config_file<T: AsRef<str>>(path: T, is_url: bool) -> String {
    let node_list: String = if !is_url {
        let mut file = fs::File::open(path.as_ref()).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    } else {
        let client = reqwest::Client::new();
        let contents = match client.get(path.as_ref()).send().await {
            Ok(resp) => {
                let res: String = match resp.text().await {
                    Ok(str) => str,
                    Err(_) => String::from(""),
                };
                res
            }
            Err(_) => String::from(""),
        };
        contents
    };
    return node_list;
}

pub fn parse_config(contents: &str) -> HashMap<String, String> {
    let mut config = HashMap::new();
    let mut current_key = String::new();
    let mut current_value = String::new();

    for line in contents.lines() {
        if line.starts_with('[') {
            if !current_key.is_empty() {
                config.insert(current_key.clone(), current_value.trim().to_string());
            }
            current_key = line.trim_matches(|c| c == '[' || c == ']').to_string();
            current_value = String::new();
        } else {
            current_value.push_str(line);
            current_value.push('\n');
        }
    }

    if !current_key.is_empty() {
        config.insert(current_key, current_value.trim().to_string());
    }

    config
}

pub fn read_io_input(
    help_text: Vec<String>,
    prefix_text: &str,
    suffix_text: &str,
    need_clear_all: bool,
) -> std::io::Result<String> {
    enable_raw_mode().unwrap();
    let mut node_list_path = String::new();
    if need_clear_all {
        // execute!(stdout(), Clear(ClearType::All),)?;
    }
    for text in &help_text {
        execute!(
            stdout(),
            MoveLeft(100),
            SetForegroundColor(Color::White),
            SetBackgroundColor(Color::Green),
            Print(text),
            ResetColor,
        )?;
        println!("");
    }

    execute!(
        stdout(),
        MoveLeft(100),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Green),
        Print(&prefix_text),
        ResetColor,
    )?;
    loop {
        if let event::Event::Key(KeyEvent {
            code,
            modifiers: _,
            kind,
            ..
        }) = event::read()?
        {
            if kind == event::KeyEventKind::Release {
                continue; // 忽略释放按键的事件
            }
            match code {
                KeyCode::Char(c) => {
                    node_list_path.push(c);
                    let offset = node_list_path.len();
                    let text = (&prefix_text).to_string() + &node_list_path;
                    execute!(
                        stdout(),
                        MoveLeft((40 + offset).try_into().unwrap()),
                        // Clear(ClearType::UntilNewLine),
                        SetForegroundColor(Color::White),
                        SetBackgroundColor(Color::Green),
                        Print(text),
                        ResetColor,
                    )?;
                }
                KeyCode::Backspace => {
                    node_list_path.pop();
                    let offset = node_list_path.len();
                    let text = (&prefix_text).to_string() + &node_list_path;
                    execute!(
                        stdout(),
                        MoveLeft((40 + offset).try_into().unwrap()),
                        Clear(ClearType::CurrentLine),
                        SetForegroundColor(Color::White),
                        SetBackgroundColor(Color::Green),
                        Print(text),
                        ResetColor,
                    )?;
                }
                KeyCode::Enter => {
                    println!("");
                    let text = (String::from(suffix_text)) + &node_list_path;
                    execute!(
                        stdout(),
                        MoveLeft((100).try_into().unwrap()),
                        // Clear(ClearType::CurrentLine),
                        SetForegroundColor(Color::White),
                        SetBackgroundColor(Color::Green),
                        Print(text),
                        ResetColor,
                    )?;
                    break;
                }
                KeyCode::Esc => break,
                _ => continue,
            }
        }
    }
    disable_raw_mode().unwrap();
    println!("");
    // enable_raw_mode().unwrap();
    Ok(node_list_path)
}

pub fn append_line_to_file(file_path: &str, content: &str) -> io::Result<()> {
    // 使用OpenOptions打开文件，设置为可写、可追加和可创建
    // 如果文件不存在，那么会创建一个新文件
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path)?; // 使用问号运算符，如果出现错误，会立即返回错误

    // 使用writeln!宏将内容写入文件，它会自动添加换行符
    // 如果写入失败，会立即返回错误
    writeln!(file, "{}", content)?;

    // 如果以上所有操作都成功，那么返回Ok(())
    Ok(())
}

pub fn append_lines<T: AsRef<str>>(file_path: &str, contents: Vec<T>) {
    for content in contents {
        append_line_to_file(file_path, content.as_ref()).unwrap();
    }
}

pub fn gen_url(rule_name: String) -> String {
    return format!(
        "https://raw.githubusercontent.com/blackmatrix7/ios_rule_script/master/rule/QuantumultX/{}/{}.list",
        rule_name,
        rule_name,
    );
}

// 异步函数以获取资源
pub async fn fetch_resource(
    rule_name: &str,
) -> impl Future<Output = Result<reqwest::Response, reqwest::Error>> {
    let url = gen_url(rule_name.to_string());
    let client = reqwest::Client::new();
    client.get(url).send()
}

pub async fn fetch_rules(rule_list: Vec<&str>) -> Vec<String> {
    let mut rule_text_list: Vec<String> = Vec::new();
    let fetch_futures: Vec<_> = rule_list.iter().map(|url| fetch_resource(url)).collect();
    let request_list = join_all(fetch_futures).await;
    for request in request_list {
        let a = request.await;
        let rule_text: String = match a {
            Ok(resp) => {
                let res: String = match resp.text().await {
                    Ok(str) => str,
                    Err(_) => String::from(""),
                };
                res
            }
            Err(_) => String::from(""),
        };
        rule_text_list.push(rule_text);
    }
    rule_text_list
}
