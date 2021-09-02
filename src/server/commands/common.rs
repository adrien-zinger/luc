use crate::post;
use std::sync::Arc;
use tokio::sync::Mutex;

/// todo:
/// Propagate a command with another strategy ?
pub async fn propagate(command: &str, options: String, streams_index: &Arc<Mutex<Vec<String>>>) {
    for stream in streams_index.lock().await.to_vec() {
        if let Ok(addr) = stream.parse::<std::net::SocketAddr>() {
            post(&(command.to_owned() + &options[..]), addr).await;
        } else {
            eprintln!("Error ! parse stream '{}' failed", stream);
        }
    }
}
