mod connections;
mod router;

use std::sync::Arc;

use tokio::net::TcpListener;

use crate::config::AppConfig;
use crate::infrastructure;
use crate::provider::AppProvider;

use connections::Connections;

pub async fn run() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let config = AppConfig::from_env()?;

    let connections = Connections::establish(&config).await?;
    infrastructure::seed::seed_master_data(&connections.redis_connection).await?;

    let nats_client = connections.nats_client.clone();

    let provider = Arc::new(AppProvider::new(
        connections.db_pool,
        connections.redis_connection,
        connections.nats_client,
    ));

    crate::subscriber::nats_order_subscriber::spawn(nats_client, Arc::clone(&provider));

    let app = router::create(Arc::clone(&provider));

    let addr = format!("0.0.0.0:{}", config.server_port);
    tracing::info!("Starting server on {addr}");
    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
