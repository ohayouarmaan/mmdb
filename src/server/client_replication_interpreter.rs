use super::parser::DS;
use crate::helpers::Helper;
use crate::server::interpreter::Reply;

pub struct ReplicationInterpreter {
    source_ds: Option<DS>,
    source_code: Option<String>,
    port: u32,
    state: ClientConnectionState
}

pub enum ClientConnectionState {
    BeforePing,
    PingSentSuccessfully,
    ReplConf1Sent,
    ReplConf2Sent, // This basically means that the handshake is complete
}

impl ReplicationInterpreter {
    pub fn new(ds: Option<DS>, listening_port: &u32) -> Self {
        Self {
            source_ds: ds,
            source_code: None,
            port: *listening_port,
            state: ClientConnectionState::BeforePing
        }
    }

    pub fn register(&mut self, ds: DS, source_code: &str) {
        self.source_ds = Some(ds);
        self.source_code = Some(source_code.to_string());
    }

    pub fn interpret(&mut self) -> Option<String> {
        match &self.source_ds {
            Some(DS::String(_, _)) => {
                if let Some(source_code) = &self.source_code {
                    match self.source_ds
                        .as_ref()
                        .unwrap()
                        .get_value(&source_code)
                        .to_lowercase()
                        .as_str() {
                        "pong" => {
                            self.state = ClientConnectionState::PingSentSuccessfully;
                            Some(Helper::build_resp(&Reply::ReplyArray(
                                vec!(
                                    Reply::ReplyBulkString("REPLCONF".to_string()),
                                    Reply::ReplyBulkString("listening-port".to_string()),
                                    Reply::ReplyBulkString(format!("{}", self.port)),
                                )
                            )))
                        },
                        "ok" => {
                            match self.state {
                                ClientConnectionState::PingSentSuccessfully => {
                                    self.state = ClientConnectionState::ReplConf1Sent;
                                    Some(Helper::build_resp(&Reply::ReplyArray(
                                        vec!(
                                            Reply::ReplyBulkString("REPLCONF".to_string()),
                                            Reply::ReplyBulkString("capa".to_string()),
                                            Reply::ReplyBulkString("psync2".to_string()),
                                        )
                                    )))
                                },
                                ClientConnectionState::ReplConf1Sent => {
                                    self.state = ClientConnectionState::ReplConf2Sent;
                                    Some(Helper::build_resp(&Reply::ReplyArray(
                                        vec!(
                                            Reply::ReplyBulkString("PSYNC".to_string()),
                                            Reply::ReplyBulkString("?".to_string()),
                                            Reply::ReplyBulkString("-1".to_string()),
                                        )
                                    )))
                                },
                                _ => {
                                    Some(Helper::build_resp(
                                        &Reply::ReplyBulkString("-ERROR Invalid Command".to_string())
                                    ))
                                }
                                
                            }
                        },
                        "fullresync" => {
                            Some(Helper::build_resp(
                                &Reply::ReplyString("OK".to_string())
                            ))
                        },
                        c => {
                            Some(Helper::build_resp(
                                &Reply::ReplyBulkString("-ERROR Invalid Command".to_string())
                            ))
                        }
                    }
                } else {
                    println!("No Source code");
                    Some(Helper::build_resp(
                        &Reply::ReplyBulkString("-ERROR Invalid Command".to_string())
                    ))
                }
            },
            c => {
                println!("DS IS NOT A STRING it is {:?}", c);
                Some(Helper::build_resp(
                    &Reply::ReplyBulkString("-ERROR Invalid Command".to_string())
                ))
            }
        }
    }
}
