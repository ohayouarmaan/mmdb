use crate::server::parser::DS;
use crate::datastore::store::{DataItem, DataStore};
use std::time::{Duration, Instant};

pub struct RESPInterpreter<'a> {
    source_code: String,
    data_store: &'a mut DataStore
}

pub struct SetOptions {
    expiry: Option<Instant>
}

impl<'a> RESPInterpreter<'a> {
    pub fn new(src_code: &str, ds: &'a mut DataStore) -> Self {
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
            let mut leader_args = std::collections::VecDeque::from(v.1);
            match leader_cmd.as_str() {
                "echo" => {
                    self.build_response(leader_args.front().expect("Expected an argument"))
                },
                "set" => {
                    let key = leader_args.pop_front().unwrap();
                    let value = leader_args.pop_front().expect("Expectend another argument");
                    
                    let key_string;
                    match key {
                        DS::String(start, end) => {
                            key_string = self.source_code.get(start..end).unwrap();
                        },
                        _ => {
                            return "-ERROR Expected the key to be a string".to_owned();
                        }
                    }

                    let value_string;
                    match value {
                        DS::String(start, end) => {
                            value_string = self.source_code.get(start..end).unwrap();
                        },
                        _ => {
                            return "-ERROR Expected the value to be a string".to_owned();
                        }
                    }
                    
                    let mut args = SetOptions {
                        expiry: None
                    };
                    
                    if leader_args.len() != 0 {
                        while let Some(current_option) = leader_args.pop_front() {
                            match current_option {
                                DS::String(start, end) => {
                                    match self.source_code.get(start..end).unwrap().to_lowercase().as_str() {
                                        "px" => {
                                            let d = leader_args.pop_front().unwrap();
                                            if let DS::String(duration_start, duration_end) = d {
                                                let x: u64 = self.source_code.get(duration_start..duration_end)
                                                    .expect("")
                                                    .to_owned()
                                                    .parse::<u64>()
                                                    .unwrap();
                                                
                                                let current_time = Instant::now();
                                                let expiry_time = current_time + Duration::from_millis(x);
                                                args.expiry = Some(expiry_time);
                                            }
                                        },
                                        _ => {}
                                    }
                                },
                                _ => {
                                    return "-ERROR Invalid argument type".to_owned();
                                }
                            }
                        }
                    }
                    
                    self.data_store.set(key_string.to_owned(), DataItem {
                        data: value_string.to_owned(),
                        expiry: args.expiry
                    });
                    return "+OK\r\n".to_string();
                },
                "get" => {
                    let key = leader_args.front().unwrap();
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
                    match self.data_store.get(key_string.to_string()) {
                        Some(v) => {
                            let current_time = Instant::now();
                            if let Some(expiry) = v.expiry {
                                if expiry < current_time {
                                    self.data_store.remove(key_string.to_string());
                                    response.push_str("-1");
                                    response.push_str("\r\n");
                                } else {
                                    response.push_str(&format!("{}", v.data.len()));
                                    response.push_str("\r\n");
                                    response.push_str(&v.data);
                                    response.push_str("\r\n");
                                }
                            } else {
                                response.push_str(&format!("{}", v.data.len()));
                                response.push_str("\r\n");
                                response.push_str(&v.data);
                                response.push_str("\r\n");
                            }
                        },
                        None => {
                            response.push_str("-1");
                            response.push_str("\r\n");
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
