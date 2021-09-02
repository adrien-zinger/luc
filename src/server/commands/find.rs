use super::common;
use crate::tools::{hash_command, post};
use std::path::Path;
use std::sync::Arc;
use std::vec::Vec;
use tokio::sync::Mutex;

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
    // 5. optional lines to execute
    // TODO: split lines, execute lines if authorized the command is authorized in
    //       current the peer when found
    let words: Vec<&str> = option.split(' ').collect();
    let hash = &words[0].to_string();
    let done = history.lock().await.to_vec().contains(hash);
    if !done {
        let found = if words[3] == "group" && Path::new(&words[4].to_string()).exists() {
            post(
                &format!("have {} {}", hash, words[4]),
                words[2].parse().unwrap(),
            )
            .await;
            true
        } else {
            false
        };
        if words[1] == "all" || words[1] == "race" && !found {
            common::propagate("findprop", option.to_string(), streams_index).await;
        }
    }
}

pub async fn _new(
    ip: &str,
    search_type: &str,
    key: &str,
    value: &str,
    streams_index: &Arc<Mutex<Vec<String>>>,
    history: &Arc<Mutex<Vec<String>>>,
) -> Option<String> {
    if let Some(hash) = hash_command("research") {
        history.lock().await.push(hash.to_owned());
        common::propagate(
            "find",
            format!("findprop {} {} {} {} {}", hash, search_type, ip, key, value),
            streams_index,
        )
        .await;
        return Some(hash);
    }
    None
}
