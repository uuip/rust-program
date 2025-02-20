use indexmap::{IndexMap, indexmap};
use std::collections::{HashMap, HashSet};

fn map() {
    let m = indexmap! {"s"=>33,"aa"=>44};
    println!("{:?}", m);
    let m = IndexMap::from([("success", 44), ("failed", 55)]);
    println!("{:?}", m);
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
    for item in &set {
        println!("{}", item)
    }

    let mut dict = HashMap::from([("key1", 2), ("key2", 22)]);
    dict.entry("key3").or_insert(222);
    dict.insert("key4", 2222);
    println!("{:?}", dict);
    for (k, v) in &dict {
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
