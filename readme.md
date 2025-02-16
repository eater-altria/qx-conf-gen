# Quantumult X 配置生成器

**这是一个个人用Rust开发的玩具项目，欢迎PR，欢迎Star**

## 简单介绍
这是一个我自己用的配置生成器，它具备如下的功能：
- 将已有的clash订阅地址，和规则分流资源组合起来，形成一个配置文件
- 在配置文件当中，每一个分流都可以单独选择全量的节点
- 虽然这样让每一个分流变的巨长无比，但是单独控制真的很重要！

## 简单使用
这是一个repl程序，内部有中文的提示
需要注意的是，规则名称目前只支持[这里](https://github.com/blackmatrix7/ios_rule_script/tree/master/rule/QuantumultX)收录的，输入规则名称时候一定要记得和这里保持一致，大小写也一样哦。

同时它也支持纯命令行模式，想体验的话可以运行：
```sh
cargo run --  --path=https://sub.com --rules=Apple,Telegram,YouTube,Netflix,OpenAI,Google,Microsoft,BiliBili,Game
```
运行之后会生成一个 qx.conf，复制到 qx 软件当中就好了

