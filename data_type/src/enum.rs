use serde::Serialize;

#[derive(Serialize, Debug, Copy, Clone)]
pub(crate) enum Gender {
    Female,
    Male,
}

#[derive(Debug)]
pub(crate) enum Gender2 {
    Female(String),
    Male(String),
}

impl Gender {
    pub fn index(&self) -> i32 {
        *self as i32
    }
}
