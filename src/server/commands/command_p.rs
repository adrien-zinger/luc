use crate::tools::get_first_n_words;
use std::sync::Arc;
use tokio::sync::Mutex;
use super::common::propagate;

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