use crate::fe::{loc::Loc};

#[derive(PartialEq, Clone)]
pub enum TokenKind {
	Function,
	Iden(String),
	Num(i32),
	Comma, 
	Arg,
	Equal,
	Plus,
	Minus,
	Mul,
	Comment,
	Alloc,
	Load,
	Store,
	Div,
	Goto,
	Label,
	If,
	LParen,
	RParen,
	Eq,
	Neq,
	Gt,
	Lt,
	Ge,
	Le,
	Param,
	Call,
	Ret,
	Eol,
	Eof,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {	
			TokenKind::Function => "function",
			TokenKind::Iden(_) => "iden",
			TokenKind::Num(_) => "num",
			TokenKind::Comma => ",",
			TokenKind::Arg => "arg",
			TokenKind::Equal => "=",
			TokenKind::Plus => "+",
			TokenKind::Minus => "-",
			TokenKind::Mul => "*",
			TokenKind::Div => "/",
			TokenKind::Comment => "//",
			TokenKind::Alloc => "alloc",
			TokenKind::Load => "load",
			TokenKind::Store => "store",
			TokenKind::Goto => "goto",
			TokenKind::Label => "label",
			TokenKind::If => "if",
			TokenKind::LParen => "(",
			TokenKind::RParen => ")",
			TokenKind::Eq => "==",
			TokenKind::Neq => "!=",
			TokenKind::Gt => ">",
			TokenKind::Lt => "<",
			TokenKind::Ge => ">=",
			TokenKind::Le => "<=",
			TokenKind::Param => "param",
			TokenKind::Call => "call",
			TokenKind::Ret => "ret",
			TokenKind::Eol => "EOL",
			TokenKind::Eof => "EOF",
        })
    }
}

pub struct Token {
	pub kind: TokenKind,
	pub loc: Loc,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.loc, self.kind)
	}
}

impl Token {
    pub fn new(kind: TokenKind, loc: Loc) -> Self {
        Self{kind, loc}
    }
	pub fn is_arith(&self) -> bool {
		match self.kind {
			TokenKind::Plus | TokenKind::Minus
				| TokenKind::Mul | TokenKind::Div => true,
			_ => false
		}
	}
	pub fn is_relop(&self) -> bool {
		match self.kind {
			TokenKind::Eq | TokenKind::Neq | TokenKind::Gt
				| TokenKind::Lt | TokenKind::Ge | TokenKind::Le => true,
			_ => false
		}
	}
	pub fn is_unary(&self) -> bool {
		match self.kind {
			TokenKind::Minus => true,
			_ => false
		}
	}
	pub fn error_token_kind_mismatch(&self, expected_kinds: Vec<TokenKind>) {
		match expected_kinds.len() {
			0 => self.loc.error(format!("expected no token; but got token of kind '{}'", self.kind)),
			1 => self.loc.error(format!("expected token of kind '{}'; but got '{}'",
										expected_kinds[0], self.kind)),
			_ => {
				let expected_kinds_string: String = expected_kinds.iter()
					.map(|expected_kind| format!("{}", expected_kind))
					.collect::<Vec<String>>()
					.join(" | ");
				self.loc.error(format!("expected token of kinds '{}'; but got '{}'",
										expected_kinds_string, self.kind))
			}
		}
	}
	pub fn assert_token_kind(&self, expected_kind: TokenKind) {
		let kind_matches: bool = match (&self.kind, expected_kind.clone()) {
            (TokenKind::Iden(_), TokenKind::Iden(_)) => true,
            (TokenKind::Num(_), TokenKind::Num(_)) => true,
            _ => self.kind == expected_kind,
        };
		if !kind_matches {
			self.error_token_kind_mismatch(vec![expected_kind]);
		}
	}
}
