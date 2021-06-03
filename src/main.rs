use std::error::Error;
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use structopt::StructOpt;
use tokio::net::UdpSocket;

const DEFAULT_BUF_SIZE: usize = 1536;

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(short, long, help = "Bind address")]
    bind_addr: String,
    #[structopt(short, long, help = "Upstream address")]
    upstream_addr: String,
}

async fn run(socket: UdpSocket, upstream: SocketAddr) -> Result<(), io::Error> {
    let bind_addr = socket.local_addr().unwrap();
    let mut buf: Vec<u8> = vec![0; DEFAULT_BUF_SIZE];
    let mut client: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 34567);
    loop {
        let (size, peer) = socket.recv_from(&mut buf).await?;
        if peer == upstream {
            // Send data from upstream to client
            socket.send_to(&buf[..size], client).await?;
            println!(
                "upstream {} -> this {} -> client {}",
                upstream, bind_addr, client
            );
            continue;
        }

        // Store client info as response from upstream will go to this.
        client = peer;
        // Send data from client to upstream
        socket.send_to(&buf[..size], upstream).await?;
        println!(
            "client {} -> this {} -> upstream {}",
            client, bind_addr, upstream
        );
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::from_args();
    let upstream: SocketAddr = args.upstream_addr.parse().unwrap();
    let socket = UdpSocket::bind(&args.bind_addr).await?;
    println!("listening on: {}", socket.local_addr()?);

    run(socket, upstream).await?;

    Ok(())
}
