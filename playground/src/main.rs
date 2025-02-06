use setting::Setting;
use std::sync::LazyLock;

static SETTING: LazyLock<Setting, fn() -> Setting> = LazyLock::new(Setting::init);

#[tokio::main]
async fn main() {
    println!("{:?}", SETTING.explorer_db)
}
