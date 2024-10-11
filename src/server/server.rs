use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use crate::server::parser::Command;

pub struct Server {
    listener: TcpListener,
    clients: Vec<TcpStream>
}

impl Server {
    pub fn new(address: &str) -> Self {
        Self {
            listener: TcpListener::bind(address).unwrap(),
            clients: vec![]
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
                    self.clients.push(stream.0);
                },
                Err(_) => {}
            }

            // Check 2 looping through every client and checking for new messages
            for client in &mut self.clients {
                let mut data: [u8; 1024] = [0; 1024]; 
                let data_read = client.read(&mut data);
                if let Ok(read_size) = data_read {
                    if read_size == 0 {
                        continue;
                    } else {
                        let message = String::from_utf8(data.to_vec()).unwrap();
                        let cmd = Command::from_message(message);
                        let response = cmd.generate_response();
                        let _ = client.write(&response);
                    }
                }
            }
        }
    }

    pub fn listen_incoming(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut valid_stream) => {
                    println!("accepted new connection");
                    let mut data: [u8; 1024] = [0; 1024];
                    loop {
                        let read_bytes = valid_stream.read(&mut data).unwrap();
                        if read_bytes != 0 {
                            println!("rb: {:?}", read_bytes);
                            let _ = valid_stream.write("+PONG\r\n".as_bytes());
                        }
                    }
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        }
    }
}
