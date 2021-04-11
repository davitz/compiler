
use std::str;
use substring::Substring;
use regex::Regex;


#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
#[allow(dead_code)]
pub enum Token<'a> {
    If,
    Then,
    Else,
    EndIf,
    Print,
    While,
    Do,
    EndDo,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Int,
    RelOp(&'a str),
    MultOp(char),
    AddingOp(char),
    Digit(char),
    Letter(char),
    Empty
}

//identifies and returns the next token along with the number of characters that went into it
fn next_token(code: &str) -> (Vec<Token>,usize) {

    lazy_static!{
	static ref IF: Regex = Regex::new(r"^if([^[:alnum:]]|$)").unwrap();
	static ref THEN: Regex = Regex::new(r"^then([^[:alnum:]]|$)").unwrap();
	static ref ELSE: Regex = Regex::new(r"^else([^[:alnum:]]|$)").unwrap();
	static ref ENDIF: Regex = Regex::new(r"^endif([^[:alnum:]]|$)").unwrap();
	static ref PRINT: Regex = Regex::new(r"^print([^[:alnum:]]|$)").unwrap();
	static ref WHILE: Regex = Regex::new(r"^while([^[:alnum:]]|$)").unwrap();
	static ref DO: Regex = Regex::new(r"^do([^[:alnum:]]|$)").unwrap();
	static ref ENDDO: Regex = Regex::new(r"^enddo([^[:alnum:]]|$)").unwrap();
	static ref OPENPAREN: Regex = Regex::new(r"^\(").unwrap();
	static ref CLOSEPAREN: Regex = Regex::new(r"^\)").unwrap();
	static ref OPENBRACE: Regex = Regex::new(r"^\{").unwrap();
	static ref CLOSEBRACE: Regex = Regex::new(r"^\}").unwrap();
	static ref INT: Regex = Regex::new(r"^int([^[:alnum:]]|$)").unwrap();
	static ref RELOP: Regex = Regex::new(r"^(<|<=|==|>=|>|!=)").unwrap();
	static ref MULTOP: Regex = Regex::new(r"^[*/]").unwrap();
	static ref ADDINGOP: Regex = Regex::new(r"^[+-]").unwrap();
	static ref ALPHANUM: Regex = Regex::new(r"^[[:alnum:]]+([^[:alnum:]]|$)").unwrap();
    }
    
    match code {
	_ if IF.is_match(code) => (vec![Token::If],2usize),
	_ if THEN.is_match(code) => (vec![Token::Then],4usize),
	_ if ELSE.is_match(code) => (vec![Token::Else],4usize),
	_ if ENDIF.is_match(code) => (vec![Token::EndIf],5usize),
	_ if PRINT.is_match(code) => (vec![Token::Print],5usize),
	_ if WHILE.is_match(code) => (vec![Token::While],5usize),
	_ if DO.is_match(code) => (vec![Token::Do],2usize),
	_ if ENDDO.is_match(code) => (vec![Token::EndDo],5usize),
	_ if OPENPAREN.is_match(code) => (vec![Token::OpenParen],1usize),
	_ if CLOSEPAREN.is_match(code) => (vec![Token::CloseParen],1usize),
	_ if OPENBRACE.is_match(code) => (vec![Token::OpenBrace],1usize),
	_ if CLOSEBRACE.is_match(code) => (vec![Token::CloseBrace],1usize),
	_ if INT.is_match(code) => (vec![Token::Int],3usize),
	_ if RELOP.is_match(code) => {
	    let match_str = RELOP.find(code).unwrap().as_str();
	    (vec![Token::RelOp(match_str)], match_str.chars().count())
	}
	_ if MULTOP.is_match(code) => (vec![Token::MultOp(MULTOP.find(code).unwrap().as_str().chars().next().unwrap())],1usize),
	_ if ADDINGOP.is_match(code) => (vec![Token::AddingOp(ADDINGOP.find(code).unwrap().as_str().chars().next().unwrap())],1usize),
	
	
	_ if ALPHANUM.is_match(code) => {
	    let mat = ALPHANUM.find(code);
	    
	    let mat_str = mat.unwrap().as_str();
	    
	    let mut tokens: Vec<Token> = vec![Token::Empty];
	    let mut len_count = 0usize;
	    
	    for c in mat_str.chars() {
		let t = match c {
		    'a'..='z' | 'A'..='Z' => Token::Letter(c),
		    '0'..='9' => Token::Digit(c),
		    _ => Token::Empty
		};
		tokens.push(t);
		len_count += 1;
	    }
	    (tokens,len_count)
	},	
	_ => (vec![Token::Empty],1usize)
    }
}

// lexes a block of text
pub fn lex(code: &str) -> Vec<Token> {
    let mut result = Vec::new();
    
    let lines = code.split("\n");
    
    for line in lines {
	let mut index = 0usize;
	let length = line.chars().count();
	while index < length {
	    let (new_tokens, len) = next_token(line.substring(index, length));
	    result.extend(new_tokens);
	    index += len;
	}
    }
	
    return result;
}
    

