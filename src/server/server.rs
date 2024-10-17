use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

use crate::datastore::store::DataStore;
use crate::server::parser::RESPParser;
use crate::server::interpreter::{RESPInterpreter, Reply};
use crate::server::client_replication_interpreter::ReplicationInterpreter;
use crate::helpers::Helper;

#[derive(Debug,Clone)]
pub struct SlaveServerOptions {
    pub master_host: String,
    pub master_port: u32,
}

#[derive(Debug,Clone)]
pub struct MasterServerOptions {
    pub master_replid: String,
    pub master_repl_offset: u32,
}


#[derive(Debug,Clone)]
pub enum ServerRole {
    Master(Option<MasterServerOptions>),
    Slave(SlaveServerOptions)
}

#[derive(Debug,Clone)]
pub struct ServerOptions {
    pub rdb_file_name: Option<std::path::PathBuf>,
    pub rdb_dir_name: Option<std::path::PathBuf>,
    pub port: Option<u32>,
    pub server_role: Option<ServerRole>,
}

pub struct Server {
    listener: TcpListener,
    clients: Vec<Client>,
    store: DataStore,
    pub server_options: ServerOptions,
    pub replication_stream: Option<TcpStream>
}

pub struct Client {
    pub client: TcpStream,
}

impl Server {
    pub fn new(address: &str, server_opts: ServerOptions, ds: Option<DataStore>) -> Self {
        match ds {
            Some(data) => {
                Self {
                    listener: TcpListener::bind(address).unwrap(),
                    clients: vec![],
                    store: data,
                    server_options: server_opts,
                    replication_stream: None
                }
            },
            None => {
                Self {
                    listener: TcpListener::bind(address).unwrap(),
                    clients: vec![],
                    store: DataStore::new(),
                    server_options: server_opts,
                    replication_stream: None
                }
            }
        }
    }

    pub fn connect_to_master(&mut self) {
        match &self.server_options.server_role {
            Some(ServerRole::Slave(slave_options)) => {
                let mut master_connection_stream = TcpStream::connect(format!("{}:{}", slave_options.master_host, slave_options.master_port)).unwrap();
                let _ = master_connection_stream.write(Helper::build_resp(&Reply::ReplyArray(vec![Reply::ReplyBulkString("PING".to_string())])).as_bytes());
                self.replication_stream = Some(master_connection_stream);
            }
            _ => {}
        }
    }

    pub fn run_event_loop(&mut self) {
        // Firstly, we will have to loop indefinitely and in every step we will check for three
        // things
        // 1. Do we have any new connection?
        // 2. Is any connection which we already have is sending something?
        // 3. Are we getting any new message from the replication_stream?
        let mut rp = RESPParser::new();
        let mut client_interpreter = ReplicationInterpreter::new(None, &self.server_options.port.unwrap_or(6379));
        let mut interpreter = RESPInterpreter::new(&mut self.store, &mut self.server_options);
        self.listener.set_nonblocking(true).unwrap();
        if let Some(rs) = &self.replication_stream {
            let _ = rs.set_nonblocking(true);
        }
        loop {
            // Check 1 ie... for any new connection
            match self.listener.accept() {
                Ok(stream) => {
                    println!("New connection found {:?}", stream.1);
                    stream.0.set_nonblocking(true).unwrap();
                    self.clients.push(Client {
                        client: stream.0,
                    });
                },
                Err(_) => {}
            }

            // Check 2 looping through every client and checking for new messages
            for client in &mut self.clients {
                let mut data: [u8; 1024] = [0; 1024]; 
                let data_read = client.client.read(&mut data);
                if let Ok(read_size) = data_read {
                    if read_size == 0 {
                        continue;
                    } else {
                        let str_message = String::from_utf8(data.to_vec()).unwrap();
                        let message = str_message.trim().replace("\0", "");
                        rp.register(&message);
                        interpreter.register(&message);
                        let ds = rp.parse();
                        let response = interpreter.interpret(ds);
                        for resp in response {
                            match resp {
                                super::interpreter::InterpreterResponse::Bytes(b) => {
                                    let _ = client.client.write(&b);
                                },
                                super::interpreter::InterpreterResponse::String(s) => {
                                    let _ = client.client.write(s.as_bytes());
                                }
                            }
                        }
                    }
                }
            }

            match &mut self.replication_stream {
                Some(replication_stream) => {
                    let mut replication_data: [u8; 1024] = [0; 1024];
                    let replication_data_read = replication_stream.read(&mut replication_data);
                    if let Ok(read_size) = replication_data_read {
                        if read_size != 0 {
                            let str_message = String::from_utf8_lossy(&replication_data);
                            let message = str_message.trim().replace("\0", "");
                            rp.register(&message);
                            let ds = rp.parse();
                            client_interpreter.register(ds, &message);
                            let response = client_interpreter.interpret();
                            if let Some(response) = response {
                                let _ = replication_stream.write(response.as_bytes());
                            }
                        }
                    }
                }
                None => {}
            }
        }
    }
}
