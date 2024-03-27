use futures_util::{pin_mut, TryStreamExt};
use log::{info, warn};
use once_cell::sync::Lazy;
use tokio_postgres::types::ToSql;
use tokio_postgres::NoTls;

use common::{init_logger, Setting};
static SETTING: Lazy<Setting, fn() -> Setting> = Lazy::new(Setting::init);
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_logger();

    let (client, connection) = tokio_postgres::connect(&SETTING.db, NoTls).await?;
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
