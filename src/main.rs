use std::io::stdin;
use std::net::{SocketAddr};

mod server;
mod tools;
use server::start_server;
use tools::send;

fn run_client(port: String) -> std::io::Result<()> {
    loop {
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a correct string");
        let local_srv = &SocketAddr::from(([127, 0, 0, 1], port[..4].parse::<u16>().unwrap()));
        send(input, local_srv);
    }
}

fn main() {
    let (jh, port) = start_server();
    run_client(port).expect("Failed running client");
    jh.join().expect("Failed joining server thread");
}
