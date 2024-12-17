# Sakiko 库使用 API

Sakiko 库提供了一系列面向用户的接口，用于在用户项目中使用 Sakiko 客服机器人。本文档将详细介绍这些接口的使用方法。

## 1. 一般接口

为了方便使用，将常用的类和函数直接封装在 `lib.rs` 中，用户可以直接通过 `use sakiko::*;` 引入这些类和函数。

### 1.1 `load_config` 函数

`load_config` 函数用于加载配置文件，返回一个由 `Arc` 智能指针包装的 `Config` 类型的对象。

#### 参数

- `path`: 引用字符串，表示配置文件的路径。

#### 返回值

- `Result<Arc<config::SakikoConfig>, serde_yaml::Error>` 类型，表示加载配置文件的结果。

使用智能指针的原因是在多线程环境下，多个线程可能会同时访问配置类，因此需要使用智能指针来保证线程安全。

#### 异常

所有异常均处于返回值中，不会触发 panic。

### 1.2 `SakikoConfig` 类

`SakikoConfig` 类用于表示配置类，包含了客服机器人的配置。

#### 内容

所有字段均为私有字段

#### 特征

- `SakikoConfig` 实现了 `Debug`、`Deserialize`、`Serialize`和`PartialEq` 特征。

#### 方法

- `pub fn serialize(&self) -> String`: 将配置类序列化为字符串，返回序列化后的字符串。
- `pub fn deserialize(s: &str) -> Result<SakikoConfig, serde_yaml::Error>`: 将字符串反序列化为配置类，返回反序列化后的配置类。
- `pub fn serialize_to_file(&self, file_path: &str) -> Result<(), serde_yaml::Error>`: 将配置类序列化到文件，返回序列化结果。
- `pub fn deserialize_from_file(file_path: &str) -> Result<SakikoConfig, serde_yaml::Error>`: 从文件中反序列化配置类，返回反序列化结果。

### 1.3 `Session` 类

`Session` 类用于表示一个会话，包含了会话的状态和上下文。

#### 内容

所有字段均为私有字段

#### 特征

- `Session` 实现了 `Debug` 和 `Clone` 特征。

#### 方法

- `pub fn new(config: Arc<SakikoConfig>) -> Session`: 创建一个新的会话，返回新的会话对象，传入的参数为配置类（`Arc` 智能指针）。
- `pub fn get_bot_name(&self) -> &str`: 获取机器人的名字，返回机器人的名字。
- `pub fn is_end(&self) -> bool`: 判断会话是否结束，返回会话是否结束的布尔值。
- `pub fn output(&self) -> Result<String, &'static str>`: 获取会话的输出，返回会话的输出，错误信息为静态字符串引用。
- `pub fn handle_empty_input(&mut self) -> Result<bool, String>`: 处理空输入，返回处理结果（真为空输入跳转成功），错误信息为字符串。
- `pub fn handle_empty_output(&mut self) -> Result<(), String>`: 处理空输出，返回空，错误信息为字符串。***注意：此方法包含循环，请注意避免无限循环***
- `pub fn handle_input(&mut self, input: &str) -> Result<(), String>`: 处理输入，返回空，错误信息为字符串。
- `pub async fn output_async<W: AsyncWrite + Unpin>(&self, mut writer: W) -> io::Result<()>`：异步输出，返回空，错误信息为 `io::Error`，参数为实现了 `AsyncWrite + Upin` 特征的对象。
- `pub async fn handle_empty_input_async<W: AsyncWrite + Unpin>(&mut self, mut writer: W) -> io::Result<bool>`：异步处理空输入，返回空，错误信息为 `io::Error`，参数为实现了 `AsyncWrite + Upin` 特征的对象。
- `pub fn need_stop(&mut self) -> Result<bool, String>`：对所有无 IO 步骤的封装，返回是否结束会话，错误信息为字符串。
- `pub fn run_stdio(&mut self) -> Result<(), String>`：运行标准 IO，返回空，错误信息为字符串。

## 2. 模块

Sakiko 库包含了多个模块，`config`、`sakiko`、`check`模块公开了一些接口，用户可以直接使用这些接口。

### 2.1 `config` 模块

`config` 模块实际是实现 `SakikoConfig` 类的模块，用户可以直接使用 `SakikoConfig` 类。

### 2.2 `sakiko` 模块

`sakiko` 模块实现了 `Session` 类，用户可以直接使用 `Session` 类。

### 2.3 `check` 模块

`check` 模块实现了 `check_config` 函数，通过调用这个函数可以检查配置文件是否合法。

#### `check_config` 函数

```rust
pub fn check_config(config: &SakikoConfig) -> Result<(), String>
```

##### 参数

- `config`: `&SakikoConfig` 类型，表示配置类的引用。

##### 返回值

在无异常时返回空，否则返回错误信息。

错误信息包含所有语法错误的报告。