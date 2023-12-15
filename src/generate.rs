use crate::constant::{GENERAL_CONTENT, DNS_CONTENT, OTHER_SETTING, FILTER_LOCAL};
use qx_conf_gen::gen_url;
use qx_conf_gen::append_lines;

fn generate_policy_info (rule_list: &Vec<&str>, node_names: String) -> Vec<String> {
  let mut policy_info_vec: Vec<String> = Vec::new();
  policy_info_vec.push(String::from("[policy]"));
  for rule in rule_list {
      let policy_info:String = format!("static={}, {}",rule, node_names);
      policy_info_vec.push(policy_info);
  }
  let auto_select = format!("url-latency-benchmark=自动选择, server-tag-regex=.*, check-interval=600, tolerance=100,");
  let direct_policy= format!("static=全球直连, proxy,direct,reject,自动选择,");
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


pub fn output_config_file_content(rule_list: Vec<&str>, node_names: String, node_list: String, path_is_url: bool, path: String ) {
  let mut contents_vec: Vec<String> = Vec::new();
  let general_content = Vec::from(GENERAL_CONTENT);
  contents_vec.extend(general_content.iter().map(|s| s.to_string()));

  let dns_content = Vec::from(DNS_CONTENT);
  contents_vec.extend( dns_content.iter().map(|&s| s.to_string()));

  let policy_info = generate_policy_info(
    &rule_list,
    node_names,
  );
  contents_vec.extend(policy_info);

  let filter_remote = generate_filter_remote(&rule_list);
  contents_vec.extend(filter_remote.iter().map(|s| s.to_string()));

  if !path_is_url {
    let mut node_list_info: Vec<&str> = vec![
      "[server_local]",
    ];

    let split_node_list:Vec<&str> = node_list.split("\n").collect();
    node_list_info.extend(split_node_list);
    contents_vec.extend(node_list_info.iter().map(|s| s.to_string()));
    contents_vec.push(String::from("[server_remote]"));
  } else {
    contents_vec.push(String::from("[server_remote]"));
    contents_vec.push(format!("{}, tag=test, update-interval=172800, opt-parser=false, enabled=false", path));
    contents_vec.push(String::from("[server_local]"));
  }
  
  contents_vec.extend(FILTER_LOCAL.iter().map(|s| s.to_string()));

  contents_vec.extend(OTHER_SETTING.iter().map(|s| s.to_string()));
  append_lines("qx.conf", contents_vec)

}

