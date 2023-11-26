mod generate;
mod constant;

use qx_conf_gen::{
    read_node_list,
    read_io_input,
    get_node_names,
    init_conf,
};
use generate::generate;
use url::Url;

#[tokio::main]
async fn main() {

    init_conf();

    let mut path: String = match read_io_input(
        vec![
            String::from("如要退出，请按ESC"),
            String::from("如果直接按下回车，将采取默认路径NodeList.snippist"),
        ],
        "请输入节点列表文件路径，并按回车键确认:",
        "您输入的节点列表路径为:",

        true
    ) {
        Ok(value) => value,
        Err(_) => String::from("NodeList.snippist"), // 默认值
    };

    if path.len() == 0 {
        path = String::from("NodeList.snippist")
    }


    let path_is_url = match Url::parse(path.as_ref()) {
        Ok(url) => url.scheme() == "http" || url.scheme() == "https",
        Err(_) => false,
    };

    let node_list = read_node_list(&path, path_is_url).await;

    if path.clone().len() == 0 {
        path = String::from ("NodeList.snippist")
    }

    let node_names = get_node_names(node_list.clone());


    let rules = match read_io_input(
        vec![
            String::from("您可以在这里查看已有的规则:"),
            String::from("https://github.com/blackmatrix7/ios_rule_script/tree/master/rule/QuantumultX"),
            String::from("如果需要添加多个规则，请用逗号分隔，不区分圆角半角")
        ],
        "请输入你需要添加的规则:",
        "您输入的规则名称路径为:",
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
            if trim_rule.len() > 0 {
                let trim_rule = rule.trim();
                rule_list.push(trim_rule)
            }
        }
    }

    println!("rule list is {:?}", rule_list);
    
    generate(rule_list, node_names, node_list.clone(), path_is_url, path);
}
