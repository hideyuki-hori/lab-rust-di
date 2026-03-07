use redis::aio::ConnectionManager;
use redis::AsyncCommands;

pub async fn seed_master_data(conn: &mut ConnectionManager) -> anyhow::Result<()> {
    let defaults = [
        ("master:tax_rate", "0.10"),
        ("master:shipping_fee", "500"),
        ("master:max_order_quantity", "99"),
    ];
    for (key, value) in defaults {
        let exists: bool = redis::cmd("EXISTS")
            .arg(key)
            .query_async(&mut *conn)
            .await?;
        if !exists {
            conn.set::<_, _, ()>(key, value).await?;
            tracing::info!("Seeded master data: {key} = {value}");
        }
    }
    Ok(())
}
