use super::commands::*;
use std::io::{stdin, Read};
use std::thread::{spawn, JoinHandle};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::vec::Vec;

fn get_msg(bytes: [u8; 1024]) -> String {
  let mut ret = String::new();
  for &c in bytes.iter() {
      if c == b'\0' || c == b'\n' {
          break;
      }
      ret += &String::from_utf8(vec![c]).unwrap()[..];
  }
  ret
}

fn handle(
  mut stream: TcpStream,
  streams_index: &mut Vec<String>,
  history: &mut Vec<String>,
  srv_addr: &SocketAddr,
) -> bool {
  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();
  let mut content = get_msg(buffer);
  if content.starts_with("index") {
    println!("index: {}", format!("{:?}", streams_index));
  } else if content.starts_with("i ") {
    command_i(&content, streams_index, history);
  } else if content.starts_with("p ") {
    command_p(&mut content, streams_index, history);
  } else if content.starts_with("connect ") {
    command_connect(&content, srv_addr.to_string(), streams_index);
  } else if content.starts_with("connection") {
    command_connection(&content, streams_index);
  } else if content.starts_with("history") {
    println!("{}", format!("{:?}", history))
  }
  content.eq(&String::from("q"))
}

pub fn start_server() -> (JoinHandle<()>, String) {
  println!("Open a port");
  let mut buffer = String::new();
  stdin()
      .read_line(&mut buffer)
      .expect("Did not enter a correct string");
  let server_addr = String::from("127.0.0.1:") + &buffer[0..4];
  (
      spawn(move || {
          let listener = TcpListener::bind(&server_addr[..]).unwrap();
          println!("server started");
          let mut streams_index: Vec<String> = Vec::new();
          let mut history: Vec<String> = Vec::new();
          for stream in listener.incoming() {
              let stream = stream.unwrap();
              if handle(stream, &mut streams_index, &mut history, &listener.local_addr().unwrap()) {
                  break;
              }
          }
          println!("server close");
      }),
      buffer[0..4].to_string(),
  )
}
