use std::time::{SystemTime, UNIX_EPOCH};

use crate::commands::{Command, Set};

pub fn resp_response(message: &str) -> String {
    let l = message.len();
    format!("${}\r\n{}\r\n", l, message)
}
pub fn parse_array(message: &str) -> Vec<&str> {
    let mut res: Vec<&str> = vec![];
    let m = message.split("\r\n").collect::<Vec<&str>>();
    for i in (2..m.len()).step_by(2) {
        res.push(m[i]);
    }
    res
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
        _ => Command::Null,
    };
    result
}
pub fn get_current_timestamp() -> u128 {
    let start = SystemTime::now();
    let timestamp = start.duration_since(UNIX_EPOCH).unwrap().as_millis();
    timestamp
}
