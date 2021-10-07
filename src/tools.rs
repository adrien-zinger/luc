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

pub async fn _send_file(command: &str, path: &str, stream: &mut TcpStream) {
    if let Ok(data) = std::fs::read(path) {
        _write_binary(command, &data, stream).await;
    }
}

pub async fn _write_binary(command: &str, binary: &[u8], stream: &mut TcpStream) {
    if stream.write_all(&write_buffer(command, Some(binary))).await.is_err() {
        eprintln!("Cannot write to stream");
    }
}

pub async fn write(content: &str, stream: &mut TcpStream) {
    if stream.write_all(content.as_bytes()).await.is_err() {
        eprintln!("Cannot write to stream");
    }
}

async fn write_command(stream: &mut TcpStream, command: &str) {
    if stream.write_all(&write_buffer(command, None)).await.is_err() {
        eprintln!("Cannot write to stream");
    }
}

pub async fn post(content: &str, addr: std::net::SocketAddr) {
    match TcpStream::connect(addr).await {
        Ok(mut stream) => {
            write_command(&mut stream, content).await;
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

fn get_len(buf: &[u8]) -> usize {
    let mut len = buf[0] as usize;
    len <<= 8;
    len += buf[1] as usize;
    len <<= 8;
    len += buf[2] as usize;
    len <<= 8;
    len += buf[3] as usize;
    len
}

fn write_buffer(command: &str, binary: Option<&[u8]>) -> Vec<u8> {
    let mut buf_vec = Vec::new();
    if let Some(vec) = binary {
        let command = format!("binary {}", command);
        let len = command.len() as i32;
        buf_vec.append(&mut len.to_be_bytes().to_vec());
        buf_vec.append(&mut command.as_bytes().to_vec());
        buf_vec.append(&mut vec.to_vec());
    } else {
        let len = command.len() as i32;
        buf_vec.append(&mut len.to_be_bytes().to_vec());
        buf_vec.append(&mut command.as_bytes().to_vec());
    }
    buf_vec
}

fn read_buffer(buf: Vec<u8>) -> Option<(String, Option<Vec::<u8>>)> {
    let command_len = get_len(&buf);
    let mut command = std::str::from_utf8(&buf[4..command_len+4]).unwrap();
    if command.starts_with("binary") {
        command = command.strip_prefix("binary ").unwrap();        
        let mut buf = buf.to_vec();
        buf.drain(0..command_len+4);
        return Some((command.to_string(), Some(buf)))
    }
    Some((command.to_string(), None))
}

pub async fn read(stream: &mut TcpStream) -> Option<(String, Option<Vec::<u8>>)> {
    let mut buf_vec = Vec::new();
    let n = match stream.read_to_end(&mut buf_vec).await {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Luc! failed to read from socket; err = {:?}", e);
            0
        }
    };
    if n < 5 {
        eprintln!("Error input too small");
        return None;
    }
    read_buffer(buf_vec)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_write() {
        let buf = write_buffer("command", Some(&vec![1,2,3]));
        let res = read_buffer(buf).unwrap();
        assert_eq!(res.0, "command");
        assert_eq!(res.1, Some(vec![1,2,3]));
        let buf = write_buffer("command one", None);
        let res = read_buffer(buf).unwrap();
        assert_eq!(res.0, "command one");
        assert_eq!(res.1, None);
    }
}