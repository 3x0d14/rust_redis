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
    net::TcpListener,
    sync::{Arc, Mutex},
    thread,
};
use types::Memory;

fn main() {
    let memory: Memory = Arc::new(Mutex::new(HashMap::new()));
    let listner = TcpListener::bind("127.0.0.1:6379").expect("Failed to bind to port 6379");
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
