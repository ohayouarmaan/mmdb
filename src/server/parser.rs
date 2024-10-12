#[derive(Debug)]
pub enum DS {
    RedArray(RedArray),
    Integer(u32),
    String(usize, usize)
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
    pub fn new(source_code: &str) -> Self {
        Self {
            source_code: source_code.to_string(),
            current_index: 0
        }
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
            _ => {
                return DS::Integer(2);
            }
        }
    }

    fn advance(&mut self) {
        if self.current_index < self.source_code.len() {
            self.current_index += 1;
            let mut curr_character = self.source_code.chars().nth(self.current_index).unwrap();
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
        
        return DS::String(self.current_index - str_len, self.current_index);
    }
}
