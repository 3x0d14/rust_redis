// Mods
mod commands;
mod data;
mod handler;
mod helpers;
mod types;
use data::Config;
// Uses
use handler::handle;
use helpers::{handshake, parse_config};
use std::{
    collections::HashMap,
    env,
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use types::Memory;

use crate::helpers::get_current_timestamp;

fn main() {
    let memory: Memory = Arc::new(Mutex::new(HashMap::new()));
    let args: Vec<String> = env::args().collect();
    let c = parse_config(args);
    let configuration = Arc::new(Mutex::new(c.clone()));
    let listner =
        TcpListener::bind(format!("127.0.0.1:{}", c.port)).expect("Failed to bind to port");
    if !c.replication.master {
        handshake(
            &c.replication.master_host,
            c.replication.master_port,
            c.port,
        );
        // replconf_port(
        //     &c.replication.master_host,
        //     c.replication.master_port,
        //     c.port,
        // );
        // replconf_capa(&c.replication.master_host, c.replication.master_port)
    }
    let check_memory = Arc::clone(&memory);
    let _j = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(60));
        let mut local_memory = check_memory.lock().unwrap();
        let x = local_memory.clone();
        println!("{:?}", x);
        for (key, val) in x.iter() {
            match val.expiry {
                Some(ex) => {
                    let now = get_current_timestamp();
                    let created_at = val.created_at;
                    if now - created_at >= ex {
                        local_memory.remove(key);
                    }
                }
                None => {}
            }
        }
    });
    for stream in listner.incoming() {
        match stream {
            Ok(mut s) => {
                let mut local_memory = Arc::clone(&memory);
                let configuration_copy = Arc::clone(&configuration);
                let _join_handle =
                    thread::spawn(move || handle(&mut s, &mut local_memory, &configuration_copy));
            }
            Err(e) => {
                println!("Error {e}");
            }
        }
    }
}
