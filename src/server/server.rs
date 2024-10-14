use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

use crate::datastore::store::DataStore;
use crate::server::parser::RESPParser;
use crate::server::interpreter::RESPInterpreter;

#[derive(Debug,Clone)]
pub struct SlaveServerOptions {
    pub master_host: String,
    pub master_port: u32,
}

#[derive(Debug,Clone)]
pub enum ServerRole {
    Master,
    Slave(SlaveServerOptions)
}

#[derive(Debug,Clone)]
pub struct ServerOptions {
    pub rdb_file_name: Option<std::path::PathBuf>,
    pub rdb_dir_name: Option<std::path::PathBuf>,
    pub port: Option<u32>,
    pub server_role: Option<ServerRole>
}

pub struct Server {
    listener: TcpListener,
    clients: Vec<Client>,
    store: DataStore,
    pub server_options: ServerOptions
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
                    server_options: server_opts
                }
            },
            None => {
                Self {
                    listener: TcpListener::bind(address).unwrap(),
                    clients: vec![],
                    store: DataStore::new(),
                    server_options: server_opts
                }
            }
        }
    }

    pub fn run_event_loop(&mut self) {
        // Firstly, we will have to loop indefinitely and in every step we will check for two
        // things
        // 1. Do we have any new connection?
        // 2. Is any connection which we already have is sending something
        let mut rp = RESPParser::new();
        let mut interpreter = RESPInterpreter::new(&mut self.store, &mut self.server_options);
        loop {
            // Check 1 ie... for any new connection
            self.listener.set_nonblocking(true).unwrap();
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
                        let _ = client.client.write(response.as_bytes());
                    }
                }
            }
        }
    }
}
