use chrono_tz::Asia::Tokyo;
use serde_json::{Value, json};
use uuid::Uuid;

pub fn make_data() -> Result<Value, Box<dyn std::error::Error>> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use model::TransactionPoolInsert;

    #[test]
    fn generated_data_preserves_the_original_transaction() {
        let data = make_data().unwrap();
        assert_eq!(data.as_object().unwrap().len(), 16);
        assert!(data.get("pay_type").is_none());
        let transaction: TransactionPoolInsert = serde_json::from_value(data).unwrap();
        let original: Value = serde_json::from_str(&transaction.ext_json).unwrap();
        assert_eq!(original["pay_type"], "xxPay");
        assert_eq!(original["trxn_result"], "SUCCESS");
        assert_eq!(original["trxn_type"], "general");
        assert_eq!(original["tag_id"], transaction.tag_id);
        assert!(Uuid::parse_str(&transaction.tag_id).is_ok());
        assert_eq!(transaction.request_time, transaction.success_time);
    }
}
