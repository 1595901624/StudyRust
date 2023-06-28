mod test;

use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

use tokio::net::TcpStream;
use futures::{stream, StreamExt};

const PORTS: &[u16] = &[21, 22, 23, 25, 80, 110, 139, 443, 3306, 3389, 8080, 8081, 8888, 22122, 65535];

#[tokio::main]
async fn main() {
    let ip_string = "182.61.200.6";
    let ip = ip_string.parse::<IpAddr>().unwrap();
    let ports = stream::iter(PORTS.into_iter());
    let x = ports.for_each_concurrent(10, |port| {
        scan_port(ip, *port)
    });
    x.await;
}

async fn scan_port(target: IpAddr, port: u16) {
    let address = SocketAddr::new(target, port);
    // println!("正在扫描:{}", address);
    let future = TcpStream::connect(address);
    if let Ok(Ok(_)) = tokio::time::timeout(Duration::from_secs(5), future).await {
        println!("开放端口：{}", port);
    }
}