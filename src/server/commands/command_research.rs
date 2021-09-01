use crate::tools::{hash_command, post};
use std::sync::Arc;
use tokio::sync::Mutex;
use super::common;
use std::vec::Vec;
use std::path::Path;

pub async fn propagate(
  option: &str,
  streams_index: &Arc<Mutex<Vec<String>>>,
  history: &Arc<Mutex<Vec<String>>>,
) {
  // options separated by a unique white space
  // 0. hash, id of the command
  // 1. type(race/all) define if the research should propagate if found
  // 2. ip, owner of the research
  // 3. search key
  // 4. search value
  let words: Vec::<&str> = option.split(' ').collect();
  let history_lock = history.lock().await.to_vec();
  if !history_lock.contains(&words[0].to_string()) {
    let found = if words[3] == "group" && Path::new(&words[4].to_string()).exists() {
      post(format!("have group {}", words[4]), words[2].parse().unwrap()).await;
      true
    } else { false };
    if words[1] == "all" || words[1] == "race" && !found {
      common::propagate("find", option.to_string(), streams_index).await;
    }
  }
}

pub async fn _new(ip: &str,
  search_type: &str,
  key: &str,
  value: &str,
  streams_index: &Arc<Mutex<Vec<String>>>,
  history: &Arc<Mutex<Vec<String>>>,
) {
  let hash = hash_command("research").unwrap_or(String::from("none"));
  history.lock().await.push(hash.to_owned());
  common::propagate("find", format!(
    "find {} {} {} {} {}",
    hash, search_type, ip, key, value
  ), streams_index).await;
}