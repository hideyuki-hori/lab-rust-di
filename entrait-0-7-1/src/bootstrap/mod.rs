mod connections;
mod router;

use std::sync::Arc;

use entrait::Impl;
use tokio::net::TcpListener;

use crate::app_state::AppState;
use crate::config::AppConfig;
use crate::infrastructure;

use connections::Connections;

pub async fn run() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let config = AppConfig::from_env()?;

    let mut connections = Connections::establish(&config).await?;
    infrastructure::seed::seed_master_data(&mut connections.redis_conn).await?;

    let nats_client = connections.nats_client.clone();
    let app = Arc::new(Impl::new(AppState {
        db_pool: connections.db_pool,
        redis_conn: connections.redis_conn,
        nats_client: connections.nats_client,
    }));

    infrastructure::nats_order_subscriber::spawn(nats_client, app.clone());

    let router = router::create(app);

    let addr = format!("0.0.0.0:{}", config.server_port);
    tracing::info!("Starting server on {addr}");
    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
