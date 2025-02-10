#![allow(dead_code)]

use log::info;
use std::net::Ipv4Addr;
use std::str::FromStr;

use crate::r#enum::Gender;
use crate::r#struct::{Count, User};

mod bytes;
mod collections;
mod r#enum;
mod ndarray;
mod number;
mod path;
mod product;
mod random;
mod r#struct;

#[allow(clippy::nonminimal_bool)]
fn main() -> anyhow::Result<()> {
    learn_str2num().expect("TODO: panic message");

    let user1 = &User {
        name: "张某某".to_string(),
        age: 20,
        gender: Gender::Male,
    };
    let data1: serde_json::Value = serde_json::to_value(user1)?;

    println!("{}", serde_json::to_string(&data1)?);
    println!("{:?}", user1.gender.index());
    println!("{:?}", user1.summarize());
    println!("{:?}", user1);

    println!("{}", true && false);
    let a = Box::new("33");
    println!("{}", (*a).type_name());

    info!("yyyzzz");
    Ok(())
}

fn learn_concat() {
    let mut a = String::from("aaaa");
    let b = String::from("bbbb");
    println!("{}", a.clone() + "333");
    a += &b;
    let somestr = format!("{a}{b}");
    println!("{}", somestr);
}

fn learn_str2num() -> anyhow::Result<()> {
    let i1 = 8999999_i64.to_string();
    let s1 = String::from("456");
    let i2: i64 = s1.parse()?;
    let i3 = i64::from_str("456")?;
    println!("{i1} {i2} {i3}");

    let f1 = 100.20.to_string();
    let s1 = String::from("456.360");
    let f2: f32 = s1.parse()?;
    let f3 = f32::from_str(&s1)?;
    let f4 = i3 as f32;
    println!("{f1} {f2} {} {}", f3.type_name(), f4.type_name());
    Ok(())
}

fn learn_ipnetwork() -> anyhow::Result<()> {
    let ip: Ipv4Addr = "1.2.3.4".parse()?;
    let ip1 = "1.2.3.4".parse::<Ipv4Addr>()?;
    let ip2 = Ipv4Addr::from_str("1.2.3.4")?;
    println!("{} {} {}", ip, ip1, ip2);
    Ok(())
}

pub trait AnyExt {
    fn type_name(&self) -> &'static str;
}

impl<T> AnyExt for T {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}
