mod server;
mod datastore;

use crate::server::{Server, ServerOptions};

use std::collections::VecDeque;

fn main() {
    let mut args: VecDeque<_> = VecDeque::from(std::env::args().skip(1).collect::<Vec<_>>());
    println!("Args: {:?}", args);
    let mut server_options: ServerOptions = ServerOptions {
        rdb_file_name: None,
        rdb_dir_name: None
    };
    while let Some(option) = args.pop_front() {
        if option == "--dir" {
            let rdb_dir_name = args.pop_front();
            server_options.rdb_dir_name = Some(std::path::PathBuf::from(rdb_dir_name.expect("Expected a value for the passed argument").to_owned()));
        }
        if option == "--dbfilename" {
            let rdb_db_file_name = args.pop_front();
            server_options.rdb_file_name = Some(std::path::PathBuf::from(rdb_db_file_name.expect("Expected a value for the passed argument").to_owned()));
        }
    }

    println!("Server Options: {:?}", server_options);
    let mut server = Server::new("127.0.0.1:6379", server_options);
    server.run_event_loop();
}
