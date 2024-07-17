use std::io::{prelude::*, BufReader};
use std::collections::HashMap;
use crate::fe::{token::{Token, TokenKind}, loc::Loc};

pub struct Lexer {
	pub tokens: Vec<Token>
}

impl Lexer {
	pub fn new(filepath: String) -> Self {
		let mut tokens: Vec<Token> = Vec::new();
		let mut content: Vec<String> = BufReader
			::new(std::fs::File::open(&filepath).expect("could not open file"))
			.lines()
			.map(|line| line.expect("could not parse file"))
			.collect();
		
		if content.is_empty() {
			return Self{tokens};
		}

		let mut row: usize = 0;
		let mut col: usize = 0;

		let operator_table: Vec<(String, TokenKind)> = vec![
			(String::from("=="), TokenKind::Eq),
			(String::from("="), TokenKind::Equal),
			(String::from("+"), TokenKind::Plus),
			(String::from("-"), TokenKind::Minus),
			(String::from("*"), TokenKind::Mul),
			(String::from("//"), TokenKind::Comment),
			(String::from("/"), TokenKind::Div),
			(String::from("!="), TokenKind::Neq),
			(String::from(">="), TokenKind::Ge),
			(String::from("<="), TokenKind::Le),
			(String::from(">"), TokenKind::Gt),
			(String::from("<"), TokenKind::Lt),
			(String::from(","), TokenKind::Comma),
			(String::from("("), TokenKind::LParen),
			(String::from(")"), TokenKind::RParen),
		];

		let keyword_table: HashMap<String, TokenKind> = [
			(String::from("function"), TokenKind::Function),
		    (String::from("arg"), TokenKind::Arg),
		    (String::from("goto"), TokenKind::Goto),
		    (String::from("label"), TokenKind::Label),
		    (String::from("if"), TokenKind::If),
		    (String::from("param"), TokenKind::Param),
		    (String::from("call"), TokenKind::Call),
		    (String::from("ret"), TokenKind::Ret)
		].iter().cloned().collect();

		for i in 0..content.len() {
			row = i+1;
			col = 1;

			while !content[i].is_empty() {

				let mut l: usize = 0;
				for c in content[i].chars() {
					if c.is_whitespace() {
						l += 1;
					} else {
						break;
					}
				}
				content[i] = content[i][l..].to_string();
				if content[i].is_empty() {
					break;
				}
				col += l;
				let loc: Loc = Loc::new(row, col, filepath.clone());

				let mut flag: bool = false;
				for (operator, token_kind) in &operator_table {
					if content[i].starts_with(operator) {
						tokens.push(Token::new(token_kind.clone(), loc.clone()));
						content[i] = content[i][operator.len()..].to_string();
						col += operator.len();
						flag = true;
						break;
					}
				}
				if flag {
					continue;
				}

				if content[i].chars().nth(0).unwrap().is_alphabetic() {
					let mut keyword_sz: usize = 1;
					while keyword_sz < content[i].len() &&
						content[i].chars().nth(keyword_sz).unwrap().is_alphanumeric() {
						keyword_sz += 1;
					}
					let keyword: String = content[i][0..keyword_sz].to_string();
					match keyword_table.get(&keyword) {
						Some(token_kind) => tokens.push(Token::new(token_kind.clone(), loc.clone())),
						None => tokens.push(Token::new(TokenKind::Iden(keyword), loc.clone())),
					}
					content[i] = content[i][keyword_sz..].to_string();
					col += keyword_sz;
					continue;
				}

				if content[i].chars().nth(0).unwrap().is_numeric() {
					let mut num_sz: usize = 0;
					let mut num: i32 = 0;
					while num_sz < content[i].len() && content[i].chars().nth(num_sz).unwrap().is_numeric() {
						num = num*10 + content[i].chars().nth(num_sz).unwrap() as i32 - '0' as i32;
						num_sz += 1;
					}
					tokens.push(Token::new(TokenKind::Num(num), loc.clone()));
					content[i] = content[i][num_sz..].to_string();
					col += num_sz;
					continue;
				}

				loc.error(format!("unexpected token '{}'", content[i].chars().nth(0).unwrap()));
			}
			tokens.push(Token::new(TokenKind::Eol, Loc::new(row, col, filepath.clone())));
		}
		tokens.push(Token::new(TokenKind::Eof, Loc::new(row, col, filepath.clone())));

		return Self{tokens};
	}
}
