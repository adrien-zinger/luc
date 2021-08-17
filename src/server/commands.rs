use std::vec::Vec;
use std::time::SystemTime;
use crate::tools::*;
use std::net::{SocketAddr};

/// todo:
/// Propagate a command with another strategy ?
fn propagate(command: &str, options: String, streams_index: &[String]) {
  for stream in streams_index {
      send(
          command.to_owned() + &options[..],
          &stream.parse::<SocketAddr>().unwrap(),
      );
  }
}

pub fn command_p(mut content: &mut String, streams_index: &[String], history: &mut Vec<String>) -> bool {
  let command = get_first_n_words(&mut content, 2);
  if let Some(cmd) = command {
    for h in history.iter() {
      if content.to_string().eq(h) {
        return false;
      }
    }
    history.push(content.to_string());
    println!("{}", cmd);
    propagate(&content[..], cmd, streams_index);
  } else {
    println!("cannot get command p from content {}", content);
  }
  true
}

pub fn command_i(content: &str, streams_index: &[String], history: &mut Vec<String>) {
  let mut command = String::from("p ");
  match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
    Ok(t) => command += &t.as_secs().to_string()[..],
    Err(_) => panic!("Error getting time"),
  }
  history.push(String::from(&command[..]));
  command.push(' ');
  propagate(&command[..], remove_prefix(content, "luc "), streams_index);
}

pub fn command_connect(content: &str, server_addr: String, streams_index: &mut Vec<String>) {
  let addr = remove_prefix(content, "luc? ");
  streams_index.push(addr);
  propagate("connection", server_addr, streams_index);
}

pub fn command_connection(content: &str, streams_index: &mut Vec<String>) {
  let addr = remove_prefix(content, "connection");
  println!("luc?");
  streams_index.push(addr);
}
