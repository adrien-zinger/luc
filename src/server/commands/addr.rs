use crate::tools::{put, write};
use std::sync::Arc;
use std::vec::Vec;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

pub async fn addr(streams_index: &Arc<Mutex<Vec<String>>>) -> Option<String> {
    // Ask to another in the network what's my ip
    let index = streams_index.lock().await.to_vec();
    for ip in index.iter() {
        let resp = put("raddr", ip.parse().unwrap()).await;
        if resp.is_some() {
            return resp;
        }
    }
    None
}

pub async fn raddr(stream: &mut TcpStream) {
    let addr = stream.peer_addr().unwrap();
    //stream.local_addr();
    write(&addr.ip().to_string(), stream).await;
}
