use futures::{pin_mut, TryStreamExt};
use log::{info, warn};
use std::sync::OnceLock;
use tokio_postgres::types::ToSql;
use tokio_postgres::NoTls;

use logging::init_logger;
use setting::Setting;

static SETTING: OnceLock<Setting> = OnceLock::new();
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let setting = SETTING.get_or_init(Setting::init);
    init_logger();

    let (client, connection) = tokio_postgres::connect(&setting.db, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let params: [&(dyn ToSql + Sync); 0] = [];
    let mut count = 0;

    let now = std::time::SystemTime::now();
    let rst = client
        .query_raw("select * from ship_transfer", params)
        .await?;
    pin_mut!(rst);
    while let Some(row) = rst.try_next().await? {
        if count == 0 {
            let token_id: i32 = row.get("token_id");
            info!("{:?}", token_id);
            count += 1;
        }
    }
    warn!("用时{:.2?}秒", now.elapsed()?.as_secs_f32());
    Ok(())
}
