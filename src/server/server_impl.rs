use super::commands::*;
use regex::Regex;
use std::net::SocketAddr;
use std::vec::Vec;

use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};

use std::str;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Copy)]
struct Command<'a> {
    name: &'a str,
    option: &'a str,
}

fn parse_command(input: &'_ str) -> Option<Command<'_>> {
    let re: Regex = Regex::new(r"^([a-z\?]+)[ ]*(.*?)$").unwrap();
    let caps = re.captures_iter(input).filter_map(|cap| {
        Some(Command {
            name: match cap.get(1) {
                Some(name) => name.as_str(),
                _ => {
                    return None;
                }
            },
            option: match cap.get(2) {
                Some(option) => option.as_str(),
                _ => "",
            },
        })
    });
    let vec = caps.collect::<Vec<Command>>();
    if vec.len() == 1 {
        Some(vec[0])
    } else {
        None
    }
}

async fn handle(
    content: &str,
    streams_index: &Arc<Mutex<Vec<String>>>,
    history: &Arc<Mutex<Vec<String>>>,
    srv_addr: &SocketAddr,
    stream: &mut TcpStream,
) -> bool {
    let act = parse_command(content);
    if act.is_none() {
        eprintln!("Luc! Error input ---> {}", content);
        return true;
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
            srv_addr,
            stream,
        )
        .await
    } else if action.name == "connection" {
        command_connection(content, streams_index).await;
    } else if action.name == "history" {
        println!("{}", format!("{:?}", history.lock().await.to_vec()))
    }
    action.name == "q"
}

pub async fn start_server(port: &str) -> Result<(), Box<dyn std::error::Error>> {
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
            let n = match socket.read(&mut buf).await {
                // socket closed
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    return;
                }
            };

            if handle(
                str::from_utf8(&buf[0..n]).unwrap().trim_end(),
                &ind_arc,
                &his_arc,
                &srv_addr,
                &mut socket,
            )
            .await
            {
                println!("Quit server");
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::parse_command;

    #[test]
    fn test_empty() {
        let command = parse_command("");
        assert!(command.is_none());
    }

    #[test]
    fn test_str_command() {
        let command = parse_command("luc");
        assert!(command.is_some());
        assert_eq!(command.unwrap().name, "luc");
        assert_eq!(command.unwrap().option, "");
    }

    #[test]
    fn test_str_command_inter() {
        let command = parse_command("luc?");
        assert!(command.is_some());
        assert_eq!(command.unwrap().name, "luc?");
        assert_eq!(command.unwrap().option, "");
    }

    #[test]
    fn test_str_command_and_option() {
        let command = parse_command("luc? opt opt ! opt ?");
        assert!(command.is_some());
        assert_eq!(command.unwrap().name, "luc?");
        assert_eq!(command.unwrap().option, "opt opt ! opt ?");
    }
}
