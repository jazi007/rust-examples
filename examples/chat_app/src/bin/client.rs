use chat_app::Result;
use std::env;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> Result<()> {
    let mut tcp_stream = TcpStream::connect("127.0.0.1:2000").await?;
    let (reader, mut writer) = tcp_stream.split();
    let mut reader = BufReader::new(reader).lines();
    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();
    let name = env::var("USERNAME").unwrap_or(env::var("USER").unwrap_or("Unknown".to_string()));
    writer.write_all(name.as_bytes()).await?;
    writer.write_all(b"\n").await?;

    // Continuous loop for concurrently sending and receiving messages.
    loop {
        tokio::select! {
            incoming = reader.next_line() => {
                match incoming {
                    Ok(Some(text)) => {
                        println!("{}", text);
                    },
                    Ok(_) => return Ok(()),
                    Err(err) => return Err(err.into()),
                }
            }
            res = stdin.next_line() => {
                match res {
                    Ok(None) => return Ok(()),
                    Ok(Some(line)) => {
                        writer.write_all(line.as_bytes()).await?;
                        writer.write_all(b"\n").await?;
                    },
                    Err(err) => return Err(err.into()),
                }
            },
        }
    }
}
