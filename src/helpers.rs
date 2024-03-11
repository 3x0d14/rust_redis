use std::{
    io::{Read, Write},
    net::TcpStream,
    str,
    time::{SystemTime, UNIX_EPOCH},
    vec,
};

use crate::{
    commands::{Command, ReplConfData, Set, XRangeData, XA},
    data::{Config, Stream, Val, ValType},
    errors::IdError,
    types::{Memory, StreamMemory},
};

pub fn resp_response(message: &str) -> String {
    let l = message.len();
    format!("${}\r\n{}\r\n", l, message)
}
pub fn parse_array(message: &str, start: Option<usize>) -> Vec<&str> {
    let start = start.unwrap_or(2);
    let mut res: Vec<&str> = vec![];
    let m = message.split("\r\n").collect::<Vec<&str>>();
    for i in (start..m.len()).step_by(2) {
        res.push(m[i]);
    }
    res
}
pub fn resp_command(command: Vec<&str>) -> String {
    let l = command.len();
    let mut result = format!("*{l}\r\n");
    for i in 0..l {
        let lx = command[i].len();
        result.push_str(format!("${lx}\r\n{}\r\n", command[i]).as_str())
    }
    result
}
pub fn loose_eq(a: &str, b: &str) -> bool {
    a.to_ascii_lowercase() == b.to_ascii_lowercase()
}
pub fn to_command(input: Vec<&str>) -> Command {
    let action = input[0].to_ascii_lowercase();
    let result = match action.as_str() {
        "ping" => Command::Ping,
        "echo" => Command::Echo(input[1].into()),
        "set" => Command::Set(Set::from(input)),
        "get" => Command::Get(input[1].into()),
        "info" => {
            if input.len() == 3 {
                return Command::Info(Some(input[2].into()));
            }
            Command::Info(None)
        }
        "xadd" => Command::XAdd(XA::from(input)),
        "type" => Command::Type(input[1].into()),
        "psync" => Command::PSYNC,
        "replconf" => Command::ReplConf(ReplConfData::from(input)),
        "xrange" => Command::Xrange(XRangeData::from(input)),
        _ => Command::Null,
    };
    result
}
pub fn get_current_timestamp() -> u128 {
    let start = SystemTime::now();
    let timestamp = start.duration_since(UNIX_EPOCH).unwrap().as_millis();
    timestamp
}
pub fn parse_config(config: Vec<String>) -> Config {
    let l = config.len();
    let mut conf = Config::default();
    let mut i = 1;
    while i < l {
        if config[i] == "--port" {
            let port = config[i + 1].parse::<i32>().expect("Wrong value for port");
            conf.port = port;
            i += 2;
        } else if config[i] == "--replicaof" {
            conf.replication.master = false;
            conf.replication.replication_id = String::new();
            conf.replication.master_host = config[i + 1].clone();
            conf.replication.master_port = config[i + 2].parse::<i32>().unwrap();
            i += 3;
        }
    }
    return conf;
}
pub fn handshake(host: &str, port: i32, this_port: i32) {
    // weak solution, must refactor
    let mut stream = TcpStream::connect(format!("{host}:{port}")).unwrap();
    stream.write(resp_command(vec!["ping"]).as_bytes()).unwrap();
    let mut result = [0; 512];
    stream.read(&mut result).unwrap();
    let result: String = str::from_utf8(&result).unwrap().into();
    let result = parse_array(&result, Some(0))[0];
    if format!("{result}\r\n") == "+PONG\r\n" {
        println!("All gone well");
    } else {
        panic!("Something went wrong")
    }
    stream
        .write(
            resp_command(vec![
                "REPLCONF",
                "listening-port",
                format!("{this_port}").as_str(),
            ])
            .as_bytes(),
        )
        .unwrap();
    let mut result = [0; 512];
    stream.read(&mut result).unwrap();
    let result: String = str::from_utf8(&result).unwrap().into();
    let result = parse_array(&result, Some(0))[0];
    if format!("{result}\r\n") == "+OK\r\n" {
        println!("All gone well");
    } else {
        panic!("Something went wrong")
    }
    stream
        .write(resp_command(vec!["REPLCONF", "capa", "psync2"]).as_bytes())
        .unwrap();
    let mut result = [0; 512];
    stream.read(&mut result).unwrap();
    let result: String = str::from_utf8(&result).unwrap().into();
    let result = parse_array(&result, Some(0))[0];
    if format!("{result}\r\n") == "+OK\r\n" {
        println!("All gone well");
    } else {
        panic!("Something went wrong")
    }
    stream
        .write(resp_command(vec!["PSYNC", "?", "-1"]).as_bytes())
        .unwrap();
    let mut result = [0; 512];
    stream.read(&mut result).unwrap();
    println!("hello");
}
// pub fn replconf_port(host: &str, port: i32, this_port: i32) {
//     // weak solution, must refactor,
//     println!("Replconf port");
//     let mut stream = TcpStream::connect(format!("{host}:{port}")).unwrap();
//     stream
//         .write(
//             resp_command(vec![
//                 "REPLCONF",
//                 "listening-port",
//                 format!("{this_port}").as_str(),
//             ])
//             .as_bytes(),
//         )
//         .unwrap();
//     stream
//         .write(resp_command(vec!["REPLCONF", "listening-port", "psync2"]).as_bytes())
//         .unwrap();
//     let mut result = [0; 512];
//     // stream.read(&mut result).unwrap();
//     // let result: String = str::from_utf8(&result).unwrap().into();
//     // println!("{result}");
//     // let result = parse_array(&result, Some(0))[0];
//     // if format!("{result}\r\n") == "+OK\r\n" {
//     //     println!("All gone well");
//     // } else {

//     //     panic!("Something went wrong")
//     // }
// }
// pub fn replconf_capa(host: &str, port: i32) {
//     // weak solution, must refactor
//     println!("Replconf capa");
//     let mut stream = TcpStream::connect(format!("{host}:{port}")).unwrap();
//     stream
//         .write(resp_command(vec!["REPLCONF", "listening-port", "psync2"]).as_bytes())
//         .unwrap();
//     // let mut result = [0; 512];
//     // stream.read(&mut result).unwrap();
//     // let result: String = str::from_utf8(&result).unwrap().into();
//     // println!("{result}");
//     // let result = parse_array(&result, Some(0))[0];
//     // if format!("{result}\r\n") == "+OK\r\n" {
//     //     println!("All gone well");
//     // } else {
//     //     panic!("Something went wrong")
//     // }
// }
pub fn hex_to_binary(hex: &str) -> Option<Vec<u8>> {
    if hex.len() % 2 != 0 {
        return None;
    }

    let mut binary_vec = Vec::new();

    for chunk in hex.as_bytes().chunks(2) {
        let chunk_str = std::str::from_utf8(chunk).ok()?;

        let byte = match u8::from_str_radix(chunk_str, 16) {
            Ok(byte) => byte,
            Err(_) => return None,
        };

        binary_vec.push(byte);
    }

    Some(binary_vec)
}
pub fn concat_u8(a: &mut Vec<u8>, b: &mut Vec<u8>) {
    a.append(b);
}
pub fn stream_add(
    memory: &mut Memory,
    stream_memory: &mut StreamMemory,
    xa: &XA,
) -> Result<Vec<u128>, IdError> {
    // for each stream create a val in main memory where type is stream
    let key = xa.stream_key.clone();
    let mut m = stream_memory.lock().unwrap();
    let mut mem = memory.lock().unwrap();
    let mut found = true;
    let mut id = vec![];
    match m.get_mut(&key) {
        Some(v) => {
            let v_top = &v.top;
            let x = Stream::parse_xa(xa.clone(), v_top);
            let x_top = x.top;
            id = x_top.clone();
            if x_top[0] > v_top[0] || (x_top[0] == v_top[0] && x_top[1] > v_top[1]) {
                v.top = x_top;
                v.map.extend(x.map)
            } else if x_top[0] == 0 && x_top[1] == 0 {
                return Err(IdError::NullIdError);
            } else {
                return Err(IdError::IdError);
            }
        }
        None => {
            found = false;
        }
    }
    println!("map {:?}", m);
    if !found {
        let x = Stream::parse_xa(xa.clone(), &vec![0, 0]);
        id = x.top.clone();
        m.insert(key.clone(), x);
        mem.insert(
            key,
            Val {
                val: "".into(),
                created_at: 0,
                expiry: None,
                value_type: ValType::STREAM,
            },
        );
    }
    Ok(id)
}
pub fn get_time(id: &String) -> (u128, u128) {
    let mut a = id.split("-");
    (
        a.next().unwrap().parse::<u128>().unwrap(),
        a.next().unwrap().parse::<u128>().unwrap(),
    )
}
