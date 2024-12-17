# Sakiko —— 基于领域特定语言的客服机器人设计与实现

## 1. 项目简介

领域特定语言（Domain Specific Language, DSL）是一种专门用于解决某一领域问题的编程语言。在软件开发中，DSL 被广泛应用于领域建模、代码生成、配置管理等领域。本项目旨在设计和实现一个基于 DSL 的客服机器人，通过 DSL 描述客服机器人的对话流程，从而简化客服机器人的设计和实现。

## 2. 项目架构

### 2.1 解释器和执行器

本项目的客服机器人由解释器和执行器两部分组成。解释器负责解析 DSL 描述的对话流程，生成对应的配置类，而执行器则负责根据配置类执行对话流程。

执行器支持同步和异步的 IO 操作，通过调用不同的方法实现不同的 IO 操作。同时执行器还将一些常用方法流程进行封装，提供了控制台标准 IO 同步操作的可直接调用的方法。

### 2.2 可执行程序

本项目包含 4 个可执行程序，分别是：

- `sakiko`：可指定配置文件的同步客服机器人程序，使用同步标准 IO 进行对话。
- `checker`：DSL 配置文件检查程序，用于检查 DSL 配置文件是否合法，可杜绝一些常见的配置错误。
- `server`: 通过 WebSocket 提供异步服务的服务器程序，可指定配置文件以及监听地址和端口。
- `client`: 配套 `server` 的客户端程序，与用户通过异步标准 IO 进行对话。

## 3. DSL 设计

DSL 设计以 yaml 为基础，支持以下几种元素：

- 步骤：对话流程的基本单元，包含对话内容和向下一步骤的跳转。
- 变量：支持整形、浮点型、字符串和他们的数组和哈希表变量类型，可对其进行赋值、运算和输出。
- 输入：支持用户输入，可以用字符串或正则表达式进行匹配。
- 条件：通过变量的比较运算进行判断，支持与运算和或运算。
- 操作：对变量进行操作，包括赋值、运算以及将用户输入内容保存到变量中。

DSL 详细语法请参考 [DSL 语法](./docs/GRAMMAR.md)。

## 4. 使用方法

### 4.1 直接使用

如果您只是想使用客服机器人，请确认自己的环境中已经安装了 Rust（cargo）运行时环境，对项目文件进行下载后，使用 `cargo run --bin sakiko -- <your_config>` 命令即可运行在标准 IO 上的客服机器人。

注意：以上程序运行在调试环境中，更具体的使用方法请参考 [使用说明](./docs/USAGE.md)。

### 4.2 调用库

如果您想在自己的项目中使用客服机器人，可以将本项目作为库引入到您的项目中。

面向用户开放的接口详见 [API 文档](./docs/API.md)。

### 4.3 底层开发

如果想修改底层逻辑或者增加新的功能，请参考 [开发文档](./docs/DEVELOP.md)，这其中包含了项目的整体设计以及各个模块的详细设计，同时也包含了测试桩以及测试方法。

## 5. 可移植性

本项目基于 Rust 语言开发，且并未调用系统特殊接口，理论上支持所有 Rust 支持的平台。

## 6. 测试文件

本项目提供多个测试文件，分别是：

- `demo.yaml`：加法口算测试器
- `demo2.yaml`：简单的课表查询系统
- `demo3.yaml`：21 点游戏
- `error.yaml`：错误的测试文件，用于测试 `checker` 程序的错误检查功能

在运行测试时，会额外产生一个 `test.yaml` 文件，该文件不可执行，其也无法通过 `checker` 程序检查，仅用于测试文件读取和解析的功能。