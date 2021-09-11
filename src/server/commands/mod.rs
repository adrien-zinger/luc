mod addr;
mod command_inter;
mod command_parser;
mod common;
mod find;
mod group;
mod insert;
mod print;

use crate::server::Globals;
use command_parser::parse_command;
use tokio::net::TcpStream;

// Todo: handle command in a separeted file
pub async fn handle(content: &str, stream: &mut TcpStream, globals: &Globals) -> bool {
    let act = parse_command(content);
    if act.is_none() {
        eprintln!("Luc! Error input ---> \n{}", content);
        return false;
    }
    let action = act.unwrap();
    let current_ip = &globals.addr.lock().await.to_owned();
    match action.name {
        "index" => println!("index: {}", format!("{:?}", globals.index.lock().await.to_vec())),
        "luc" => insert::insert(content, &globals.index, &globals.history).await,
        "p" => print::print(content, &globals.index, &globals.history).await,
        "luc?" => {
            command_inter::commands_luc_inter(
                action.option,
                content,
                &globals.index,
                &globals.history,
                &globals.addr.lock().await.to_owned(),
                stream,
            ) // todo send less objects
            .await;
            addr::addr(&globals.index).await;
        }
        "connection" => {
            command_inter::command_connection(content, &globals.index).await;
            addr::addr(&globals.index).await;
        }
        "history" => println!("{}", format!("{:?}", globals.history.lock().await.to_vec())),
        "group" => group::create(action.option, current_ip).await,
        "invite" => group::invite(action.option).await,
        "updategroup" => group::update(action.option).await,
        "have" => group::have(action.option).await, // response to check or force update
        "fetch" => group::fetch(action.option).await, // want or check
        "receive" => group::receive(action.option).await, // receive file from someone
        "findprop" => find::propagate(action.option, &globals.index, &globals.history).await,
        "addr" => if let Some(ip) = addr::addr(&globals.index).await { // require my ip address
            *globals.addr.lock().await = ip;
        }
        "raddr" => addr::raddr(stream).await, // return the ip of the client
        _ => {}
    };
    action.name == "q"
}
