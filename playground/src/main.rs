use eyre::{anyhow, Result};
use tokio::task;

#[tokio::main]
async fn main() {
    let tasks: Vec<_> = [1, 2, 3, 4, 5]
        .into_iter()
        .map(|i| task::spawn(async move { doit(i).await }))
        .collect();
    let results = futures::future::join_all(tasks).await;
    for x in results.into_iter().flatten().flatten() {
        println!("{}", x);
    }
}

async fn doit(i: i32) -> Result<i32> {
    if i % 2 == 0 {
        Err(anyhow!("something"))
    } else {
        Ok(i)
    }
}
