#[derive(Debug)]
pub enum DS {
    RedArray(RedArray),
    String(usize, usize),
    BulkString(usize, usize)
}

impl DS {
    pub fn debug(&self, source_code: &str) {
        match self {
            Self::String(start, end) => {
                println!("string: {:?}", source_code.get(*start..*end).expect("-ERROR Expected a value found nothing\r\n"));
            },
            _ => {
                println!("{:?}", self);
            }
        }
    }

    pub fn get_value(&self, source_code: &str) -> String {
        match self {
            Self::BulkString(start, end) => {
                return source_code.get(*start..*end).expect("-ERROR Expected a value found nothing\r\n").to_owned();
            },
            Self::String(start, end) => {
                return source_code.get(*start..*end).expect("-ERROR Expected a value found nothing\r\n").to_owned();
            },
            _ => {
                return format!("{:?}", self);
            }
        }
    }

    pub fn get_type(&self) -> String {
        match self {
            Self::String(_, _) => {
                return String::from("string");
            },
            Self::BulkString(_, _) => {
                return String::from("string");
            },
            Self::RedArray(_) => {
                return String::from("list");
            },
        }
    }
}

#[derive(Debug)]
pub struct RedArray {
    pub length: usize,
    pub value: Vec<DS>
}

#[derive(Debug)]
pub struct RESPParser {
    source_code: String,
    current_index: usize
}

impl RESPParser {
    pub fn new() -> Self {
        Self {
            source_code: String::from(""),
            current_index: 0
        }
    }

    pub fn register(&mut self, source_code: &str) {
        self.source_code = source_code.to_string();
        self.current_index = 0;
    }

    pub fn parse(&mut self) -> DS {
        match self.source_code.chars().nth(self.current_index).unwrap() {
            '*' => {
                // Parse Array
                return self.parse_array();
            },
            '$' => {
                // Parse Bulk String
                let s = self.parse_string();
                self.advance();
                return s;
            }, 
            '+' => {
                self.parse_simple_string()
            }
            c => {
                unreachable!("found: {}", c);
                //return DS::Integer(2);
            }
        }
    }

    fn parse_simple_string(&mut self) -> DS {
        self.advance();
        let mut end_index = 0;
        let start_index = self.current_index;
        loop {
            match self.source_code.chars().nth(self.current_index) {
                Some(curr_character) => {
                    if curr_character == '\r' {
                        self.advance();
                        let curr_character = self.source_code.chars().nth(self.current_index).unwrap();
                        if curr_character == '\n' {
                            self.advance();
                            break
                        }
                    }
                    end_index += 1;
                    self.advance();
                },
                None => break
            };
        }
        return DS::String(start_index, start_index + end_index)
    }

    fn advance(&mut self) {
        if self.current_index <= self.source_code.len() - 1 {
            self.current_index += 1;
            let mut curr_character = self.source_code.chars().nth(self.current_index).unwrap_or_else(|| panic!("AAAAAAAA: {}", self.source_code));
            while curr_character == '\r' || curr_character == '\n' && self.current_index < self.source_code.len() {
                self.current_index += 1;
                match self.source_code.chars().nth(self.current_index) {
                    None => {
                        break;
                    },
                    Some(x) => {
                        curr_character = x;
                    }
                };
            }
        }
    }

    fn parse_array(&mut self) -> DS {
        self.advance();
        let number_of_elements = self.parse_number();
        self.advance();
        let mut tokens: Vec<DS> = Vec::new();
        for _ in 0..number_of_elements {
            tokens.push(self.parse());
        }
        return DS::RedArray(RedArray {
            length: number_of_elements as usize,
            value: tokens
        })
    }

    fn parse_number(&mut self) -> u32 {
        let mut x = String::from("");
        while '0' <= self.source_code.chars().nth(self.current_index).expect("expected a char") && self.source_code.chars().nth(self.current_index).expect("expected a char") <= '9' {
            x.push(self.source_code.chars().nth(self.current_index).expect("Expected a character"));
            self.current_index += 1;
        }

        let to_return: u32 = x.parse().unwrap();
        return to_return;
    }

    fn parse_string(&mut self) -> DS {
        self.advance();
        let str_len = self.parse_number() as usize;
        self.advance();
        self.current_index += str_len;
        
        return DS::BulkString(self.current_index - str_len, self.current_index);
    }
}
