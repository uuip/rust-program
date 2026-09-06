use chrono::Local;
use futures::future;
use log::info;
use std::sync::OnceLock;
use tokio_postgres::types::ToSql;

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
    let mansy_objs = (0..10)
        .map(|_| {
            let data = make_data().unwrap();
            serde_json::from_value(data).unwrap()
        })
        .collect::<Vec<TransactionPoolInsert>>();

    // let fields:Vec<String>= data.as_object().unwrap().iter().map(|(k,v)| k.to_owned()).collect();
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
    let values_placehold = (1..=fields.len())
        .map(|x| format!("${x}"))
        .collect::<Vec<String>>()
        .join(",");

    let pool = create_pool(&setting.db)?;
    let mut conn = pool.get().await?;

    let tr = conn.transaction().await?;
    let statement = tr
        .prepare(&format!(
            "INSERT INTO transactions_pool({}) values({});",
            fields.join(","),
            values_placehold
        ))
        .await?;
    let tasks = mansy_objs.iter().map(|m| {
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
        tr.execute_raw(&statement, params)
    });
    let now = Local::now();
    let _ = future::try_join_all(tasks).await;
    tr.commit().await?;

    info!(
        "run time: {}s",
        Local::now().signed_duration_since(now).num_seconds()
    );
    Ok(())
}
