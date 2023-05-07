use crate::token::{self, Token};

pub struct Lexer {
    input: Vec<char>,
    position: i32,
    read_position: usize,
    pub ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect::<Vec<char>>(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();

        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position as i32;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let mut token = token::ILLEGAL;

        token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    token::EQ
                } else {
                    token::ASSIGN
                }
            }

            ';' => token::SEMICOLON,
            '(' => token::LPAREN,
            ')' => token::RPAREN,
            ',' => token::COMMA,
            '+' => token::PLUS,
            '-' => token::MINUS,
            '!' => {
                if self.peek_char() == '=' {
                    token::NOT_EQ
                } else {
                    token::BANG
                }
            }
            '/' => token::SLASH,
            '*' => token::ASTERISK,
            '<' => token::LT,
            '>' => token::GT,
            '{' => token::LBRACE,
            '}' => token::RBRACE,
            '\0' => token::EOF,
            _ => {
                if Self::is_letter(self.ch) {
                    token::IDENT
                } else if Self::is_digit(self.ch) {
                    token::INT
                } else {
                    token::ILLEGAL
                }
            }
        };

        let mut token = Token::new(token, self.ch.to_string());
        match token.r#type {
            token::EQ => {
                let ch = self.ch;
                self.read_char();
                token.literal = ch.to_string() + &self.ch.to_string();
            }
            token::IDENT => {
                token.literal = self.read_ident();
                token.r#type = Token::lookup_ident(&token.literal);
            }
            token::INT => {
                token.literal = self.read_number();
            }
            _ => {
                self.read_char();
            }
        }

        token
    }

    fn read_ident(&mut self) -> String {
        let position = self.position;
        while Self::is_letter(self.ch) {
            self.read_char();
        }
        self.input[position as usize..self.position as usize]
            .into_iter()
            .collect::<String>()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while Self::is_digit(self.ch) {
            self.read_char();
        }
        self.input[position as usize..self.position as usize]
            .into_iter()
            .collect::<String>()
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        }
    }
    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }
    fn is_letter(ch: char) -> bool {
        ('a' <= ch && ch <= 'z') || ('A' <= ch && ch <= 'Z') || ch == '_'
    }

    fn is_digit(ch: char) -> bool {
        '0' <= ch && ch <= '9'
    }
}
