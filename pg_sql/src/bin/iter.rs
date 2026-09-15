//! 逐行读取用 `query_raw`；需要控制服务端每批返回行数时，用事务内的 `query_portal_raw`。
//! 两者均返回行流，通过 `try_next().await?` 读取并向调用方返回错误。

use chrono::Local;
use futures::{TryStreamExt, pin_mut};
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
    // Portal 保存本次查询的参数和执行进度，只能在创建它的连接、事务中使用。
    let portal = tr.bind(&statement, &[&StatusCode::Success]).await?;
    loop {
        let max_rows = 2;
        // 继续同一个 Portal，每次最多返回 max_rows 行，不会从头重新执行查询。
        // max_rows 必须为正数才限制批次大小；0 或负数表示不限制返回行数。
        let rows = tr.query_portal_raw(&portal, max_rows).await?;
        let mut count = 0;
        // RowStream 是 !Unpin，不能直接调用要求 Unpin 的 try_next()。
        // pin_mut! 保持底层流的位置不变，并生成满足接口约束的 Pin<&mut _>。
        pin_mut!(rows);
        while let Some(row) = rows.try_next().await? {
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
    // query_raw 发起一次新查询，返回整个查询的行流；逐行消费，但没有服务端分批上限。
    // 它无需显式创建事务；与 query_portal_raw 一样返回流，不会先收集成 Vec<Row>。
    let rows = conn.query_raw(&statement, params).await?;
    pin_mut!(rows);
    while let Some(row) = rows.try_next().await? {
        let o: TransactionPool = TransactionPool::from_row(&row);
        info!("{}", o.status_code);
    }

    info!(
        "run time: {}s",
        Local::now().signed_duration_since(now).num_seconds()
    );
    Ok(())
}
