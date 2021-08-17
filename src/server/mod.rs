mod server;
mod commands;
use std::thread::JoinHandle;

pub fn start_server() -> (JoinHandle<()>, String) {
  server::start_server()
}