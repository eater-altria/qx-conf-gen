use serde::{Deserialize, Serialize};// use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginOpts {
    mode: Option<String>,
    host: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Proxy {
    #[serde(rename = "type")]
    pub proxy_type: String,
    pub server: String,
    pub port:String,
    pub cipher: Option<String>,
    #[serde(rename = "plugin-opts")]
    pub plugin_opts: Option<PluginOpts>,
    pub udp: Option<bool>,
    pub name: String,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClashConfig {
    pub proxies: Vec<Proxy>,
}


pub async fn fetch_clash_conf(url: String) -> String {
  // 发送 GET 请求
  let response = reqwest::get(&url).await.unwrap();
  
  // 检查请求是否成功
  if response.status().is_success() {
      // 将响应体转换为字符串
      let body = response.text().await.unwrap();
      body
  } else {
      // 如果请求失败，返回错误
      String::new()
  }
}

pub fn parse_proxies_from_yaml(yaml_content: String) -> Vec<Proxy> {
  // 解析 YAML 内容为 ClashConfig 结构体
  let config: ClashConfig = serde_yaml::from_str(&yaml_content).unwrap();
  // 返回 proxies 字段
  let valid_proxies: Vec<Proxy> = config
      .proxies
      .into_iter()
      .filter(|proxy| !proxy.cipher.is_none() && !proxy.server.is_empty())
      .collect();
  valid_proxies
}

fn is_emoji(c: char) -> bool {
  // Unicode 中 emoji 的范围
  match c as u32 {
      // 基本 emoji 范围
      0x1F600..=0x1F64F => true, // Emoticons
      0x1F300..=0x1F5FF => true, // Miscellaneous Symbols and Pictographs
      0x1F680..=0x1F6FF => true, // Transport and Map Symbols
      0x1F700..=0x1F77F => true, // Alchemical Symbols
      0x1F780..=0x1F7FF => true, // Geometric Shapes Extended
      0x1F800..=0x1F8FF => true, // Supplemental Arrows-C
      0x1F900..=0x1F9FF => true, // Supplemental Symbols and Pictographs
      0x1FA00..=0x1FA6F => true, // Chess Symbols
      0x1FA70..=0x1FAFF => true, // Symbols and Pictographs Extended-A
      0x2600..=0x26FF   => true, // Miscellaneous Symbols
      0x2700..=0x27BF   => true, // Dingbats
      0xFE00..=0xFE0F   => true, // Variation Selectors
      0x1F1E6..=0x1F1FF => true, // Regional Indicator Symbols
      _ => false,
  }
}

fn filter_emoji(input: String) -> String {
  input.chars() // 将字符串转换为字符迭代器
      .filter(|&c| !is_emoji(c)) // 过滤掉 emoji 字符
      .collect() // 将字符重新组合成字符串
}

pub fn format_proxies(proxies: Vec<Proxy>) -> Vec<String> {
  proxies
      .into_iter()
      .map(|proxy| {
          let mut parts = Vec::new();

          // 添加 type 和 server
          parts.push(format!("{} = {}:{}", if proxy.proxy_type == "ss" {"shadowsocks"} else {"unknown"}, proxy.server, proxy.port));

          // 添加 method (cipher)
          parts.push(format!("method = {}", proxy.cipher.unwrap()));

          // 添加 password (假设 password 是 Proxy 结构体的一个字段)
          // 如果 Proxy 结构体中没有 password 字段，可以忽略这部分
          parts.push(format!("password = {}", proxy.password.unwrap()));

          // 添加 plugin-opts 相关字段
          if let Some(plugin_opts) = proxy.plugin_opts {
              if let Some(mode) = plugin_opts.mode {
                  parts.push(format!("obfs = {}", mode));
              }
              if let Some(host) = plugin_opts.host {
                  parts.push(format!("obfs-host = {}", host));
              }
          }

          // 添加 udp
          if let Some(udp) = proxy.udp {
              parts.push(format!("udp-relay = {}", udp));
          }

          // 添加 name (tag)
          parts.push(format!("tag = {}", filter_emoji(proxy.name)));

          // 将部分拼接成完整字符串
          parts.join(", ")
      })
      .collect()
}
