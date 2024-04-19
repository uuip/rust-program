use chrono::Local;
use futures_util::{pin_mut, StreamExt};
use log::info;
use once_cell::sync::Lazy;

use common::model::{FromRow, StatusCode, TransactionPool};
use common::{create_pool, init_logger, Setting};

static SETTING: Lazy<Setting, fn() -> Setting> = Lazy::new(Setting::init);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();

    let pool = create_pool(&SETTING.explorer_db).await;
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
