use chrono::Local;
use chrono_tz::Asia::Tokyo;
use futures_util::future;
use log::info;
use once_cell::sync::Lazy;
use serde_json::{json, Value};
use tokio_postgres::types::ToSql;
use uuid::Uuid;

use common::model::TransactionPoolInsert;
use common::{create_pool, init_logger, Setting};

static SETTING: Lazy<Setting, fn() -> Setting> = Lazy::new(Setting::init);

fn make_data() -> Result<Value, Box<dyn std::error::Error>> {
    let t = chrono::Utc::now().with_timezone(&Tokyo);

    let mut data = json!({
        "coin_code": "JPY",
        "pay_type": "xxPay",
        "trxn_result": "SUCCESS",
        "trxn_type": "general",
        "store_id": "devtest",
        "point": 10.0,
        "from_user_id": "CPM1696751455",
        "to_user_id": "920MH0OFY6c",
        "tag_id": Uuid::new_v4().to_string(),
        "gen_time": t.format("%Y-%m-%d %H:%M:%S%:z").to_string(),
    });
    data["ext_json"] = serde_json::to_value(data.to_string())?;

    let data2 = json!({
        "tx_hash": "0x77babc8124b64c6556976c847a16590600135307f1ba4cc0d2d1a7e98a55b230",
        "gas": 50000,
        "nonce": 50,
        "fail_reason": None::<String>,
        "status_code": 200,
        "block_number": 1007334,
        "success_time": t,
        "request_time": t,
    });
    let data_obj = data.as_object_mut().unwrap();
    data_obj.remove("trxn_result");
    data_obj.remove("trxn_type");
    data_obj.remove("pay_type");
    for (k, v) in data2.as_object().unwrap() {
        data_obj.insert(k.to_owned(), v.to_owned());
    }
    Ok(serde_json::to_value(data_obj)?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let pool = create_pool(&SETTING.db).await;
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
