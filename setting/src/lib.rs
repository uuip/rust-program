use dotenvy::dotenv;
use serde::Deserialize;

/// usage:
/// ```
/// use setting::Setting;
/// use std::sync::LazyLock;
///
/// static SETTING: LazyLock<Setting, fn() -> Setting> = LazyLock::new(Setting::init);
/// ```
#[derive(Debug, Deserialize)]
pub struct Setting {
    pub explorer_db: Option<String>,
    #[serde(rename = "db_url")]
    pub db: String,
    pub rpc_list: Option<Vec<String>>,
}

impl Setting {
    pub fn init() -> Self {
        dotenv().ok();
        envy::from_env::<Setting>().unwrap()
    }
}

// use config::{Config, Environment};
//
// impl Setting {
//     pub fn init() -> Self {
//         dotenv().ok();
//
//         Config::builder()
//             .add_source(
//                 Environment::default()
//                     .try_parsing(true)
//                     .list_separator(",")
//                     .with_list_parse_key("rpc_list"),
//             )
//             .build().and_then(|c|c.try_deserialize::<Setting>())
//             .unwrap()
//     }
// }
