/// 对于Option字段，使用值时：setting.explorer_db.as_ref().unwrap()
#[derive(Debug)]
pub struct Setting {
    pub pulsar_addr: Option<String>,
    pub topic: Option<String>,
    pub explorer_db: Option<String>,
    pub db: String,
    pub rpc_list: Option<Vec<String>>,
}

pub fn get_env(key: &str) -> Option<String> {
    dotenvy::var(key).ok()
}

impl Setting {
    pub fn init() -> Self {
        let pulsar_addr = get_env("PULSAR_URL");
        let topic = get_env("PULSAR_TOPIC");
        let explorer_db = get_env("EXPLORER_DB");
        let db = get_env("DB_URL").expect("DB_URL not set");
        let rpc_list =
            get_env("rpc_list").map(|s| s.split(',').map(str::to_string).collect::<Vec<String>>());

        Self {
            pulsar_addr,
            topic,
            explorer_db,
            db,
            rpc_list,
        }
    }
}
