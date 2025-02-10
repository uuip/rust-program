use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use rust_decimal::prelude::*;

fn main() -> anyhow::Result<()> {
    let d = BigDecimal::from(2);
    println!("{}", d);

    let d = Decimal::from_f32(3.22).unwrap();
    println!("{}", d);
    println!("{}", Decimal::MAX);

    let d = BigInt::from(11111111111_i64);
    println!("{}", d);
    Ok(())
}
