use super::commands::*;
use std::net::SocketAddr;
use std::vec::Vec;

use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

use std::str;
use tokio::sync::Mutex;
use std::sync::Arc;

async fn handle(
    content: &str,
    streams_index: &mut Vec<String>,
    history: &mut Vec<String>,
    srv_addr: &SocketAddr,
) -> bool {
    if content.starts_with("index") {
        println!("index: {}", format!("{:?}", streams_index));
    } else if content.starts_with("luc ") {
        println!("command luc");
        command_i(content, streams_index, history).await;
    } else if content.starts_with("p ") {
        command_p(&mut content.to_owned(), streams_index, history).await;
    } else if content.starts_with("luc? ") {
        command_connect(content, srv_addr.to_string(), streams_index).await;
    } else if content.starts_with("connection") {
        command_connection(content, streams_index);
    } else if content.starts_with("history") {
        println!("{}", format!("{:?}", history))
    }
    content.eq(&String::from("q"))
}

pub async fn start_server(port: &str) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(String::from("127.0.0.1:") + port).await?;
    let streams_index = Arc::new(Mutex::new(Vec::<String>::new()));
    let history = Arc::new(Mutex::new(Vec::<String>::new()));
    println!("Start listening on {}", port);
    loop {
        let (mut socket, _) = listener.accept().await?;
        let srv_addr = listener.local_addr().unwrap();
        let streams_index = streams_index.clone();
        let history = history.clone();
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };
                let mut hist_lock = history.lock().await;
                let mut index_lock = streams_index.lock().await;
                if handle(
                    str::from_utf8(&buf[0..n]).unwrap().trim_end(),
                    &mut index_lock,
                    &mut hist_lock,
                    &srv_addr,
                ).await {
                    println!("Quit server");
                    return;
                }
                //if let Err(e) = socket.write_all(&buf[0..n]).await {
                //    eprintln!("failed to write to socket; err = {:?}", e);
                //    return;
                //}
            }
        });
    }
}
