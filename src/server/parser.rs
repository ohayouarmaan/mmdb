#[derive(Debug)]
pub enum DS {
    RedArray(RedArray),
    Integer(u32),
    String(usize, usize)
}


#[derive(Debug)]
pub struct RedArray {
    length: usize,
    value: Vec<DS>
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
        let mut current_token = "";
        match self.source_code.chars().nth(self.current_index).unwrap() {
            '*' => {
                // Parse Array
                println!("PARSING ARRAY");
                return self.parse_array();
            },
            '$' => {
                println!("PARSING STRING");
                return self.parse_string();
            }, 
            _ => {
                return DS::Integer(2);
            }
        }
        self.advance();
        return DS::Integer(2);
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
        let number_of_elements = (self.source_code.chars().nth(self.current_index).unwrap()).to_digit(10).expect("Expected a number");
        println!("NUMBER OF ELEMENTS: {:?}", number_of_elements);
        self.advance();
        let mut tokens: Vec<DS> = Vec::new();
        for i in 0..number_of_elements {
            println!("parsing {}", i);
            tokens.push(self.parse());
        }
        return DS::RedArray(RedArray {
            length: number_of_elements as usize,
            value: tokens
        })
    }

    fn parse_string(&mut self) -> DS {
        self.advance();
        let str_len = (self.source_code.chars().nth(self.current_index).unwrap()).to_digit(10).expect("Expected a number") as usize;
        self.advance();
        println!("Str len: {:?}", str_len);
        self.current_index += str_len;
        println!("parsed_string: {:?}", self.source_code.get((self.current_index - str_len)..self.current_index));
        self.advance();
        return DS::String(self.current_index - str_len, self.current_index - 2);
    }
}
