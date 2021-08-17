mod commands;
mod server_impl;

pub async fn start_server_thread(port: &str) -> Result<(), Box<dyn std::error::Error>> {
    server_impl::start_server(port).await
}
