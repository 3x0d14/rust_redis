// Mods
mod commands;
mod data;
mod handler;
mod helpers;
mod types;
// Uses
use handler::handle;
use std::{
    collections::HashMap,
    env,
    net::TcpListener,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use types::Memory;

use crate::helpers::get_current_timestamp;

fn main() {
    let memory: Memory = Arc::new(Mutex::new(HashMap::new()));
    let args: Vec<String> = env::args().collect();
    let mut port = 6379;
    if args.len() > 1 {
        let arg = &args[1];
        let val = &args[2];
        if arg == "--port" {
            port = val.parse::<i32>().unwrap();
        }
    }
    let listner =
        TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Failed to bind to port 6379");
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
                let _join_handle = thread::spawn(move || handle(&mut s, &mut local_memory));
            }
            Err(e) => {
                println!("Error {e}");
            }
        }
    }
}
