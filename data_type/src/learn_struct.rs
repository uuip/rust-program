use serde::Serialize;

use crate::learn_enum::Gender;

#[derive(Serialize, Debug)]
pub struct User {
    pub(crate) name: String,
    pub(crate) age: i32,
    pub(crate) gender: Gender,
}

impl User {
    fn query_age(&self) -> i32 {
        self.age
    }
    fn query_gender(&self) -> i32 {
        self.gender.index()
    }
}

pub trait Count {
    fn all(&self) -> i32;
    fn summarize(&self) -> String {
        String::from("事实上")
    }
}

impl Count for User {
    fn all(&self) -> i32 {
        self.age
    }
}

// impl Display for User {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "name: {}",self.name)
//     }
// }

// 两者在使用上没有实质区别，主要是风格选择：
//
//     struct S; 更常见，也是 Rust 社区推荐的风格
//     struct S {} 与常规结构体声明的语法更一致
