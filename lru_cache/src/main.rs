use cached::cached;

#[derive(thiserror::Error, Debug, Clone)]
#[error("uuiiop0")]
struct APIError;

fn main() {
    (0..20).for_each(|_| {
        let _ = keyed("aaabbb".into());
    })
}

#[cached(ttl_secs = 100)]
fn keyed(a: String) -> Result<usize, APIError> {
    println!("{}", a);
    if a == "a" { Ok(a.len()) } else { Err(APIError) }
}
