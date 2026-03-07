mod connections;
mod module_builder;
mod router;

use tokio::net::TcpListener;

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
    let modules = module_builder::build(connections);

    infrastructure::nats_order_subscriber::spawn(nats_client, modules.infra);

    let app = router::create(modules.app);

    let addr = format!("0.0.0.0:{}", config.server_port);
    tracing::info!("Starting server on {addr}");
    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
