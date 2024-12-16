/// 基于 tokio-tungstenite 的 WebSocket 客户端
/// 用法：cargo run --bin client <host> <port>
/// 默认连接为 ws://127.0.0.1:3000
use futures_util::{SinkExt, StreamExt};
use std::env;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::sync::watch;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let (host, port) = if args.len() == 3 {
        (args[1].clone(), args[2].clone())
    } else if args.len() == 1 {
        ("127.0.0.1".to_string(), "3000".to_string())
    } else {
        eprintln!("Usage: {} <host> <port>", args[0]);
        std::process::exit(1);
    };
    let url = format!("ws://{}:{}", host, port);
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    let (mut write, mut read) = ws_stream.split();

    // 创建标准输入输出的读写器
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin_reader = BufReader::new(stdin).lines();
    let mut stdout_writer = BufWriter::new(stdout);

    let mut bot_name = String::new();

    // 接收 bot_name
    if let Some(Ok(msg)) = read.next().await {
        if let Message::Text(name) = msg {
            println!("Connected to {}", name);
            bot_name = name.to_string();
        }
    }

    // 创建一个 watch 用于通知退出
    let (tx, mut rx) = watch::channel(());

    // 从标准输入读取用户输入，并发送到 WebSocket 服务器
    let client_to_server = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = rx.changed() => {
                    break;
                }
                line = stdin_reader.next_line() => {
                    if let Ok(Some(line)) = line {
                        let msg = Message::Text(line.into());
                        write.send(msg).await.expect("Failed to send message");
                    } else {
                        break;
                    }
                }
            }
        }
    });

    // 从 WebSocket 服务器读取消息，并输出到标准输出
    let server_to_client = tokio::spawn(async move {
        while let Some(Ok(msg)) = read.next().await {
            if let Message::Text(text) = msg {
                stdout_writer
                    .write_all(format!("[{}] ", bot_name).as_bytes())
                    .await
                    .unwrap();
                stdout_writer
                    .write_all(text.to_string().as_bytes())
                    .await
                    .unwrap();
                stdout_writer.write_all(b"\n").await.unwrap();
                stdout_writer.flush().await.unwrap();
            }
        }
        let _ = tx.send(());
    });

    server_to_client.await.expect("server_to_client panicked");

    client_to_server.await.expect("client_to_server panicked");

    println!("Connection closed, press Enter to exit");
}
