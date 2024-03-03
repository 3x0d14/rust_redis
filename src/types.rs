use crate::data::Val;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub type Memory = Arc<Mutex<HashMap<String, Val>>>;
