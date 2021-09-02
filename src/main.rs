use std::io::stdin;
use std::net::SocketAddr;

mod server;
mod tools;
use server::start_server;
use tokio::runtime::Runtime;
use tools::post;

fn run_client(port: String) -> std::io::Result<()> {
    loop {
        let p = port.to_owned();
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Luc! You not a correct string");
        tokio::spawn(async move {
            post(
                &input,
                SocketAddr::from(([127, 0, 0, 1], p.clone()[..4].parse::<u16>().unwrap())),
            )
            .await;
        });
    }
}

fn main() {
    if let Ok(rt) = Runtime::new() {
        rt.block_on(async {
            println!("Open a port");
            let mut buffer = String::new();
            stdin()
                .read_line(&mut buffer)
                .expect("Luc! You not a correct string");
            let p1 = buffer.clone();
            tokio::spawn(async move {
                if let Err(err) = start_server(&buffer[..4]).await {
                    eprintln!("Luc! Error server at {} n't ?", err);
                }
            });
            run_client(p1[..4].to_owned()).expect("Failed running local luc");
        });
    } else {
        eprintln!("Luc! Error runtime")
    }
}
