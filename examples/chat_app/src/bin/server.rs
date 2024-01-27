use chat_app::Result;
use once_cell::sync::Lazy;
use std::collections::VecDeque;
use std::net::SocketAddr;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::{
        broadcast::{channel, Sender},
        RwLock,
    },
};

static MESSAGES: Lazy<RwLock<VecDeque<String>>> = Lazy::new(|| RwLock::new(VecDeque::new()));

async fn update_message(msg: String) {
    let mut messages = MESSAGES.write().await;
    if messages.len() >= 100 {
        let _ = messages.pop_front();
    }
    messages.push_back(msg);
}

async fn handle_connection(
    addr: SocketAddr,
    mut tcp_stream: TcpStream,
    bcast_tx: Sender<(SocketAddr, String)>,
) -> Result<()> {
    let (reader, mut writer) = tcp_stream.split();
    let mut reader = BufReader::new(reader).lines();
    let mut bcast_rx = bcast_tx.subscribe();
    let name = reader.next_line().await?.unwrap();
    println!("New connection from {addr:?} : {:?}", name);
    writer.write_all(b"Welcome to chat! Type a message").await?;
    writer.write_all(b"\n").await?;
    {
        // Send last 100 messages to
        let messages = MESSAGES.read().await;
        for msg in messages.iter() {
            writer.write_all(msg.as_bytes()).await?;
            writer.write_all(b"\n").await?;
        }
    }

    // Continuous loop to handle message and broadcast
    loop {
        tokio::select! {
            incoming = reader.next_line() => {
                match incoming {
                    Ok(Some(text)) => {
                        let msg = format!("{:?}: {:?}", name, text);
                        bcast_tx.send((addr, msg.clone()))?;
                        update_message(msg).await;
                    }
                    Ok(_) => return Ok(()),
                    Err(err) => return Err(err.into()),
                }
            }
            msg = bcast_rx.recv() => {
                let (c_addr, text) = msg?;
                if c_addr.ne(&addr) {
                    writer.write_all(text.as_bytes()).await?;
                    writer.write_all(b"\n").await?;
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let (bcast_tx, _) = channel(16);
    let listener = TcpListener::bind("127.0.0.1:2000").await?;
    loop {
        let (socket, addr) = listener.accept().await?;
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move { handle_connection(addr, socket, bcast_tx).await });
    }
}
