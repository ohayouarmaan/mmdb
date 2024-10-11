use std::net::TcpListener;
use std::io::prelude::*;

fn main() {
    println!("Logs from your program will appear here!");

   
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
   
     for stream in listener.incoming() {
        match stream {
            Ok(mut valid_stream) => {
                println!("accepted new connection");
                let mut data: Vec<u8> = Vec::new();
                let data_read = valid_stream.read(&mut data);
                if let Ok(read) = data_read {
                    valid_stream.write("+PONG\r\n".as_bytes());
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
