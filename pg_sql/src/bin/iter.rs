//! 演示 `tokio-postgres` 迭代查询结果的两种方式。
//!
//! 第一种方式在事务中将预处理语句绑定到 Portal，并重复调用带行数上限的
//! `query_portal_raw`。每次调用以流的方式消费一个受限批次，直到当前批次的
//! 行数少于请求上限。第二种方式调用 `query_raw`，并通过 `StreamExt::next`
//! 直接消费返回的异步行流。
//!
//! `query_raw` 不会先把完整结果集加载到应用程序内存后再进行迭代。它返回
//! `RowStream`，驱动从 PostgreSQL 接收到行后逐行产出。相比之下，`query`
//! 是一个便捷封装，它通过 `try_collect` 耗尽同一个流，并将所有行放入
//! `Vec<Row>` 后返回。`query_raw` 不会限制服务端单次执行返回的行数；如果
//! 必须将每次服务端执行限制为固定批次，应使用 Portal 实现。

use chrono::Local;
use futures::{StreamExt, pin_mut};
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

    let pool = create_pool(setting.explorer_db.as_ref().unwrap())?;
    let mut conn = pool.get().await?;
    let statement = conn
        .prepare("SELECT * FROM transactions_pool where status_code=$1 ORDER BY created_at;")
        .await?;

    let now = Local::now();
    let tr = conn.transaction().await?;
    let portal = tr.bind(&statement, &[&StatusCode::Success]).await?;
    loop {
        let max_rows = 2;
        let rows = tr.query_portal_raw(&portal, max_rows).await?;
        let mut count = 0;
        pin_mut!(rows);
        while let Some(row) = rows.next().await {
            let row = row?;
            count += 1;
            let o: TransactionPool = TransactionPool::from_row(&row);
            info!("{}", o.tag_id);
        }
        if count < max_rows {
            break;
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
