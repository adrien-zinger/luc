use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::time::SystemTime;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub fn get_first_n_words(cmd: &mut String, n: usize) -> Option<String> {
    let mut spaces = 0;
    for (i, c) in cmd.chars().enumerate() {
        if c == ' ' {
            spaces += 1;
        }
        if spaces == n {
            let r = &*cmd.split_off(i);
            return Some(r.to_string());
        }
    }
    None
}

pub fn remove_prefix(msg: &str, to_strip: &str) -> String {
    match msg.strip_prefix(to_strip) {
        Some(m) => m.to_string(),
        None => panic!("No I cannot remove prefix {} of {}", to_strip, msg),
    }
}

pub async fn write(content: &str, stream: &mut TcpStream) {
    if stream.write_all(content.as_bytes()).await.is_err() {
        eprintln!("Cannot write to stream");
    }
}

pub async fn post(content: &str, addr: std::net::SocketAddr) {
    match TcpStream::connect(addr).await {
        Ok(mut stream) => {
            if stream.write_all(content.as_bytes()).await.is_err() {
                eprintln!("Cannot write to stream");
            }
        }
        Err(e) => panic!("{}", e),
    };
}

pub async fn put(content: &str, addr: std::net::SocketAddr) -> Option<String> {
    if let Ok(mut stream) = TcpStream::connect(addr).await {
        if stream.write_all(content.as_bytes()).await.is_err() {
            eprintln!("Nothing for mow");
        }
        let mut buffer = String::new();
        match stream.read_to_string(&mut buffer).await {
            Ok(_) => Some(buffer),
            Err(_) => None,
        }
    } else {
        eprintln!("Nothing for mow");
        None
    }
}

pub fn hash_command(command: &str) -> Option<String> {
    if let Ok(t) = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        let to_hash = command.to_owned() + &t.as_secs().to_string()[..];
        let mut s = DefaultHasher::new();
        to_hash.hash(&mut s);
        return Some(to_hash);
    }
    None
}
