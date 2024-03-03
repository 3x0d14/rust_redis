use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    commands::{Command, Set},
    data::Config,
};

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
    println!("{:?}", input);
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
            conf.replicaof = true;
            i += 3;
        }
    }
    return conf;
}
