use crate::server::parser::{DS, RedArray};

pub struct RESPInterpreter {
    source_code: String
}

impl RESPInterpreter {
    pub fn new(src_code: &str) -> Self {
        Self {
            source_code: src_code.to_string()
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

    pub fn interpret(&self, ds: DS) -> String {
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
