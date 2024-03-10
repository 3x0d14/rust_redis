use std::{collections::HashMap, io::Write, net::TcpStream};

use crate::{
    commands::{Set, XA},
    helpers::{get_current_timestamp, resp_command},
    types::StreamMap,
};
#[derive(Debug, Clone)]
pub enum ValType {
    STRING,
    STREAM,
}
#[derive(Debug, Clone)]
pub struct Val {
    pub val: String,
    pub created_at: u128,
    pub expiry: Option<u128>,
    pub value_type: ValType,
}
impl From<Set> for Val {
    fn from(value: Set) -> Self {
        let created_at = get_current_timestamp();
        let v = Val {
            val: value.val,
            created_at: created_at,
            expiry: value.expiry,
            value_type: ValType::STRING,
        };

        v
    }
}
#[derive(Clone, Debug)]
pub struct Stream {
    pub map: StreamMap,
    pub created_at: u128,
    pub top: Vec<u128>,
}
impl From<XA> for Stream {
    fn from(value: XA) -> Self {
        let created_at = get_current_timestamp();
        let mut map = HashMap::new();
        let mut key_vals_map = HashMap::new();
        let id = value.id;
        let top = id
            .split("-")
            .map(|a| a.parse::<u128>().unwrap())
            .collect::<Vec<u128>>();
        let key_vals = value.key_vals;
        for i in (0..key_vals.len()).step_by(2) {
            let val = Val {
                created_at: created_at,
                val: key_vals[i + 1].clone(),
                expiry: None,
                value_type: ValType::STRING,
            };
            println!("val : {:?}\n\n", val);
            let key = key_vals[i].clone();
            key_vals_map.insert(key, val);
        }
        println!("KEY VAL MAP : {:?}", key_vals_map);
        map.insert(id, key_vals_map);
        let v = Stream {
            top: top,
            map: map,
            created_at: created_at,
        };
        println!("{:?}", v);
        v
    }
}
impl Stream {
    pub fn parse_xa(value: XA, v_top: &Vec<u128>) -> Self {
        let created_at = get_current_timestamp();
        let mut map = HashMap::new();
        let mut key_vals_map = HashMap::new();
        let id = value.id;
        let mut top = id
            .split("-")
            .map(|a| String::from(a))
            .collect::<Vec<String>>();
        if top[1] == "*" {
            let x = top[0].parse::<u128>().unwrap();
            if x == v_top[0] {
                let x = format!("{}", v_top[1] + 1);
                top[1] = x;
            } else {
                top[1] = "0".into();
            }
        }
        let top = top
            .into_iter()
            .map(|a| a.parse::<u128>().unwrap())
            .collect::<Vec<u128>>();
        let id = format!("{}-{}", top[0], top[1]);
        let key_vals = value.key_vals;
        for i in (0..key_vals.len()).step_by(2) {
            let val = Val {
                created_at: created_at,
                val: key_vals[i + 1].clone(),
                expiry: None,
                value_type: ValType::STRING,
            };
            println!("val : {:?}\n\n", val);
            let key = key_vals[i].clone();
            key_vals_map.insert(key, val);
        }
        println!("KEY VAL MAP : {:?}", key_vals_map);
        map.insert(id, key_vals_map);
        let v = Stream {
            top: top,
            map: map,
            created_at: created_at,
        };
        println!("{:?}", v);
        v
    }
}
#[derive(Clone, Debug)]
pub struct Replica {
    pub port: i32,
    pub host: String,
}

impl Replica {
    pub fn propagate(&self, command: Vec<&str>) {
        println!("{:?}", self);
        let resp_command = resp_command(command);
        println!("{resp_command}");
        let mut stream = TcpStream::connect(format!("{}:{}", self.host, self.port)).unwrap();
        stream.write(resp_command.as_bytes()).unwrap();
    }
}
#[derive(Clone)]
pub struct ReplicationData {
    pub master: bool,
    pub replication_id: String,
    pub offset: i32,
    pub master_host: String,
    pub master_port: i32,
}
#[derive(Clone)]
pub struct Config {
    pub port: i32,
    pub replication: ReplicationData,
    pub replicas: Vec<Replica>,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            port: 6379,
            replication: ReplicationData {
                master: true,
                replication_id: String::from("8371b4fb1155b71f4a04d3e1bc3e18c4a990aeeb"),
                offset: 0,
                master_host: "127.0.0.1".into(),
                master_port: 6379,
            },
            replicas: vec![],
        }
    }
}
