#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::thread;

use log::info;

use crate::learn_enum::Gender;
use crate::learn_struct::{Count, User};

mod learn_bytes;
mod learn_enum;
mod learn_random;
mod learn_struct;
mod ndarray_usage;
mod product;

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
    thread::sleep(std::time::Duration::from_secs(0));
    let a = Box::new("33");
    println!("{}", (*a).type_name());

    info!("yyyzzz");
    Ok(())
}

fn learn_collections() {
    let mut set: HashSet<&str> = HashSet::new();
    set.insert("aaa");
    set.insert("aaa");
    println!("{:?}", set);
    let set2: HashSet<&str> = ["bbb", "aaa"].into();
    let rst = set.union(&set2).collect::<Vec<&&str>>();
    println!("并集 {:?}", rst);
    println!("交集 {:?}", set.intersection(&set2).collect::<Vec<&&str>>());
    for item in set.iter() {
        println!("{}", item)
    }

    let mut dict = HashMap::from([("key1", 2), ("key2", 22)]);
    dict.entry("key3").or_insert(222);
    dict.insert("key4", 2222);
    println!("{:?}", dict);
    for (k, v) in dict.iter() {
        println!("{k}: {v}");
    }
}

fn learn_sort() {
    let mut arr = [111, 11, 1];
    arr.sort_unstable();
    println!("{:?}", arr);
    let mut arr = [111.0, 11.0, 1.0];
    arr.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{:?}", arr);
    let mut arr = ["ccc", "bbb", "aaa"];
    arr.sort_unstable();
    println!("{:?}", arr);
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
