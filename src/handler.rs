use crate::commands::Command;
use crate::data::Val;
use crate::helpers::{get_current_timestamp, parse_array, resp_response, to_command};
use crate::types::Memory;
use std::{
    io::{Read, Write},
    net::TcpStream,
    str,
};

pub fn handle(stream: &mut TcpStream, memory: &mut Memory) {
    println!("Received message");
    let mut buffer: [u8; 532] = [0; 532];
    loop {
        let n = stream.read(&mut buffer).unwrap();
        if n == 0 {
            break;
        }
        // println!("{}", str::from_utf8(&buffer).unwrap());
        let command = parse_array(str::from_utf8(&buffer).unwrap());
        let parsed_command = to_command(command);
        match parsed_command {
            Command::Ping => {
                stream
                    .write_all(String::from("+PONG\r\n").as_bytes())
                    .unwrap();
            }
            Command::Echo(v) => {
                stream.write_all(resp_response(&v).as_bytes()).unwrap();
            }
            Command::Set(s) => {
                let mut m = memory.lock().unwrap();
                m.insert(s.key.clone(), Val::from(s));
                stream.write_all(resp_response("OK").as_bytes()).unwrap();
            }
            Command::Get(k) => {
                let mut map = memory.lock().unwrap();
                let mut response: String = "$-1\r\n".into();
                let mut delete: bool = false;
                let x = map.clone();
                match x.get(&k) {
                    Some(v) => {
                        if let Some(expiry) = v.expiry {
                            let now = get_current_timestamp();
                            let created_at = v.created_at;
                            if now - created_at >= expiry {
                                map.remove(&k);
                            } else {
                                response = resp_response(v.val.as_str());
                            }
                        } else {
                            response = resp_response(v.val.as_str());
                        }
                    }
                    None => {}
                }
                stream.write_all(response.as_bytes()).unwrap();
            }
            Command::Null => {
                let response = "$-1\r\n";
                stream.write_all(response.as_bytes()).unwrap();
            }
        }
    }
}
