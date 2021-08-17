use tokio::net::{TcpStream};
use tokio::io::AsyncWriteExt;
//use tokio::io::AsyncReadExt;

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

pub async fn post(content: String, addr: std::net::SocketAddr) {
    if let Ok(mut stream) = TcpStream::connect(addr).await {
        if stream.write_all(content.as_bytes()).await.is_err() {
            println!("Nothing for mow");
        }
    } else {
        println!("Nothing for mow");
    }
}

//pub async fn put(content: String, addr: std::net::SocketAddr) -> Option<String> {
//    if let Ok(mut stream) = TcpStream::connect(addr).await {
//        if stream.write_all(content.as_bytes()).await.is_err() {
//            println!("Nothing for mow");
//        }
//        let mut buffer = String::new();
//        match stream.read_to_string(&mut buffer).await {
//            Ok(_) => Some(buffer),
//            Err(_) => None,
//        }
//    } else {
//        println!("Nothing for mow");
//        None
//    }
//}
//