use crate::actions::{echo, get, info, ping, psync, replconf, set, type_fn, xadd};
use crate::commands::Command;
use crate::data::{Replica, Stream, Val, ValType};
use crate::helpers::{
    concat_u8, get_current_timestamp, hex_to_binary, parse_array, resp_response, to_command,
};
use crate::types::{AConf, Memory, StreamMemory};
use std::{
    io::{Read, Write},
    net::TcpStream,
    str,
};

pub fn handle(
    stream: &mut TcpStream,
    memory: &mut Memory,
    stream_memory: &mut StreamMemory,
    configuration: &AConf,
) {
    loop {
        let mut buffer: [u8; 532] = [0; 532];
        let n = stream.read(&mut buffer).unwrap();
        if n == 0 {
            break;
        }
        let command = parse_array(str::from_utf8(&buffer).unwrap(), None);
        let parsed_command = to_command(command.clone());
        match parsed_command {
            Command::Ping => ping(stream),
            Command::Echo(v) => echo(stream, v),
            Command::Set(s) => set(s, stream, memory, configuration, command),
            Command::Get(k) => get(stream, memory, k),
            Command::Info(o) => info(stream, configuration),
            Command::ReplConf(data) => replconf(stream, configuration, data),
            Command::PSYNC => psync(stream, configuration),
            Command::Type(k) => type_fn(stream, memory, k),
            Command::XAdd(xa) => xadd(stream, configuration, memory, stream_memory, command, xa),
            Command::Null => {
                let response = "$-1\r\n";
                stream.write_all(response.as_bytes()).unwrap();
            }
        }
    }
}
