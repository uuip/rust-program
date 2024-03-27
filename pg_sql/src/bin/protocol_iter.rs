use futures_util::{pin_mut, TryStreamExt};
use log::{info, warn};
use once_cell::sync::Lazy;
use tokio_postgres::{NoTls, SimpleQueryMessage};

use common::{init_logger, Setting};
static SETTING: Lazy<Setting, fn() -> Setting> = Lazy::new(Setting::init);
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_logger();

    let (client, connection) = tokio_postgres::connect(&SETTING.db, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
            std::process::exit(-1);
        }
    });
    let query = "select * from ship_transfer";
    let mut count = 0;
    info!("run with simple query mode");

    let now = std::time::SystemTime::now();
    let row_stream = client.simple_query_raw(query).await?;
    pin_mut!(row_stream);
    while let Some(m) = row_stream.try_next().await? {
        if let SimpleQueryMessage::Row(r) = m {
            if count == 0 {
                let token_id = r.get("token_id").unwrap();
                info!("token_id: {}", token_id);
                count += 1;
            }
        }
    }
    warn!("用时{:.2?}秒", now.elapsed()?.as_secs_f32());
    Ok(())
}
