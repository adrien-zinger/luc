// Todo will need to lock other peer lists when the
// synchronisation will be implemented
use crate::tools::post;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn read_group_file(group: &str) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    let file_path = String::from(group) + ".group";
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.flatten() {
            if !line.is_empty() {
                ret.push(line);
            }
        }
    }
    ret
}

fn write_group_file(group: &str, ips: Vec<String>) {
    let file_path = String::from(group) + ".group";
    let path = Path::new(&file_path);
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    let mut out = String::new();
    for ip in ips {
        out += &ip;
        out.push('\n');
    }
    if let Err(why) = file.write_all(out.as_bytes()) {
        panic!("couldn't write to {}: {}", display, why)
    }
}

pub async fn create(option: &str, ip: &str) {
    // todo: verify if group is valid
    // send in all the network a log that a group
    // has been created with this name
    // Check in a local file group names
    write_group_file(option, vec![ip.to_owned()])
}

pub async fn invite(option: &str) {
    let command: Vec<&str> = option.split(' ').collect();
    let mut ips = read_group_file(command[0]);
    for ip in command.iter().skip(1) {
        let ip = ip.to_string();
        if ips.contains(&ip) {
            return println!("Luc! Luc is already in the group");
        }
        ips.push(ip);
    }
    let content = format!("updategroup {}\n{}", command[0], ips.join("\n"));
    for ip in ips.iter() {
        // todo multiple send
        post(&content, ip.parse().unwrap()).await;
    }
}

pub async fn update(option: &str) {
    let command: Vec<&str> = option.split('\n').collect();
    let mut ips: Vec<String> = Vec::new();
    for ip in command[1..].iter() {
        ips.push(ip.to_string());
    }
    write_group_file(command[0], ips);
}

pub async fn _quit(_option: &str) {
    // 1. Delete my group file
    // 2. Propagate in the group an updated list without me
}
