#[derive(Debug)]
pub enum SupportedDataTypes {
    STRING(String),
    NUMBER(f32)
}

#[derive(Debug)]
pub struct CommandArgument {
    arg_type: SupportedDataTypes
}

impl ToString for CommandArgument {
    fn to_string(&self) -> String {
        match &self.arg_type {
            SupportedDataTypes::STRING(x) => { return x.to_string() },
            SupportedDataTypes::NUMBER(x) => {
                return x.to_string();
            }
        }
    }
}

#[derive(Debug)]
pub enum CommandType {
    ECHO(CommandArgument),
    PING,
}

#[derive(Debug)]
pub struct Command {
    pub c_type: CommandType,
}

impl Command {
    pub fn new(c_type: CommandType) -> Self {
        Self {
            c_type
        }
    }
    pub fn from_message(message: String) -> Self {
        let cmds: Vec<&str> = message.split_whitespace().collect();
        if (*cmds.get(0).unwrap()).trim().to_lowercase() == "echo" {
            return Self {
                c_type: CommandType::ECHO(CommandArgument {
                    arg_type: SupportedDataTypes::STRING(String::from(*cmds.get(1).unwrap()))
                })
            }
        } else {
            return Self {
                c_type: CommandType::PING
            }
        }
    }

    pub fn generate_response(&self) -> Vec<u8> {
        match &self.c_type {
            CommandType::ECHO(message) => {
                if let SupportedDataTypes::STRING(echo_msg) = &message.arg_type {
                    return (echo_msg.to_string() + "\r\n").as_bytes().to_vec();
                }
                return b"unsupported datatype\r\n".to_vec();
            }

            CommandType::PING => {
                return b"+PONG\r\n".to_vec();
            }
        }
    }
}
