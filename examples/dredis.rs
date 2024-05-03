use std::{io, net::SocketAddr};

use anyhow::Result;
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};
use tracing::{info, warn};

const ADDR: &str = "0.0.0.0:6380";
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind(ADDR).await?;
    info!("redis server address: {}", ADDR);

    loop {
        let (stream, client_addr) = listener.accept().await?;
        info!("redis client address: {}", client_addr);
        tokio::spawn(async move {
            if let Err(e) = process_redis_conn(stream, client_addr).await {
                warn!("Error processing conn with {}: {:?}", client_addr, e);
            }
        });
    }
}
const BUF_SIZE: usize = 4096;

async fn process_redis_conn(mut stream: TcpStream, client_addr: SocketAddr) -> Result<()> {
    loop {
        stream.readable().await?;
        let mut buf = Vec::with_capacity(BUF_SIZE);
        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                let line = String::from_utf8_lossy(&buf);
                info!(
                    "read {} bytes from {} content is {:?}",
                    n, client_addr, line
                );
                stream.write_all(b"+OK\r\n").await?;
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
    warn!("redis client {} closed", client_addr);
    Ok(())
}
