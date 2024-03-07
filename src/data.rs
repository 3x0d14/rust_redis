use std::{io::Write, net::TcpStream};

use crate::{
    commands::Set,
    helpers::{get_current_timestamp, resp_command},
};
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
