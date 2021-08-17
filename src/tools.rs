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

pub fn remove_prefix(msg: &String, to_strip: &str) -> String {
  match msg.strip_prefix(to_strip) {
    Some(m) => m.to_string(),
    None => panic!("No I cannot remove prefix {} of {}", to_strip, msg),
  }
}

pub fn send(content: String, addr: &std::net::SocketAddr) -> bool {
  let mut stream =
      TcpStream::connect_timeout(addr, std::time::Duration::new(30, 0)).expect("Stream connection timeout");
  stream.write(content.as_bytes()).expect("Error write in stream");
  content.eq(&String::from("q\n"))
}
