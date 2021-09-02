mod commands;

use std::str;
use std::sync::Arc;
use std::vec::Vec;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Globals {
    pub index: Arc<Mutex<Vec<String>>>,
    pub history: Arc<Mutex<Vec<String>>>,
    pub addr: Arc<Mutex<String>>,
}

impl Globals {
    fn new() -> Globals {
        Globals {
            index: Arc::new(Mutex::new(Vec::<String>::new())),
            history: Arc::new(Mutex::new(Vec::<String>::new())),
            addr: Arc::new(Mutex::new(String::new())),
        }
    }
}

pub async fn start_server(port: &str) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(String::from("0.0.0.0:") + port).await?;
    let globals = Globals::new();
    println!("Start listening on {}", port);
    loop {
        let (mut socket, _) = listener.accept().await?;
        let globals = globals.clone();
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            // In a loop, read data from the socket and write the data back.
            // todo: read should decreypt with a local file key, and post
            // should encrypt with the same
            // todo 2: the read should be able to handle chunks, and post
            // should be able to understand when to send a chunk and sends
            let n = match socket.read(&mut buf).await {
                // socket closed
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Luc! failed to read from socket; err = {:?}", e);
                    return;
                }
            };
            if commands::handle(
                str::from_utf8(&buf[0..n]).unwrap().trim_end(),
                &mut socket,
                &globals,
            )
            .await
            {
                println!("Quit server");
            }
        });
    }
}
