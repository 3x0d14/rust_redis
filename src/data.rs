use crate::{commands::Set, helpers::get_current_timestamp};
#[derive(Debug, Clone)]
pub struct Val {
    pub val: String,
    pub created_at: u128,
    pub expiry: Option<u128>,
}
impl From<Set> for Val {
    fn from(value: Set) -> Self {
        let created_at = get_current_timestamp();
        let v = Val {
            val: value.val,
            created_at: created_at,
            expiry: value.expiry,
        };
        println!("{:?}", v);
        v
    }
}
