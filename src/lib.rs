use crossterm::{
    cursor::MoveLeft, // ExecutableCommand
    event::{
        self, KeyCode, KeyEvent,
    },
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen},
};
use futures::future::join_all;
use reqwest::{self};
use std::{
    io::{self, stdout, Read, Write},
    fs::{self, OpenOptions, remove_file, File},
    future::Future,
};
use regex::Regex;

pub fn get_node_name_from_node(node_info: String) -> String {
     // 创建一个正则表达式来匹配 tag= 后面的内容
    let re = Regex::new(r"tag=([^,]+)").unwrap();
    
     // 查找匹配并打印结果
    if let Some(caps) = re.captures(&node_info) {
        if let Some(matched) = caps.get(1) {
            return matched.as_str().to_string()
        } else {
            return String::new();
        }
    } else {
        return String::new();
    }
}

pub fn get_node_names(node_list: String) -> String{
    let node_list_vec: Vec<&str> = node_list.split("\n").collect();
    let mut node_names = String::from("proxy, direct, reject,");
    for node in node_list_vec {
        let mut name = get_node_name_from_node(node.to_string());
        println!("node name is {}", name);
        if(name.len() > 0) {
            node_names.push_str(&name);
            node_names.push_str(",");
        }
    }
    return node_names
}

pub  fn read_node_list<T: AsRef<str>>(path: T) -> io::Result<String>  {
    let mut file = fs::File::open(path.as_ref())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub async fn  read_node_list_new<T: AsRef<str>>(path: T, is_url: bool) -> String {
    let mut node_list = String::from("");
    println!("is url? {}", is_url);
    if !is_url {
        let mut file = fs::File::open(path.as_ref()).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        node_list = contents
    } else {
        let client = reqwest::Client::new();
        let contents =  match client.get(path.as_ref()).send().await {
            Ok(resp) => {
                let res: String = match resp.text().await {
                    Ok(str) => str,
                    Err(_)=> String:: from(""),
                };
                res
            },
            Err(_) => String:: from(""),
        };
        println!("contents is {}", contents);
        node_list = contents;
    }
    return node_list;
}

pub fn read_io_input(help_text: Vec<String>, prefix_text: String, need_clear_all: bool) -> std::io::Result<String> {
    let mut node_list_path = String::new();
    if need_clear_all {
        execute!(
            stdout(),
            Clear(ClearType::All),
        )?;
    }
    for text in help_text {
        execute!(
            stdout(),
            EnterAlternateScreen,
            SetForegroundColor(Color::White),
            SetBackgroundColor(Color::Blue),
            Print(text),
            MoveLeft(100),
        )?;
        println!("");
    }
    execute!(
        stdout(),
        EnterAlternateScreen,
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Green),
        Print(&prefix_text),
        ResetColor,
    )?;
    // terminal::enable_raw_mode()?;
    loop {
        if let event::Event::Key(KeyEvent {
            code, modifiers:_, ..
        }) = event::read()?
        {
            match code {
                KeyCode::Char(c) => {
                    node_list_path.push(c);
                    let offset = node_list_path.len();
                    let text = (&prefix_text).to_string() + &node_list_path;
                    execute!(
                        stdout(),
                        MoveLeft((40 + offset).try_into().unwrap()),
                        Clear(ClearType::CurrentLine),
                        EnterAlternateScreen,
                        SetForegroundColor(Color::White),
                        SetBackgroundColor(Color::Green),
                        Print(text),
                        ResetColor,
                    )?;
                }
                KeyCode::Backspace => {
                    node_list_path.pop();
                    println!("{}{}", &prefix_text, node_list_path);
                }
                KeyCode::Enter => {
                    println!("");
                    let text = (String::from("您输入的节点列表文件路径为:")) + &node_list_path;
                    execute!(
                        stdout(),
                        MoveLeft((100).try_into().unwrap()),
                        Clear(ClearType::CurrentLine),
                        EnterAlternateScreen,
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
    // disable_raw_mode();
    println!("");
    Ok(node_list_path)
}

pub fn init_conf() {
    let file_path = "path/to/your/file.txt";
    remove_file(file_path).unwrap();
    File::create (file_path);
}

pub fn append_line_to_file(file_path: &str, content:  &str) -> io::Result<()> {
    // 打开文件，准备追加内容
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path)?;

    // 将内容写入文件
    writeln!(file, "{}", content)?;

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
pub async fn fetch_resource(rule_name: &str) -> impl Future<Output = Result<reqwest::Response, reqwest::Error>> {
    let url = gen_url(rule_name.to_string());
    let client = reqwest::Client::new();
    client.get(url).send()
}



pub  async fn fetch_rules(rule_list: Vec<&str>) -> Vec<String> {
    println!("----fetch_rules---start----");
    let mut rule_text_list: Vec<String> = Vec::new();
    let fetch_futures: Vec<_> = rule_list.iter()
        .map(|url| fetch_resource(url))
        .collect();
    let request_list = join_all(fetch_futures).await;
    for request in request_list {
        let a = request.await;
        let rule_text:String = match a {
            Ok(resp) => {
                let res: String = match resp.text().await {
                    Ok(str) => str,
                    Err(_)=> String:: from(""),
                };
                res
            },
            Err(_) => String:: from(""),
        };
        println!("{}", rule_text);
        rule_text_list.push(rule_text);
    }
    println!("----fetch_rules---end----");
    rule_text_list
}