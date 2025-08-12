use cached::proc_macro::cached;
use std::time::Duration;

#[derive(thiserror::Error, Debug, Clone)]
#[error("uuiiop0")]
struct APIError;

fn main() {
    (0..20).for_each(|_| {
        let _ = keyed("aaabbb".into());
    })
}

#[cached(time = 10, result = false)]
fn keyed(a: String) -> Result<usize, APIError> {
    println!("{}", a);
    if a == "a" { Ok(a.len()) } else { Err(APIError) }
}
