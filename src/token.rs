use once_cell::sync::{Lazy, OnceCell};
use std::{collections::hash_map::HashMap, hash, sync::Once};
pub use TokenType::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,   //=
    PLUS,     //+
    MINUS,    //-
    BANG,     //\!
    ASTERISK, //\*
    SLASH,
    ///
    LT, //<
    GT, //>
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
    EQ,
    NOT_EQ
}

#[derive(Debug)]
pub struct Token {
    pub r#type: TokenType,
    pub literal: String,
}

static keywords: Lazy<HashMap<&str, TokenType>> = Lazy::new(|| {
    let mut hashmap = HashMap::new();
    hashmap.insert("fn", TokenType::FUNCTION);
    hashmap.insert("let", TokenType::LET);
    hashmap.insert("true", TokenType::TRUE);
    hashmap.insert("false", TokenType::FALSE);
    hashmap.insert("if", TokenType::IF);
    hashmap.insert("else", TokenType::ELSE);
    hashmap.insert("return", TokenType::RETURN);

    hashmap
});

impl Token {
    pub fn new(r#type: TokenType, value: String) -> Self {
        Self {
            r#type,
            literal: value,
        }
    }

    pub fn lookup_ident(key: &str) -> TokenType {
        if let Some(token) = keywords.get(key) {
            *token
        } else {
            TokenType::IDENT
        }
    }
}
