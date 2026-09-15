use chrono::Local;
use futures::pin_mut;
use log::info;
use std::sync::OnceLock;
use tokio_postgres::binary_copy::BinaryCopyInWriter;
use tokio_postgres::types::{ToSql, Type};

use connection::create_pool;
use logging::init_logger;
use model::TransactionPoolInsert;
use pg_sql::make_data;
use setting::Setting;

static SETTING: OnceLock<Setting> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let setting = SETTING.get_or_init(Setting::init);
    init_logger();
    let fields = [
        "block_number",
        "coin_code",
        "ext_json",
        "fail_reason",
        "from_user_id",
        "gas",
        "gen_time",
        "nonce",
        "point",
        "request_time",
        "status_code",
        "store_id",
        "success_time",
        "tag_id",
        "to_user_id",
        "tx_hash",
    ];
    let fields_type = [
        Type::INT8,
        Type::TEXT,
        Type::TEXT,
        Type::TEXT,
        Type::TEXT,
        Type::INT8,
        Type::TEXT,
        Type::INT8,
        Type::FLOAT8,
        Type::TIMESTAMPTZ,
        Type::INT4,
        Type::TEXT,
        Type::TIMESTAMPTZ,
        Type::TEXT,
        Type::TEXT,
        Type::TEXT,
    ];

    let now = Local::now();
    let pool = create_pool(&setting.db)?;
    let mut conn = pool.get().await?;
    let statement = conn
        .prepare(&format!(
            "copy transactions_pool({}) FROM STDIN BINARY;",
            fields.join(","),
        ))
        .await?;
    let tr = conn.transaction().await?;
    // Sink 是异步写入端：程序向它发送字节，PostgreSQL 通过 COPY FROM STDIN 接收。
    // writer 负责把每行的 Rust 字段编码成 COPY 二进制格式，再交给 sink。
    let sink = tr.copy_in(&statement).await?;
    let writer = BinaryCopyInWriter::new(sink, &fields_type);
    // pin_mut! 固定底层 writer 的位置，并用同名的 Pin<&mut _> 包装它，
    // 满足异步 write/finish 接口对固定可变引用的要求；它不会启动任务或加锁。
    pin_mut!(writer);

    for _ in 0..10 {
        let data = make_data()?;
        let m: TransactionPoolInsert = serde_json::from_value(data).unwrap();
        let params = [
            &m.block_number as &(dyn ToSql + Sync),
            &m.coin_code,
            &m.ext_json,
            &m.fail_reason,
            &m.from_user_id,
            &m.gas,
            &m.gen_time,
            &m.nonce,
            &m.point,
            &m.request_time,
            &m.status_code,
            &m.store_id,
            &m.success_time,
            &m.tag_id,
            &m.to_user_id,
            &m.tx_hash,
        ];
        // as_mut() 临时重新借用固定引用，让下一轮仍能使用 writer。
        // write 一次编码一行；内部已有缓冲，不是每写一行就等待数据库确认。
        writer.as_mut().write(&params).await?;
    }
    // 发送剩余缓冲和 COPY 结束标记，等待数据库确认；不能省略。
    writer.finish().await?;
    tr.commit().await?;

    info!(
        "run time: {}s",
        Local::now().signed_duration_since(now).num_seconds()
    );

    Ok(())
}
