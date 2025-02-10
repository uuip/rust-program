use std::thread::sleep;
use std::time::Duration;

use rocksdb::{DBCompactionStyle, DBCompressionType, Options, DB};
use serde_json::json;
use uuid::Uuid;

fn main() {
    let mut options = Options::default();
    options.create_if_missing(true);
    options.set_compaction_style(DBCompactionStyle::Level);
    options.set_compression_type(DBCompressionType::Lz4);
    options.set_level_compaction_dynamic_level_bytes(true);
    options.set_write_buffer_size(128 * 1024 * 1024);
    options.set_max_write_buffer_number(6);
    options.set_target_file_size_base(128 * 1024 * 1024);
    options.set_max_background_jobs(8);

    let db = DB::open(&options, "./kvstore").unwrap();

    for _ in 0..1_0000_i32 {
        let v = json!({"block":fastrand::i32(..)});
        let v_bytes = serde_json::to_vec(&v).unwrap();
        db.put(Uuid::new_v4(), v_bytes).unwrap();
    }
    sleep(Duration::from_secs(60));
}
