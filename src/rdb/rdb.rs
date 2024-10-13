use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{fs, path};
use std::collections::{VecDeque,HashMap};

use crate::datastore::store::DataItem;
use crate::server::ServerOptions;

pub struct RDBFileHelper {
    file_path: Option<path::PathBuf>
}

impl RDBFileHelper {
    pub fn new(server_configuration: ServerOptions) -> Self {
        let rdb_file_path: Option<path::PathBuf> = match server_configuration.rdb_dir_name {
            Some(dir_name) => {
                match server_configuration.rdb_file_name {
                    Some(file_name) => {
                        Some(dir_name.join(file_name))
                    },
                    None => {
                        None
                    }
                }
            },
            None => {
                None
            }
        };
        Self {
            file_path: rdb_file_path
        }
    }

    pub fn read_file(&mut self) -> Result<Vec<u8>, ()> {
        match &self.file_path {
            Some(file_name) => {
                match fs::read(file_name) {
                    Ok(result) => {
                        Ok(result)
                    },
                    Err(_) => {
                        println!("ERROR While reading rdb file");
                        Err(())
                    }
                }
            },
            None => {
                Err(())
            }
        }
    }

    pub fn get_kv_table(&mut self) -> Result<Vec<u8>, ()> {
        let file_content = self.read_file()?;
        let kp = file_content.iter().position(|x| *x == 0xfe).ok_or(())? + 2;
        let ending = file_content.iter().position(|x| *x == 0xff).ok_or(())?;
        let kv_table = file_content.get(kp+3..ending).ok_or(())?.to_vec();
        Ok(kv_table)
    }

    pub fn decode_kv_table(&mut self) -> Result<HashMap<String, DataItem>, ()> {
        let mut kv_table = VecDeque::from(self.get_kv_table()?);
        let mut data_stored: HashMap<String, DataItem> = HashMap::new();
        while let Some(mut key_length) = kv_table.pop_front() {
            let mut expiry_value: Option<SystemTime> = None;
            if key_length == 0xfc {
                let mut exp: [u8; 8] = [0; 8];
                for i in 0..8 {
                    let v = kv_table.pop_front().ok_or(())?;
                    exp[i] = v;
                }
                let exp = u64::from_le_bytes(exp);
                let system_time = UNIX_EPOCH + Duration::from_millis(exp);
                expiry_value = Some(system_time);
            }
            key_length = kv_table.pop_front().ok_or(())?;
            if key_length == 0 {
                key_length = kv_table.pop_front().ok_or(())?;
            }
            let mut key_string = String::from("");
            for _ in 0..key_length {
                let f = kv_table.pop_front().ok_or(())?;
                key_string.push(char::from(f));
            }
            let value_length = kv_table.pop_front().ok_or(())?;

            let mut value_string = String::from("");
            for _ in 0..value_length {
                let f = kv_table.pop_front().ok_or(())?;
                value_string.push(char::from(f));
            }
            data_stored.insert(key_string, DataItem {
                data: value_string,
                expiry: expiry_value
            });
        }
        Ok(data_stored)
    }
}
