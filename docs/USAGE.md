# Sakiko 可执行程序使用说明

## 1. sakiko

`saikko` 是一个可指定配置文件的同步客服机器人程序，使用同步标准 IO 进行对话。

### 1.1 使用方法

使用 `cargo`

```shell
cargo run --release --bin sakiko -- <your_config>
```

直接运行

```shell
./sakiko <your_config>
```

### 1.2 交互

`saikko` 会读取配置文件，然后开始对话。对话的过程中，由于采用了同步标准 IO，请在输出完成后输入内容。

### 1.3 错误处理

由于是同步且单线程处理，`saikko` 会在出现错误时直接退出。

## 2. checker

`checker` 是一个 DSL 配置文件检查程序，用于检查 DSL 配置文件是否合法，可杜绝一些常见的配置错误。

`checker` 的用法详见 [DSL 语法](./docs/GRAMMAR.md) 中的 `checker` 部分。

***注意*** 除了 `checker` 外的所有可执行程序均不会预检查配置文件的合法性，请确保配置文件正确。

## 3. server

`server` 是一个通过 WebSocket 提供异步服务的服务器程序，可指定配置文件以及监听地址和端口。

### 3.1 使用方法

使用 `cargo`

```shell
cargo run --release --bin server -- <your_config> <your_address> <your_port>
```

直接运行

```shell
./server <your_config> <your_address> <your_port>
```

其中 `<your_address>` 和 `<your_port>` 为可选参数，分别表示监听地址和端口，默认为 `127.0.0.1` 和 `3000`。

### 3.1 交互

`server` 会读取配置文件，然后开始监听。当有客户端连接时，`server` 会将配置文件发送给客户端，然后开始对话。

`server` 会显示默认 `info` 和以上级别的日志，包括连接信息、对话信息等。

要想显示更多信息，请修改日志环境变量 `RUST_LOG`。

### 3.2 错误处理

`server` 会在出现错误时直接结束当前连接，但不会退出。

异步过程中所有的设计内可能出现的错误都会被捕获并记录在日志中。

设计外的错误会直接显示但程序依然继续运行。

在初始化阶段出现的错误会导致程序直接退出。

## 4. client

`client` 是配套 `server` 的客户端程序，与用户通过异步标准 IO 进行对话。

### 4.1 使用方法

使用 `cargo`

```shell
cargo run --release --bin client -- <your_address> <your_port>
```

直接运行

```shell
./client <your_address> <your_port>
```

其中 `<your_address>` 和 `<your_port>` 为可选参数，分别表示服务器地址和端口，默认为 `127.0.0.1` 和 `3000`。

### 4.2 交互

`client` 会连接到服务器，然后开始对话。对话的过程中，由于采用了异步标准 IO，在输出完成前的输入依然有效，但仍然建议在输出完成后输入内容。

### 4.3 错误处理

`client` 会在出现错误时直接退出。

`server` 的错误信息不会告诉给 `client`。