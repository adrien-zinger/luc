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

fn read_group_file(group: &str) -> Vec::<String> {
  let mut ret: Vec::<String> = Vec::new();
  if let Ok(lines) = read_lines(group) {
      for line in lines.flatten() {
          if !line.is_empty() {
              ret.push(line);
          }
      }
  }
  ret
}

fn write_group_file(group: &str, ips: Vec::<String>) {
  let path = Path::new(&group);
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

pub async fn group_create(
  option: &str,
  ip: &str
) {
  // todo: verify if group is valid
  // send in all the network a log that a group
  // has been created with this name
  // Check in a local file group names
  write_group_file(option, vec![ip.to_owned()])
}

pub async fn group_invite(
  option: &str
) {
  let command: Vec<&str> = option.split(' ').collect();
  let ips = read_group_file(command[0]);
  let mut new_ips: Vec<String> = ips.clone();
  for ip in command.iter().skip(1) {
    new_ips.push(ip.to_string());
  }
  let content = format!("update {}\n{}", command[0], new_ips.join("\n"));
  for ip in ips {
    // todo multiple send
    post(content.to_owned(), ip.parse().unwrap()).await;
  }
}

pub async fn group_update(
  option: &str,
) {
  let command: Vec<&str> = option.split('\n').collect();
  let mut ips: Vec<String> = Vec::new();
  for ip in command[1..].iter() {
    ips.push(ip.to_string());
  }
  write_group_file(command[0], ips);
}

pub async fn _command_quit(
  _option: &str,
) {
  // 1. Delete my group file
  // 2. Propagate in the group an updated list without me
}
