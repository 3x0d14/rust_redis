use std::{io::Write, net::TcpStream};

use crate::{
    commands::{ReplConfData, Set, XRangeData, XA},
    data::{EntryRepresentation, Replica, StreamRepresentation, Val, ValType},
    errors::IdError,
    helpers::{
        concat_u8, get_current_timestamp, get_time, hex_to_binary, resp_response, stream_add,
    },
    types::{AConf, Memory, StreamMemory},
};

pub fn ping(stream: &mut TcpStream) {
    stream
        .write_all(String::from("+PONG\r\n").as_bytes())
        .unwrap();
}
pub fn echo(stream: &mut TcpStream, data: String) {
    stream.write_all(resp_response(&data).as_bytes()).unwrap();
}
pub fn set(
    s: Set,
    stream: &mut TcpStream,
    memory: &mut Memory,
    configuration: &AConf,
    command: Vec<&str>,
) {
    let config = configuration.lock().unwrap();
    let replicas = config.replicas.clone();
    for replica in replicas {
        replica.propagate(command.clone());
    }
    let mut m = memory.lock().unwrap();
    m.insert(s.key.clone(), Val::from(s));
    stream.write_all(resp_response("OK").as_bytes()).unwrap();
}
pub fn get(stream: &mut TcpStream, memory: &mut Memory, k: String) {
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
pub fn info(stream: &mut TcpStream, configuration: &AConf) {
    let replication = configuration.lock().unwrap();
    let role;
    if replication.replication.master {
        role = "master";
    } else {
        role = "slave";
    }
    let message = format!(
        "role:{role}\nmaster_replid:{}\nmaster_repl_offset:{}\n",
        replication.replication.replication_id, replication.replication.offset
    );
    stream
        .write_all(resp_response(message.as_str()).as_bytes())
        .unwrap();
}
pub fn replconf(stream: &mut TcpStream, configuration: &AConf, data: ReplConfData) {
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
pub fn psync(stream: &mut TcpStream, configuration: &AConf) {
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
pub fn type_fn(stream: &mut TcpStream, memory: &mut Memory, k: String) {
    let mem = memory.lock().unwrap();
    let x = mem.get(&k);
    match x {
        Some(v) => {
            let typ;
            match v.value_type {
                ValType::STRING => typ = "string",
                ValType::STREAM => typ = "stream",
            }
            stream.write_all(resp_response(typ).as_bytes()).unwrap();
        }
        None => {
            stream.write_all(resp_response("none").as_bytes()).unwrap();
        }
    }
}
pub fn xadd(
    stream: &mut TcpStream,
    configuration: &AConf,
    memory: &mut Memory,
    stream_memory: &mut StreamMemory,
    command: Vec<&str>,
    xa: XA,
) {
    let config = configuration.lock().unwrap();
    let replicas = config.replicas.clone();
    for replica in replicas {
        replica.propagate(command.clone());
    }
    match stream_add(memory, stream_memory, &xa) {
        Ok(id) => {
            stream
                .write_all(resp_response(&format!("{}-{}", id[0], id[1])).as_bytes())
                .unwrap();
        }
        Err(e) => match e {
            IdError::NullIdError => {
                stream
                    .write_all(
                        "-ERR The ID specified in XADD must be greater than 0-0\r\n".as_bytes(),
                    )
                    .unwrap();
            }
            IdError::IdError => {
                stream.write_all("-ERR The ID specified in XADD is equal or smaller than the target stream top item\r\n".as_bytes()).unwrap();
            }
        },
    }
}
pub fn xrange_action(stream: &mut TcpStream, stream_memory: &mut StreamMemory, xrange: XRangeData) {
    // get Stream data with key,
    // convert stream into a filtrable structure
    // filter on a b
    // transform the result into a parsable structure
    // parse the structure
    // RESP array return
    let key = xrange.key;
    let a = xrange.a;
    let b = xrange.b;
    let mut representation = StreamRepresentation::new();
    let local_memory = stream_memory.lock().unwrap();
    let s = local_memory.get(&key).unwrap();
    for (id, entry) in s.map.clone().into_iter() {
        let nid = get_time(&id);
        if nid >= a && nid <= b {
            let mut repr_data: Vec<String> = vec![];
            for (key, val) in entry.into_iter() {
                repr_data.push(key);
                repr_data.push(val.val);
            }
            let entry_representation = EntryRepresentation {
                nid: nid,
                id: id,
                data: repr_data,
            };
            representation.data.push(entry_representation);
        }
    }
    representation
        .data
        .sort_by(|a, b| a.nid.partial_cmp(&b.nid).unwrap());
    println!("{}", representation.resp());
    let response = representation.resp();
    stream.write_all(response.as_bytes()).unwrap();
}
