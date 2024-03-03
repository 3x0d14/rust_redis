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
};
use types::Memory;

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
