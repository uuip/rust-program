use log::{info, warn};
use once_cell::sync::Lazy;
use tokio_postgres::NoTls;

use common::{init_logger, Setting};
static SETTING: Lazy<Setting, fn() -> Setting> = Lazy::new(Setting::init);
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_logger();

    let (mut client, connection) = tokio_postgres::connect(&SETTING.db, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let query = "select * from ship_transfer";
    let cursor_name = "iterquery";
    let stmt = format!("DECLARE {cursor_name} NO SCROLL CURSOR WITHOUT HOLD FOR {query}");
    info!("run with server side cursor");

    let now = std::time::SystemTime::now();
    let tr = client.build_transaction().start().await?;
    let _ = tr.execute(&stmt, &[]).await;
    loop {
        let rows = tr.query("FETCH 1000 FROM iterquery", &[]).await?;
        if rows.is_empty() {
            break;
        }
        for _ in rows {}
    }
    let _ = tr.execute(&format!("close {cursor_name}"), &[]).await;
    let _ = tr.commit().await;
    warn!("用时{:.2?}秒", now.elapsed()?.as_secs_f32());
    Ok(())
}
