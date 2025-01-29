use chrono::Local;
use futures::{pin_mut, StreamExt};
use log::info;
use std::sync::OnceLock;

use connection::create_pool;
use logging::init_logger;
use model::{FromRow, StatusCode, TransactionPool};
use setting::Setting;

static SETTING: OnceLock<Setting> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let setting = SETTING.get_or_init(Setting::init);
    init_logger();

    let pool = create_pool(setting.explorer_db.as_ref().unwrap()).await?;
    let mut conn = pool.get().await?;
    let statement = conn
        .prepare("SELECT * FROM transactions_pool where status_code=$1 ORDER BY created_at;")
        .await?;

    let now = Local::now();
    let tr = conn.transaction().await?;
    let portal = tr.bind(&statement, &[&StatusCode::Success]).await?;
    loop {
        let max_rows = 2;
        let rst = tr.query_portal_raw(&portal, max_rows).await;
        match rst {
            Ok(rows) => {
                let mut count = 0;
                pin_mut!(rows);
                while let Some(Ok(row)) = rows.next().await {
                    count += 1;
                    let o: TransactionPool = TransactionPool::from_row(&row);
                    info!("{}", o.tag_id);
                }
                if count < max_rows {
                    break;
                }
            }
            Err(_) => break,
        }
    }
    tr.commit().await?;
    info!(
        "run time: {}s",
        Local::now().signed_duration_since(now).num_seconds()
    );

    let now = Local::now();
    let params = &[&StatusCode::Success];
    let rows = conn.query_raw(&statement, params).await?;
    pin_mut!(rows);
    while let Some(Ok(row)) = rows.next().await {
        let o: TransactionPool = TransactionPool::from_row(&row);
        info!("{}", o.status_code);
    }

    info!(
        "run time: {}s",
        Local::now().signed_duration_since(now).num_seconds()
    );
    Ok(())
}
