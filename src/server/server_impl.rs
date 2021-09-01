use crate::server::command_parser::parse_command;
use super::commands::command_i::command_i;
use super::commands::command_p::command_p;
use super::commands::command_inter::{commands_luc_inter, command_connection};
use super::commands::command_group::*;
use super::commands::command_research as find;
use std::net::SocketAddr;
use std::vec::Vec;

use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};

use std::str;
use std::sync::Arc;
use tokio::sync::Mutex;

// Todo: handle command in a separeted file
async fn handle(
    content: &str,
    streams_index: &Arc<Mutex<Vec<String>>>,
    history: &Arc<Mutex<Vec<String>>>,
    srv_addr: &SocketAddr,
    stream: &mut TcpStream,
) -> bool {
    let act = parse_command(content);
    if act.is_none() {
        eprintln!("Luc! Error input ---> \n{}", content);
        return false;
    }
    let action = act.unwrap();
    if action.name == "index" {
        println!(
            "index: {}",
            format!("{:?}", streams_index.lock().await.to_vec())
        );
    } else if action.name == "luc" {
        command_i(content, streams_index, history).await;
    } else if action.name == "p" {
        command_p(content, streams_index, history).await;
    } else if action.name == "luc?" {
        commands_luc_inter(
            action.option,
            content,
            streams_index,
            history,
            srv_addr, // todo: send &str instead of a useless big object
            stream,
        ) // todo send less objects, maybe create structures for index/history?
        .await
    } else if action.name == "connection" {
        command_connection(content, streams_index).await;
    } else if action.name == "history" {
        println!("{}", format!("{:?}", history.lock().await.to_vec()))
    } else if action.name == "group" {
        group_create(action.option, &srv_addr.to_string()).await;
    } else if action.name == "invite" {
        group_invite(action.option).await;
    } else if action.name == "updategroup" {
        group_update(action.option).await;
    } else if action.name == "find" {
        find::propagate(action.option, streams_index, history).await;
    }
    action.name == "q"
}

pub async fn start_server(port: &str)
-> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(String::from("0.0.0.0:") + port).await?;
    let streams_index = Arc::new(Mutex::new(Vec::<String>::new()));
    let history = Arc::new(Mutex::new(Vec::<String>::new()));
    println!("Start listening on {}", port);
    loop {
        let (mut socket, _) = listener.accept().await?;
        let srv_addr = listener.local_addr().unwrap();
        let ind_arc = Arc::clone(&streams_index);
        let his_arc = Arc::clone(&history);
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            // In a loop, read data from the socket and write the data back.
            // todo: read should decreypt with a local file key, and post
            // should encrypt with the same
            // todo 2: the read should be able to handle chunks, and post
            // should be able to understand when to send a chunk and sends
            let n = match socket.read(&mut buf).await {
                // socket closed
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Luc! failed to read from socket; err = {:?}", e);
                    return;
                }
            };
            if handle(
                // todo: make a structure for arcs & for command
                str::from_utf8(&buf[0..n]).unwrap().trim_end(), // todo: struct 1
                &ind_arc, // todo: struct 2
                &his_arc, // todo: struct 2
                &srv_addr, // todo: struct 1
                &mut socket, // todo: struct 1
            )
            .await
            {
                println!("Quit server");
            }
        });
    }
}
