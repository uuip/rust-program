use chrono::Local;
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Local::now();

    info!(
        "run time: {}s",
        Local::now().signed_duration_since(now).num_seconds()
    );
    Ok(())
}
