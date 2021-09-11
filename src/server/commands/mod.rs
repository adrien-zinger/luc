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
    if action.name == "index" {
        println!(
            "index: {}",
            format!("{:?}", globals.index.lock().await.to_vec())
        );
    } else if action.name == "luc" {
        insert::insert(content, &globals.index, &globals.history).await;
    } else if action.name == "p" {
        print::print(content, &globals.index, &globals.history).await;
    } else if action.name == "luc?" {
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
    } else if action.name == "connection" {
        command_inter::command_connection(content, &globals.index).await;
        addr::addr(&globals.index).await;
    } else if action.name == "history" {
        println!("{}", format!("{:?}", globals.history.lock().await.to_vec()))
    } else if action.name == "group" {
        group::create(action.option, current_ip).await;
    } else if action.name == "invite" {
        group::invite(action.option).await;
    } else if action.name == "updategroup" {
        group::update(action.option).await;
    } else if action.name == "have" { // response to check or force update
        group::have(action.option).await;
    } else if action.name == "fetch" { // want or check
        group::fetch(action.option).await;
    } else if action.name == "receive" { // receive file from someone
        group::receive(action.option).await;
    } else if action.name == "findprop" {
        find::propagate(action.option, &globals.index, &globals.history).await;
    } else if action.name == "addr" {
        if let Some(ip) = addr::addr(&globals.index).await {
            *globals.addr.lock().await = ip;
        }
    } else if action.name == "raddr" {
        addr::raddr(stream).await;
    }
    action.name == "q"
}
