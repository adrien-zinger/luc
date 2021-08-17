use std::io::prelude::*;
use std::io::stdin;
use std::net::{SocketAddr, TcpStream};

mod server;
mod tools;
use server::start_server;

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

fn send(content: String, addr: &SocketAddr) -> bool {
    let mut stream =
        TcpStream::connect_timeout(addr, std::time::Duration::new(30, 0)).expect("wow");
    stream.write(content.as_bytes()).expect("writting stream error");
    content.eq(&String::from("q\n"))
}

fn main() {
    let (jh, port) = start_server();
    run_client(port).expect("Failed running client");
    jh.join().expect("Failed joining server thread");
}
