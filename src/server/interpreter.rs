use crate::server::parser::{DS, RedArray};
use std::collections::HashMap;

pub struct RESPInterpreter<'a> {
    source_code: String,
    data_store: &'a mut HashMap<String, String>
}

impl<'a> RESPInterpreter<'a> {
    pub fn new(src_code: &str, ds: &'a mut HashMap<String, String>) -> Self {
        Self {
            source_code: src_code.to_string(),
            data_store: ds
        }
    }

    fn build_command(&self, value: DS) -> Result<(String, Vec<DS>), ()> {
        match value {
            DS::RedArray(a) => {
                if let Some(command) = a.value.first() {
                    if let DS::String(start, end) = command {
                        Ok(
                            (
                                self.source_code.get(*start..*end).unwrap().to_lowercase().to_string(),
                                a.value.into_iter().skip(1).collect()
                            )
                        )
                    } else {
                        Err(())
                    }
                } else {
                    Err(())
                }
            },
            _ => {
                Err(())
            }
        }
    }

    pub fn build_response(&self, ds: &DS) -> String {
        match ds {
            DS::String(start, end) => {
                let mut response = String::from("+");
                response.push_str(&(self.source_code[*start..*end]).to_string());
                response.push_str("\r\n");
                return response;
            },
            DS::Integer(x) => {
                return format!("{}\r\n", x);
            }
            DS::RedArray(x) => {
                let mut response = String::from("[");
                for d in &x.value {
                    let x = self.build_response(&d);
                    response.push_str(&x);
                    response.push_str(" ");
                }
                response.push_str("]");
                return response;
            }
        }
    }

    pub fn interpret(&mut self, ds: DS) -> String {
        let cmd = self.build_command(ds);
        if let Err(_) = cmd {
            return "- Error while interpreting the message".to_string();
        } else {
            let v = cmd.unwrap();
            let leader_cmd = v.0;
            let leader_args = v.1;
            match leader_cmd.as_str() {
                "echo" => {
                    self.build_response(leader_args.first().expect("Expected an argument"))
                },
                "set" => {
                    let key = leader_args.first().unwrap();
                    let value = leader_args.get(1).expect("Expectend another argument");
                    
                    let key_string;
                    match key {
                        DS::String(start, end) => {
                            key_string = self.source_code.get(*start..*end).unwrap();
                        },
                        _ => {
                            return "-ERROR Expected the key to be a string".to_owned();
                        }
                    }

                    let value_string;
                    match value {
                        DS::String(start, end) => {
                            value_string = self.source_code.get(*start..*end).unwrap();
                        },
                        _ => {
                            return "-ERROR Expected the value to be a string".to_owned();
                        }
                    }
                    
                    self.data_store.insert(key_string.to_owned(), value_string.to_owned());
                    println!("ds: {:?}", self.data_store);
                    return "+OK\r\n".to_string();
                },
                "get" => {
                    let key = leader_args.first().unwrap();
                    println!("key: {:?}", key);
                    println!("ds: {:?}", self.data_store);
                    let key_string;
                    match key {
                        DS::String(start, end) => {
                            key_string = self.source_code.get(*start..*end).unwrap();
                        },
                        _ => {
                            return "-ERROR Expected the key to be a string".to_owned();
                        }
                    }
                    let mut response = String::from("$");
                    match self.data_store.get(key_string) {
                        Some(v) => {
                            response.push_str(&format!("{}", v.len()));
                            response.push_str("\r\n");
                            response.push_str(v);
                            response.push_str("\r\n");
                            println!("ds: {:?}", self.data_store);
                        },
                        None => {
                            response.push_str("-1");
                            response.push_str("\r\n");
                            println!("ds: {:?}", self.data_store);
                        }
                    }
                    return response;
                },
                "ping" => {
                    return "+PONG\r\n".to_string();
                },
                _ => {
                    return "-ERROR Unknown command".to_string();
                }
            }
        }
    }
}
