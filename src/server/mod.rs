mod commands;

use std::str;
use std::sync::Arc;
use std::vec::Vec;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use crate::tools::read;

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
        let msg = read(&mut socket).await;
        if let Some(msg) = msg {
            tokio::spawn(async move {
                if commands::handle(
                    &msg.0,
                    None,
                    &mut socket,
                    &globals,
                )
                .await
                {
                    println!("Quit server");
                }
            });
        } else {
            eprintln!("Luc! error reading the message")
        }
        
    }
}
