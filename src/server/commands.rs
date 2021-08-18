use crate::tools::*;
use std::net::SocketAddr;
use std::sync::Arc;
use std::vec::Vec;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

/// todo:
/// Propagate a command with another strategy ?
async fn propagate(command: &str, options: String, streams_index: &Arc<Mutex<Vec<String>>>) {
    for stream in streams_index.lock().await.to_vec() {
        if let Ok(addr) = stream.parse::<std::net::SocketAddr>() {
            post(command.to_owned() + &options[..], addr).await;
        } else {
            eprintln!("Error ! parse stream '{}' failed", stream);
        }
    }
}

pub async fn command_p(
    content: &str,
    streams_index: &Arc<Mutex<Vec<String>>>,
    history: &Arc<Mutex<Vec<String>>>,
) {
    let mut hist_key = content.to_owned();
    let command = get_first_n_words(&mut hist_key, 2);
    let mut hist_guard = history.lock().await;
    if let Some(cmd) = command {
        if !hist_guard.iter().any(|i| hist_key.eq(i)) {
            hist_guard.push(hist_key);
            std::mem::drop(hist_guard);
            println!("{}", cmd.trim_start()); // Print message from luc
            propagate(content, cmd, streams_index).await;
        }
    } else {
        eprintln!("cannot get command p from content {}", content);
    }
}

pub async fn command_i(
    content: &str,
    streams_index: &Arc<Mutex<Vec<String>>>,
    history: &Arc<Mutex<Vec<String>>>,
) {
    let mut command = String::from("p ");
    if let Some(hash) = hash_command(&command) {
        command += &hash[..];
        history.lock().await.push(command.to_owned());
        command.push(' ');
        propagate(&command[..], remove_prefix(content, "luc "), streams_index).await;
    } else {
        eprintln!("Luc! Error hash command");
    }
}

pub async fn command_connect(
    content: &str,
    server_addr: String,
    streams_index: &Arc<Mutex<Vec<String>>>,
) {
    let addr = remove_prefix(content, "luc? ");
    streams_index.lock().await.push(addr);
    propagate("connection", server_addr, streams_index).await;
}

pub async fn command_connection(content: &str, streams_index: &Arc<Mutex<Vec<String>>>) {
    let addr = remove_prefix(content, "connection");
    println!("luc?");
    streams_index.lock().await.push(addr);
}

async fn command_collect(
    content: &str,
    streams_index_copy: &[String],
    history: &Arc<Mutex<Vec<String>>>,
) -> bool {
    let mut hist_guard = history.lock().await;
    if !hist_guard.iter().any(|i| content.eq(i)) {
        hist_guard.push(content.to_owned());
        std::mem::drop(hist_guard);
        for stream in streams_index_copy {
            if let Ok(addr) = stream.parse::<std::net::SocketAddr>() {
                if let Some(response) = put(content, addr).await {
                    println!("Luc's response {}", response);
                }
            } else {
                eprintln!("Error ! parse stream '{}' failed", stream);
            }
        }
        return true;
    }
    false
}

pub async fn commands_luc_inter(
    option: &str,
    content: &str,
    streams_index: &Arc<Mutex<Vec<String>>>,
    history: &Arc<Mutex<Vec<String>>>,
    srv_addr: &SocketAddr,
    stream: &mut tokio::net::TcpStream,
) {
    if option.parse::<std::net::SocketAddr>().is_ok() {
        command_connect(content, srv_addr.to_string(), streams_index).await;
        return;
    }
    let index_copy = streams_index.lock().await.clone();
    if option.is_empty() {
        if let Some(hash) = hash_command(&content.to_owned()) {
            if command_collect(&hash, &index_copy, history).await {
                if let Err(e) = stream
                    .write_all(format!("{:?}", index_copy).as_bytes())
                    .await
                {
                    eprintln!("Luc! I 'nt luc? !! {}", e);
                }
            }
        } else {
            eprintln!("Luc! Error hash command");
        }
    } else if command_collect(content, &index_copy, history).await {
        if let Err(e) = stream
            .write_all(format!("{:?}", index_copy).as_bytes())
            .await
        {
            eprintln!("Luc! I 'nt luc? !! {}", e);
        }
    }
}
