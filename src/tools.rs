use std::net::{TcpStream};
use std::io::Write;

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

pub fn send(content: String, addr: &std::net::SocketAddr) -> bool {
  match TcpStream::connect_timeout(addr, std::time::Duration::new(30, 0)) {
    Ok(mut stream) => { stream.write_all(content.as_bytes()).expect("Error write in stream"); },
    Err(_) => { println!("Nothing for mow"); }, // should remove from index
  };
  content.eq(&String::from("q\n"))
}
