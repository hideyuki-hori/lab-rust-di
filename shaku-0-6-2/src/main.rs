mod config;
mod domain;
mod error;
mod handler;
mod infrastructure;
mod interface;
mod module;
mod service;
mod bootstrap;
#[cfg(test)]
mod test_support;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    bootstrap::run().await
}
