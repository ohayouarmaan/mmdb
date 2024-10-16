use crate::server::interpreter::Reply;

pub struct Helper;

impl Helper {
    pub fn build_resp(reply: &Reply) -> String {
        match reply {
            Reply::ReplyString(s) => {
                let mut response = String::from("+");
                response.push_str(&(s).to_string());
                response.push_str("\r\n");
                return response;
            },
            Reply::ReplyArray(arr_data) => {
                let mut response = String::from("*");
                response.push_str(&(arr_data.len() as u8).to_string());
                response.push_str("\r\n");
                for d in arr_data {
                    let x = Helper::build_resp(d);
                    response.push_str(&x);
                }
                return response;
            },
            Reply::ReplyBulkString(s) => {
                let mut response = String::from("$");
                response.push_str(&(s.len()).to_string());
                response.push_str("\r\n");
                response.push_str(&(s).to_string());
                response.push_str("\r\n");
                return response;
            }
        }
    }

}
