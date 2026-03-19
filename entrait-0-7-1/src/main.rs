mod app_state;
mod bootstrap;
mod config;
mod domain;
mod error;
mod handler;
mod infrastructure;
mod interface;
mod service;
#[cfg(test)]
mod test_support;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    bootstrap::run().await
}
