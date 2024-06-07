use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
struct Args {
    port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let sock = tokio::net::UdpSocket::bind(("0.0.0.0", args.port)).await?;

    let mut buf = [0; 65535];
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        let buf = &buf[..len];
        let time = chrono::Utc::now();
        println!("{}: Received {} bytes from {}", time, buf.len(), addr);
        sock.send_to(format!("Received {} bytes\n", buf.len()).as_bytes(), addr)
            .await?;
    }
}
