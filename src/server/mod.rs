mod server_impl;
mod commands;
use std::thread::JoinHandle;

pub fn start_server() -> (JoinHandle<()>, String) {
  server_impl::start_server()
}