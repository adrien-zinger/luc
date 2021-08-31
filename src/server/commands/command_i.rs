use crate::tools::{hash_command, remove_prefix};
use std::sync::Arc;
use tokio::sync::Mutex;
use super::common::propagate;

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