use futures_util::{SinkExt, StreamExt};
use log::{error, info};
use sakiko::{load_config, Session};
use std::env;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let (host, port, config_file) = if args.len() == 4 {
        (args[2].clone(), args[3].clone(), args[1].clone())
    } else if args.len() == 2 {
        ("127.0.0.1".to_string(), "3000".to_string(), args[1].clone())
    } else {
        eprintln!("Usage: {} <config file> <host> <port>", args[0]);
        std::process::exit(1);
    };

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let addr = format!("{}:{}", host, port);
    let config = load_config(&config_file).unwrap();
    let listener = TcpListener::bind(&addr).await.unwrap();

    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let mut session = Session::new(config.clone());
        tokio::spawn(async move {
            let ws_stream = match accept_async(stream).await {
                Ok(ws_stream) => ws_stream,
                Err(err) => {
                    error!("Error during WebSocket handshake: {}", err);
                    return;
                }
            };
            let (mut write, mut read) = ws_stream.split();
            let msg = Message::Text(session.get_bot_name().into());
            match write.send(msg).await {
                Ok(_) => info!("Connected to {}", session.get_bot_name()),
                Err(err) => {
                    error!("Failed to send message: {}", err);
                    return;
                }
            }
            loop {
                match session.need_stop() {
                    Ok(true) => break,
                    Ok(false) => (),
                    Err(err) => {
                        error!("Error: {}", err);
                        break;
                    }
                }
                let msg = match session.output() {
                    Ok(output) => Message::Text(output.into()),
                    Err(err) => {
                        error!("Error: {}", err);
                        break;
                    }
                };
                match write.send(msg).await {
                    Ok(_) => (),
                    Err(err) => {
                        error!("Failed to send message: {}", err);
                        break;
                    }
                }
                match session.handle_empty_input() {
                    Ok(true) => continue,
                    Ok(false) => (),
                    Err(err) => {
                        error!("Error: {}", err);
                        break;
                    }
                }
                let msg = match read.next().await {
                    Some(Ok(msg)) => msg,
                    Some(Err(err)) => {
                        error!("Error: {}", err);
                        break;
                    }
                    None => {
                        error!("Connection closed");
                        break;
                    }
                };
                let input = match msg {
                    Message::Text(text) => text,
                    _ => continue,
                };
                match session.handle_input(&input.to_string()) {
                    Ok(_) => (),
                    Err(err) => {
                        error!("Error: {}", err);
                        break;
                    }
                }
            }
            info!("Session ended");
        });
    }
}
