# qqbot-rust-sdk
基于 [QQ官方机器人文档](https://bot.q.qq.com/wiki/develop/api-v2/) 中声明的事件数据模型、API及相关所需要的工具类所开发的QQ官方机器人，仅提供了事件类型、数据模型、QQ官方API的定义等。

目前是围绕着Webhook的需要去开发的，关于在Websocket连接方式仅有的一些数据可能有缺失，后面会慢慢补上。

根据自己的需要添加对应的Feature，默认则是全部。

## Features

### events
提供 Webhook / Websocket 回调后接受到的全部事件模型。
### openapi
提供根据文档中服务端接口以及 [Swagger](https://app.swaggerhub.com/apis/QQ_channel/QQ_bot/) 所定义的API接口。
### signature
根据文档中签名校验部分 https://bot.q.qq.com/wiki/develop/api-v2/dev-prepare/interface-framework/sign.html 提供了签名工具类。