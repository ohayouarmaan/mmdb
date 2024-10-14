mod server;
mod datastore;
mod rdb;

use datastore::store::DataStore;

use crate::server::{Server, ServerOptions};
use crate::rdb::rdb::RDBFileHelper;

use std::collections::VecDeque;

fn main() {
    let mut args: VecDeque<_> = VecDeque::from(std::env::args().skip(1).collect::<Vec<_>>());
    println!("Args: {:?}", args);
    let mut server_options: ServerOptions = ServerOptions {
        rdb_file_name: None,
        rdb_dir_name: None,
        port: None
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

        if option == "--port" {
            let port = args.pop_front().expect("Expected a value to be passed for port");
            server_options.port = Some(port.parse().expect("Expected a number"));
        }
    }

    println!("Server Options: {:?}", server_options);
    let mut rdb_helper = RDBFileHelper::new(server_options.clone());
    let exisisting_db = match rdb_helper.decode_kv_table() {
        Ok(x) => {
            DataStore {
                memory: x
            }
        },
        Err(_) => {
            DataStore::new()
        }
    };
    let mut server = Server::new(&format!("127.0.0.1:{}", server_options.port.unwrap_or(6379)), server_options, Some(exisisting_db));
    server.run_event_loop();
}
