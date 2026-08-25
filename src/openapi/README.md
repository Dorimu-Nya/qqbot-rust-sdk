# OpenAPI 目录说明

该目录按领域组织 QQ Bot OpenAPI。`QQApiClient` 是链式调用的根节点，各级 API 分组只持有同一个 `Arc<dyn ApiRequest>`，不直接依赖具体的 HTTP 请求库。

## 顶层文件

```text
openapi/
├── api.rs                  # QQApiClient 根节点
├── api_request.rs          # 请求抽象、通用请求处理和错误类型
├── create_api_client.rs    # 组装默认客户端
├── reqwest.rs              # 基于 reqwest 的 ApiRequest 实现
├── reqwest_middleware.rs   # token 获取、缓存和注入中间件
├── apis/                   # 按领域组织的 API
└── models/                 # 跨领域共用模型
```

`api_request.rs` 负责请求体序列化、响应体反序列化及错误响应处理。`reqwest.rs` 只负责发送 HTTP 请求。后续切换请求库时，应新增或替换 `ApiRequest` 的实现，避免修改领域 API。

## API 分组

`apis` 第一层按领域分组，例如：

```text
apis/
├── user/
├── guild/
├── channel/
├── message/
├── interaction/
└── menu_panel/
```

领域内可以根据模型共用范围继续拆分子分组。例如频道领域分为 `audio`、`message`、`permissions`、`schedule` 等。每一级分组的结构体都定义在对应目录的 `mod.rs` 中，并通过为上一级结构体实现方法形成链式调用：

```rust
client.channel().message().post_message(...).await
```

## 接口文件

一个接口对应一个 `.rs` 文件。文件名和方法名使用该接口在官方 SDK 中对应的方法命名，不按 URL 机械生成名称。

接口文件包含：

- 请求方法；
- 仅由该接口使用的请求和响应模型；
- 接口路径、HTTP Method、Query 和 Body 的组装。

GET 接口不为 Query 单独创建请求结构体，直接通过方法参数接收并组装 Query。其他请求只有在模型被同一分组内多个接口共用时，才将模型提取到 `models` 目录。

## 模型放置规则

模型按最小共用范围放置：

1. 仅一个接口使用：放在该接口文件中；
2. 同一子分组的多个接口共用：放在 `apis/<domain>/<group>/models/`；
3. 同一领域的多个子分组共用：放在 `apis/<domain>/models/`；
4. 跨领域共用：放在顶层 `models/<model_name>.rs`。

不要因为模型名称通用就放到顶层，应以实际引用范围为准。

## 模块引用

模块之间使用相对路径引用，不通过 `pub use` 重导出类型。新增接口后，需要在所属目录的 `mod.rs` 中声明对应模块；只有确实需要暴露给外部调用方的模块才使用 `pub mod`。

## 默认客户端创建流程

`create_qq_api_client` 接收 `app_id` 和 `app_secret`，创建 token 中间件及 reqwest 客户端，最后返回 `QQApiClient`。中间件创建时会获取一次 token，后续请求到达时按过期时间自动刷新并写入认证请求头。
