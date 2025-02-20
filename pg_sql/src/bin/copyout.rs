use chrono::Local;
use futures::{StreamExt, pin_mut};
use log::info;
use std::sync::OnceLock;
use tokio_postgres::binary_copy::BinaryCopyOutStream;
use tokio_postgres::types::Type;

use connection::create_pool;
use logging::init_logger;
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
    let pool = create_pool(&setting.db).await?;
    let mut conn = pool.get().await?;
    let statement = conn
        .prepare(&format!(
            "copy (select {} from transactions_pool limit 10) to STDOUT BINARY;",
            fields.join(","),
        ))
        .await?;
    let tr = conn.transaction().await?;
    let sink = tr.copy_out(&statement).await?;
    let reader = BinaryCopyOutStream::new(sink, &fields_type);
    pin_mut!(reader);
    while let Some(Ok(row)) = reader.next().await {
        let tag_id: i64 = row.get(0);
        info!("{tag_id:?}");
    }
    tr.commit().await?;

    info!(
        "run time: {}s",
        Local::now().signed_duration_since(now).num_seconds()
    );

    Ok(())
}
