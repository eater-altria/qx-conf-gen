# Quantumult X 配置生成器

**这是一个个人用Rust开发的玩具项目，欢迎PR，欢迎Star**

## 简单介绍
这是一个我自己用的配置生成器，它具备如下的功能：
- 将已有的节点信息，和规则分流资源组合起来，形成一个配置文件
- 在配置文件当中，每一个分流都可以单独选择全量的节点
- 虽然这样让每一个分流变的巨长无比，但是单独控制真的很重要！
- 另外节点支持网络下载，也支持本地路径

## 简单使用
这是一个repl程序，内部有中文的提示
需要注意的是，规则名称目前只支持[这里](https://github.com/blackmatrix7/ios_rule_script/tree/master/rule/QuantumultX)收录的，输入规则名称时候一定要记得和这里保持一致，大小写也一样哦。

## 未来计划
1. 增加命令行模式
2. 支持从完整的配置文件当中解析出所有节点，而非使用现成的节点片段。
3. 支持从其他工具的配置文件解析节点（这会是一个大工程，也许我会考虑用前人的轮子）