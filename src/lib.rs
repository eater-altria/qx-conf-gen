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
    io::{self, stdout, Read},
    fs,
    future::Future,
};



pub fn read_node_list(path: String) -> io::Result<String>  {
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
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
    if node_list_path.len() == 0 {
        return Ok(String::from("NodeList.snippist"));
    }
    Ok(node_list_path)
}

// 异步函数以获取资源
pub async fn fetch_resource(rule_name: &str) -> impl Future<Output = Result<reqwest::Response, reqwest::Error>> {
    let url = format!(
        "https://raw.githubusercontent.com/blackmatrix7/ios_rule_script/master/rule/QuantumultX/{}/{}.list",
        rule_name,
        rule_name,
    );
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