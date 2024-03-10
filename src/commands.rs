use crate::helpers::loose_eq;
#[derive(Debug)]
pub enum Command {
    Echo(String),
    Ping,
    Set(Set),
    Get(String),
    Info(Option<String>),
    ReplConf(ReplConfData),
    PSYNC,
    Type(String),
    XAdd(XA),
    Null,
}
#[derive(Debug)]
pub struct Set {
    pub key: String,
    pub val: String,
    pub expiry: Option<u128>,
}

impl From<Vec<&str>> for Set {
    fn from(value: Vec<&str>) -> Self {
        let key = String::from(value[1]);
        let val = String::from(value[2]);
        let mut expiry: Option<u128> = None;

        if value.len() > 3 {
            let parameter = value[3];
            if loose_eq(parameter, "px") {
                match value[4].parse::<u128>() {
                    Ok(v) => {
                        expiry = Some(v);
                    }
                    Err(_) => {}
                }
            }
        }
        Set {
            key: key,
            val: val,
            expiry: expiry,
        }
    }
}
#[derive(Debug, Clone)]
pub struct XA {
    pub stream_key: String,
    pub id: String,
    pub key_vals: Vec<String>,
}
impl From<Vec<&str>> for XA {
    fn from(value: Vec<&str>) -> Self {
        let l = value.len();
        let stream_key = String::from(value[1]);
        let id = String::from(value[2]);
        let mut key_vals: Vec<String> = vec![];
        for i in 3..l {
            key_vals.push(value[i].into())
        }
        let x = XA {
            stream_key: stream_key,
            id: id,
            key_vals: key_vals,
        };
        println!("{:?}", x);
        x
    }
}
#[derive(Debug)]
pub struct ReplConfData {
    pub port: Option<i32>,
}
impl From<Vec<&str>> for ReplConfData {
    fn from(value: Vec<&str>) -> Self {
        let argument = value[1];
        let value = String::from(value[2]);
        if argument == "listening-port" {
            ReplConfData {
                port: Some(value.parse::<i32>().unwrap()),
            }
        } else {
            ReplConfData { port: None }
        }
    }
}
