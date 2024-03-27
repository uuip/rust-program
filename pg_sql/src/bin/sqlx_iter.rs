#![allow(non_snake_case)]

use futures_util::TryStreamExt;
use log::{info, warn};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPoolOptions, Postgres};
use sqlx::FromRow;

use common::{init_logger, Setting};
static SETTING: Lazy<Setting, fn() -> Setting> = Lazy::new(Setting::init);
#[derive(Clone, Debug, Deserialize, Serialize, FromRow)]
pub struct Transaction {
    pub id: i64,
    pub transactionHash: Option<String>,
    pub logIndex: Option<i32>,
    pub event: String,
    pub transactionIndex: Option<i32>,
    pub blockNumber: Option<i32>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub token_id: Option<i32>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_logger();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .min_connections(1)
        .connect(&SETTING.db)
        .await?;

    info!("run with sqlx");
    let mut count = 0;
    let now = std::time::SystemTime::now();
    let mut rows =
        sqlx::query_as::<Postgres, Transaction>("select * from ship_transfer").fetch(&pool);
    while let Some(row) = rows.try_next().await? {
        if count == 0 {
            info!("{:?}", row.token_id.unwrap());
            count += 1;
        }
    }
    warn!("用时{:.2?}秒", now.elapsed()?.as_secs_f32());
    Ok(())
}
