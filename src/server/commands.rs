use crate::tools::*;
use std::time::SystemTime;
use std::vec::Vec;

/// todo:
/// Propagate a command with another strategy ?
async fn propagate(command: &str, options: String, streams_index: &[String]) {
    for stream in streams_index {
        if let Ok(addr) = stream.parse::<std::net::SocketAddr>() {
            post(command.to_owned() + &options[..], addr).await;
        } else {
            eprintln!("Error ! parse stream '{}' failed", stream);
        }
    }
}

pub async fn command_p(
    mut content: &mut String,
    streams_index: &[String],
    history: &mut Vec<String>,
) -> bool {
    let command = get_first_n_words(&mut content, 2);
    if let Some(cmd) = command {
        for h in history.iter() {
            if content.to_string().eq(h) {
                return false;
            }
        }
        history.push(content.to_string());
        println!("{}", cmd.trim_start());
        propagate(&content[..], cmd, streams_index).await;
    } else {
        println!("cannot get command p from content {}", content);
    }
    true
}

pub async fn command_i(content: &str, streams_index: &[String], history: &mut Vec<String>) {
    let mut command = String::from("p ");
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(t) => command += &t.as_secs().to_string()[..],
        Err(_) => panic!("Error getting time"),
    }
    history.push(String::from(&command[..]));
    command.push(' ');
    propagate(&command[..], remove_prefix(content, "luc "), streams_index).await;
}

pub async fn command_connect(content: &str, server_addr: String, streams_index: &mut Vec<String>) {
    let addr = remove_prefix(content, "luc? ");
    streams_index.push(addr);
    propagate("connection", server_addr, streams_index).await;
}

pub fn command_connection(content: &str, streams_index: &mut Vec<String>) {
    let addr = remove_prefix(content, "connection");
    println!("luc?");
    streams_index.push(addr);
}

//pub async fn command_collect(content: &str, streams_index: &mut Vec<String>) {
//    for stream in streams_index {
//        if let Ok(addr) = stream.parse::<SocketAddr>() {
//            put(String::new(), addr).await;
//        } else {
//            eprintln!("Error ! parse stream '{}' failed", stream);
//        }
//    }
//}
