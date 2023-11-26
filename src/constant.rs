pub const GENERAL_CONTENT:[&str; 5] = [
  "[general]",
  "excluded_routes=192.168.0.0/16, 172.16.0.0/12, 100.64.0.0/10, 10.0.0.0/8",
  "geo_location_checker=http://ip-api.com/json/?lang=zh-CN, https://github.com/KOP-XIAO/QuantumultX/raw/master/Scripts/IP_API.js",
  "network_check_url=http://www.baidu.com/",
  "server_check_url=http://www.gstatic.com/generate_204",
];

pub const DNS_CONTENT:[&str; 5] = [
  "[dns]",
  "server=119.29.29.29",
  "server=223.5.5.5",
  "server=1.0.0.1",
  "server=8.8.8.8",
];

pub const FILTER_LOCAL:[&str; 3] = [
  "[filter_local]",
  "GEOIP,CN,全球直连",
  "FINAL,漏网之鱼",
];

pub const OTHER_SETTING_1: [&str;4] = [
  "[server_remote]",
  "[rewrite_remote]",
  "[rewrite_local]",
  "[mitm]"
];

pub const OTHER_SETTING_2: [&str;4] = [
  "[server_local]",
  "[rewrite_remote]",
  "[rewrite_local]",
  "[mitm]"
];