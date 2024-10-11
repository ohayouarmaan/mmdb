use std::net::TcpListener;
use std::io::prelude::*;

fn main() {
    println!("Logs from your program will appear here!");

   
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
   
     for stream in listener.incoming() {
        match stream {
            Ok(mut valid_stream) => {
                println!("accepted new connection");
                let mut data: [u8; 1024] = [0; 1024];
                loop {
                    let read_bytes = valid_stream.read(&mut data).unwrap();
                    if read_bytes != 0 {
                        println!("rb: {:?}", read_bytes);
                        valid_stream.write("+PONG\r\n".as_bytes());
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
