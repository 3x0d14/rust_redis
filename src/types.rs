use crate::data::{Config, Val};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub type Memory = Arc<Mutex<HashMap<String, Val>>>;
pub type AConf = Arc<Mutex<Config>>;
