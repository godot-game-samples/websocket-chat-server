use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use std::io::{Write};

#[tokio::main]
async fn main() {
    let (ws_stream, _) = connect_async("ws://127.0.0.1:9080").await.expect("Connection failed.");
    println!("✅ WebSocket connection succeeded");

    let (mut write, mut read) = ws_stream.split();

    // Standard Input → Send Task
    tokio::spawn(async move {
        let mut stdin_reader = BufReader::new(tokio::io::stdin()).lines();
        loop {
            print!("> ");
            std::io::stdout().flush().unwrap();

            if let Ok(Some(line)) = stdin_reader.next_line().await {
                let msg = line.trim();
                if msg.is_empty() {
                    continue;
                }
                if let Err(e) = write.send(Message::Text(msg.to_string().into())).await {
                    eprintln!("❌ Failure to deliver: {:?}", e);
                    break;
                }
            } else {
                break;
            }
        }
    });

    // 受信タスク
    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => println!("📩 receive: {}", text),
            Ok(_) => println!("📩 non-text message receiving"),
            Err(e) => {
                eprintln!("❌ receive error: {:?}", e);
                break;
            }
        }
    }

    println!("🔚 end of connection");
}
