//! 不使用连接池，直接连接 PostgreSQL，并在后台驱动连接。
//! 通过 `query_raw` 获取行流，使用 `try_next().await?` 逐行读取并返回错误。

use futures::{TryStreamExt, pin_mut};
use log::{info, warn};
use std::sync::OnceLock;
use tokio_postgres::NoTls;
use tokio_postgres::types::ToSql;

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
    // RowStream 是 !Unpin，直接调用要求 Unpin 的 try_next() 会编译失败。
    // pin_mut! 固定底层流并生成 Pin<&mut _>，使引用满足接口约束。
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
