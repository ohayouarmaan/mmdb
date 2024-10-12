use crate::server::parser::DS;
use crate::datastore::store::{DataItem, DataStore};
use crate::server::server::ServerOptions;

use std::time::{Duration, Instant};

pub struct RESPInterpreter<'a> {
    source_code: String,
    data_store: &'a mut DataStore,
    server_options: &'a mut ServerOptions
}

pub struct SetOptions {
    expiry: Option<Instant>
}

impl<'a> RESPInterpreter<'a> {
    pub fn new(ds: &'a mut DataStore, server_options: &'a mut ServerOptions) -> Self {
        Self {
            source_code: String::from(""),
            data_store: ds,
            server_options
        }
    }

    pub fn register(&mut self, src_code: &str) {
        self.source_code = src_code.to_string();
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
                "config" => {
                    let config_action = leader_args.pop_front();
                    if let Some(ca) = config_action {
                        if let DS::String(_, _) = ca {
                            let action = ca.get_value(&self.source_code);
                            match action.to_lowercase().as_str() {
                                "get" => {
                                    let config_key = leader_args.pop_front();
                                    if let Some(key_ds) = config_key {
                                        let key = key_ds.get_value(&self.source_code);
                                        if key == "dir" {
                                            let dir_name: String = self.server_options.rdb_dir_name.clone().expect("expected a directory name found nothing").as_path().to_str().expect("no dirname").to_owned();
                                            return format!("*2\r\n$3\r\ndir\r\n${}\r\n{}\r\n", dir_name.len(), dir_name).to_owned();
                                        } else if key == "dbfilename" {
                                            let db_file_name: String = self.server_options.rdb_file_name.clone().expect("expected a file name found nothing").as_path().to_str().expect("no dbfilename").to_owned();
                                            return format!("*2\r\n$10\r\ndbfilename\r\n${}\r\n{}\r\n", db_file_name.len(), db_file_name).to_owned();

                                        } else {
                                            return "".to_owned();
                                        }

                                    } else {
                                        return "".to_owned();
                                    }
                                },
                                "set" => {
                                    let config_key = leader_args.pop_front();
                                    if let Some(key_ds) = config_key {
                                        let key = key_ds.get_value(&self.source_code);
                                        if key == "dir" {
                                            let new_dir_value = leader_args.pop_front().expect("Expected a value").get_value(&self.source_code);
                                            self.server_options.rdb_dir_name = Some(std::path::PathBuf::from(new_dir_value));
                                            return "+OK\r\n".to_owned();
                                        } else if key == "dbfilename" {
                                            let new_db_file_name_value = leader_args.pop_front().expect("Expected a value").get_value(&self.source_code);
                                            self.server_options.rdb_file_name = Some(std::path::PathBuf::from(new_db_file_name_value));
                                            return "+OK\r\n".to_owned();
                                        } else {
                                            return "-ERROR trying to set invalid config option\r\n".to_owned();
                                        }

                                    } else {
                                        return "-ERROR no config key sent\r\n".to_owned();
                                    }
                                },
                                c => {
                                    println!("{:?}", c);
                                    todo!("");
                                },
                            }
                        } else {
                            return "-ERROR config action invalid".to_owned();
                        }
                    } else {
                        return "-ERROR config action invalid".to_owned();
                    }
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
