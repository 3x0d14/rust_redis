use crate::data::{Config, Stream, Val};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub type Memory = Arc<Mutex<HashMap<String, Val>>>;
pub type StreamMemory = Arc<Mutex<HashMap<String, Stream>>>;
pub type AConf = Arc<Mutex<Config>>;
pub type StreamMap = HashMap<String, HashMap<String, Val>>;
