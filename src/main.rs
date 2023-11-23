use qx_conf_gen::{read_node_list, read_io_input, fetch_rules};


#[tokio::main]
async fn main() {
    let path = match read_io_input(
        vec![
            String::from("如要退出，请按Ctrl + C"),
            String::from("如果直接按下回车，将采取默认路径NodeList.snippist"),
        ],
        String::from("请输入节点列表文件路径，并按回车键确认:"),
        true
    ) {
        Ok(value) => value,
        Err(_) => String::from("NodeList.snippist"), // 默认值
    };
    
    let node_list_path = match read_node_list(path) {
        Ok(value) => value,
        Err(_) => String::from(""), // 默认值
    };
    

    let rules = match read_io_input(
        vec![
            String::from("您可以在这里查看已有的规则:"),
            String::from("https://github.com/blackmatrix7/ios_rule_script/tree/master/rule/QuantumultX"),
            String::from("如果需要添加多个规则，请用逗号分隔，不区分圆角半角")
        ],
        String::from("请输入你需要添加的规则:"),
        false
    ) {
        Ok(value) => value,
        Err(_) => String::from("NodeList.snippist"), // 默认值
    };

    let rule_list_first: Vec<&str> = rules.split(',').collect();
    let mut rule_list: Vec<&str> = Vec::new();
    for rule in rule_list_first {
        let rule_list_second: Vec<&str> = rule.split('，').collect();
        for rule in rule_list_second {
            let trim_rule = rule.trim();
            if(trim_rule.len() > 0) {
                let trim_rule = rule.trim();
                rule_list.push(trim_rule)
            }
        }
    }

    
    println!("rule list is {:?}", rule_list);

    let rule_text_list = fetch_rules(rule_list).await;
}


// use reqwest;
// use std::error::Error;

// // 异步函数以获取资源
// async fn fetch_resource(url: &str) -> Result<String, Box<dyn Error>> {
//     let response = reqwest::get(url).await?;

//     if response.status().is_success() {
//         let contents = response.text().await?;
//         Ok(contents)
//     } else {
//         Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Request failed")))
//     }
// }

// #[tokio::main]
// async fn main() {
//     let url1 = "
//     https://raw.githubusercontent.com/blackmatrix7/ios_rule_script/master/rule/QuantumultX/AppleTV/AppleTV.list"; // 替换为你想访问的 URL
//     let url2 = "
//     https://raw.githubusercontent.com/blackmatrix7/ios_rule_script/master/rule/QuantumultX/BBC/BBC.list"; // 替换为你想访问的 URL
//     let url3 = "
//     https://raw.githubusercontent.com/blackmatrix7/ios_rule_script/master/rule/QuantumultX/BiliBili/BiliBili.list"; // 替换为你想访问的 URL  
//     match fetch_resource(url1).await {
//         Ok(contents) => println!("1 is done"),
//         Err(e) => println!("Error fetching resource: {}", e),
//     }
//     match fetch_resource(url2).await {
//         Ok(contents) => println!("2 is done"),
//         Err(e) => println!("Error fetching resource: {}", e),
//     }
//     match fetch_resource(url3).await {
//         Ok(contents) => println!("3 is done"),
//         Err(e) => println!("Error fetching resource: {}", e),
//     }
//     match fetch_resource(url1).await {
//         Ok(contents) => println!("4 is done"),
//         Err(e) => println!("Error fetching resource: {}", e),
//     }
// }