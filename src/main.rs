mod clash_to_qx;
mod constant;
mod generate;

use clash_to_qx::{fetch_clash_conf, format_proxies, parse_proxies_from_yaml};
use generate::output_config_file_content;
use qx_conf_gen::{get_node_names, init_conf, read_io_input};
use std::env;

#[tokio::main]
async fn main() {
    init_conf();

    let args: Vec<String> = env::args().collect();

    for arg in args.iter() {
        println!("{}", arg);
    }

    let mut path = String::new();
    let mut rules = String::new();

    // 遍历参数
    for arg in args.iter() {
        // 检查参数是否以 "--path=" 开头
        if arg.starts_with("--path=") {
            path = arg
                .trim_start_matches("--path=")
                .trim_matches('"') // 移除可能的双引号
                .to_string();
        } else if arg.starts_with("--rules=") {
            rules = arg
                .trim_start_matches("--rule=")
                .trim_matches('"') // 移除可能的双引号
                .to_string();
        }
    }

    if path.len() == 0 {
        path = match read_io_input(
            vec![
                String::from("如要退出，请按ESC"),
                // String::from("如果直接按下回车，将采取默认路径old.conf"),
            ],
            "请输入配置文件路径，并按回车键确认:",
            "",
            true,
        ) {
            Ok(value) => value,
            Err(_) => String::from("old.conf"), // 默认值
        };
    }

    if path.len() == 0 {
        path = String::from("old.conf")
    }
    let conf = fetch_clash_conf(path.clone()).await;
    let proxies = parse_proxies_from_yaml(conf);
    let node_list = format_proxies(proxies).join("\n");
    println!("{}", node_list);

    // if path.clone().len() == 0 {
    //     path = String::from ("old.conf")
    // }

    let node_names = get_node_names(node_list.clone());

    if rules.len() == 0 {
        rules = match read_io_input(
            vec![
                String::from("您可以在这里查看已有的规则:"),
                String::from(
                    "https://github.com/blackmatrix7/ios_rule_script/tree/master/rule/QuantumultX",
                ),
                String::from("如果需要添加多个规则，请用逗号分隔，不区分圆角半角"),
            ],
            "请输入你需要添加的规则:",
            "您输入的规则名称路径为:",
            false,
        ) {
            Ok(value) => value,
            Err(_) => String::from("old.conf"), // 默认值
        };
    }

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

    output_config_file_content(rule_list, node_names, node_list.clone());
}
