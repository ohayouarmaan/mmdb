use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

use crate::datastore::store::DataStore;
use crate::server::parser::RESPParser;
use crate::server::interpreter::RESPInterpreter;


pub struct Server {
    listener: TcpListener,
    clients: Vec<Client>,
    store: DataStore
}

pub struct Client {
    pub client: TcpStream,
}

impl Server {
    pub fn new(address: &str) -> Self {
        Self {
            listener: TcpListener::bind(address).unwrap(),
            clients: vec![],
            store: DataStore::new()
        }
    }

    pub fn run_event_loop(&mut self) {
        // Firstly, we will have to loop indefinitely and in every step we will check for two
        // things
        // 1. Do we have any new connection?
        // 2. Is any connection which we already have is sending something
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
                        let mut rp = RESPParser::new(&message);
                        let ds = rp.parse();
                        let mut interpreter = RESPInterpreter::new(&message, &mut self.store);
                        let response = interpreter.interpret(ds);
                        let _ = client.client.write(response.as_bytes());
                    }
                }
            }
        }
    }
}
