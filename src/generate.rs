use crate::constant::{GENERAL_CONTENT, DNS_CONTENT, OTHER_SETTING, FILTER_LOCAL};
use qx_conf_gen::append_line_to_file;
use qx_conf_gen::gen_url;
use qx_conf_gen::append_lines;

fn generate_policy_info (rule_list: &Vec<&str>, node_names: String) -> Vec<String> {
  println!("nodenames is {}", node_names);
  let mut policy_info_vec: Vec<String> = Vec::new();
  policy_info_vec.push(String::from("[policy]"));
  for rule in rule_list {
      let policy_info:String = format!("static={}, {}",rule, node_names);
      policy_info_vec.push(policy_info);
  }
  let auto_select = format!("url-latency-benchmark=自动选择,{}", node_names.replace("direct, reject, proxy, 自动选择,", ""));
  let direct_policy= format!("static=全球直连, direct,reject,proxy");
  let omissive = format!("static=漏网之鱼, {}", node_names);
  policy_info_vec.push(auto_select);
  policy_info_vec.push(direct_policy);
  policy_info_vec.push(omissive);
  return policy_info_vec
}

fn generate_filter_remote (rule_list: &Vec<&str>) -> Vec<String> {
  let mut filter_remote_vec: Vec<String> = Vec::new();
  filter_remote_vec.push(String::from("[filter_remote]"));
  for rule in rule_list {
    let url = gen_url(rule.to_string());
    let policy_info:String = format!("{}, tag={}, update-interval=172800, opt-parser=false, enabled=true", url, rule);
    filter_remote_vec.push(policy_info);
  }
  return filter_remote_vec;
}


pub fn generate(rule_list: Vec<&str>, node_names: String, node_list: String, path_is_url: bool, path: String ) {
  let general_content = Vec::from(GENERAL_CONTENT);
  append_lines("qx.conf", general_content);

  let dns_content = Vec::from(DNS_CONTENT);
  append_lines("qx.conf", dns_content);

  let policy_info = generate_policy_info(
    &rule_list,
    node_names,
  );
  append_lines("qx.conf", policy_info);

  let filter_remote = generate_filter_remote(&rule_list);
  append_lines("qx.conf", filter_remote);

  if !path_is_url {
    let mut node_list_info: Vec<&str> = vec![
      "[server_local]",
    ];

    let split_node_list:Vec<&str> = node_list.split("\n").collect();
    node_list_info.extend(split_node_list);
    append_lines("qx.conf", node_list_info);
  } else {
    append_line_to_file("qx.conf", "[server_remote]").unwrap();
    append_line_to_file(
      "qx.conf", 
      &format!("{}, tag=test, update-interval=172800, opt-parser=false, enabled=false", path),
    ).unwrap();
  }
  
  let filter_local = Vec::from(FILTER_LOCAL);
  append_lines("qx.conf", filter_local);

  let other_setting = Vec::from(OTHER_SETTING);
  append_lines("qx.conf", other_setting);
}


