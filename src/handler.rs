use crate::commands::Command;
use crate::data::{Config, Replica, Val};
use crate::helpers::{
    concat_u8, get_current_timestamp, hex_to_binary, parse_array, resp_command, resp_response,
    to_command,
};
use crate::types::{AConf, Memory};
use std::{
    io::{Read, Write},
    net::TcpStream,
    str,
};

pub fn handle(stream: &mut TcpStream, memory: &mut Memory, configuration: &AConf) {
    loop {
        let mut buffer: [u8; 532] = [0; 532];
        let n = stream.read(&mut buffer).unwrap();
        if n == 0 {
            break;
        }
        let command = parse_array(str::from_utf8(&buffer).unwrap(), None);
        let parsed_command = to_command(command.clone());
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
                let config = configuration.lock().unwrap();
                let replicas = config.replicas.clone();
                for replica in replicas {
                    // println!("We are here");
                    // let resp_command = resp_command(command.clone());
                    // println!("{resp_command}");
                    // let mut stream =
                    //     TcpStream::connect(format!("{}:{}", replica.host, replica.port)).unwrap();
                    // stream.write(resp_command.as_bytes()).unwrap();
                    replica.propagate(command.clone());
                }
                let mut m = memory.lock().unwrap();
                m.insert(s.key.clone(), Val::from(s));
                stream.write_all(resp_response("OK").as_bytes()).unwrap();
            }
            Command::Get(k) => {
                let mut map = memory.lock().unwrap();
                let mut response: String = "$-1\r\n".into();
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
            Command::Info(o) => {
                let replication = configuration.lock().unwrap();
                let role;
                if replication.replication.master {
                    role = "master";
                } else {
                    role = "slave";
                }
                let message = format!(
                    "role:{role}\nmaster_replid:{}\nmaster_repl_offset:{}",
                    replication.replication.replication_id, replication.replication.offset
                );
                stream
                    .write_all(resp_response(message.as_str()).as_bytes())
                    .unwrap();
            }
            Command::ReplConf(data) => {
                let mut config = configuration.lock().unwrap();
                let port = data.port;
                match port {
                    Some(v) => config.replicas.push(Replica {
                        port: v,
                        host: String::from("127.0.0.1"),
                    }),
                    None => {}
                }
                let response = "+OK\r\n";
                stream.write_all(response.as_bytes()).unwrap();
            }
            Command::PSYNC => {
                let config = configuration.lock().unwrap();
                let response = format!(
                    "+FULLRESYNC {} {}\r\n",
                    config.replication.replication_id, config.replication.offset
                );
                let response = response.as_str();
                let empty_rdb = "524544495330303131fa0972656469732d76657205372e322e30fa0a72656469732d62697473c040fa056374696d65c26d08bc65fa08757365642d6d656dc2b0c41000fa08616f662d62617365c000fff06e3bfec0ff5aa2";
                let mut x = hex_to_binary(empty_rdb).unwrap();
                let mut a = Vec::from(format!("${}\r\n", x.len()).as_bytes());
                concat_u8(&mut a, &mut x);
                stream.write_all(response.as_bytes()).unwrap();
                stream.write_all(&a).unwrap();
            }
            Command::Null => {
                let response = "$-1\r\n";
                stream.write_all(response.as_bytes()).unwrap();
            }
        }
    }
}
