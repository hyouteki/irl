use crate::fe::token::{Token, TokenKind};
use crate::fe::{loc::Loc, ast::*};

#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum Precedence {
    Default = 0,
    Label = 10,
    Function = 20,
    Start = 100,
}

pub struct Parser {
	pub nodes: Vec<AstNode>,
}

fn token_prec(token: &Token) -> Precedence {
	match token.kind {
		TokenKind::Function => Precedence::Function,
		TokenKind::Label => Precedence::Label,
		_ => Precedence::Default, 
	}
}

fn eat_iden(tokens: &Vec<Token>, ix: &mut usize) -> String {
	tokens[*ix].assert_token_kind(TokenKind::Iden(String::from("")));
	let name: String = match &tokens[*ix].kind {
		TokenKind::Iden(id) => id.to_string(),
		_ => unreachable!(),
	};
	*ix += 1;
	name
}

fn eat_num(tokens: &Vec<Token>, ix: &mut usize) -> i32 {
	tokens[*ix].assert_token_kind(TokenKind::Num(0));
	let num: i32 = match tokens[*ix].kind {
		TokenKind::Num(num_) => num_,
		_ => unreachable!(),
	};
	*ix += 1;
	num
}

fn eat_operand(tokens: &Vec<Token>, ix: &mut usize) -> AstNode {
	let loc: Loc = tokens[*ix].loc.clone();
	match tokens[*ix].kind {
		TokenKind::Iden(_) => AstNode::Iden(IdenAstNode{name: eat_iden(tokens, ix), loc: loc}),
		TokenKind::Num(_) => AstNode::Num(NumAstNode{num: eat_num(tokens, ix), loc: loc}),
		_ => {
			tokens[*ix].error_token_kind_mismatch(vec![
				TokenKind::Iden(String::from("")), TokenKind::Num(0)]);
			unreachable!()  
		},
	}
}

fn assert_n_eat(tokens: &Vec<Token>, expected_kind: TokenKind, ix: &mut usize) {
	tokens[*ix].assert_token_kind(expected_kind);
	*ix += 1;
}

fn parse_function(tokens: &Vec<Token>, ix: &mut usize) -> AstNode {
	let loc: Loc = tokens[*ix].loc.clone();
	assert_n_eat(tokens, TokenKind::Function, ix);
	let name: String = eat_iden(tokens, ix);
	assert_n_eat(tokens, TokenKind::Comma, ix);
	let arg_count: i32 = eat_num(tokens, ix);
	assert_n_eat(tokens, TokenKind::Eol, ix);
	let mut args: Vec<AstNode> = vec![];
	for _ in 0..arg_count {
		assert_n_eat(tokens, TokenKind::Arg, ix);
		let arg_loc: Loc = tokens[*ix].loc.clone();
		args.push(AstNode::Iden(IdenAstNode{name: eat_iden(tokens, ix), loc: arg_loc}));
		assert_n_eat(tokens, TokenKind::Eol, ix);
	}
	let body = build_ast_prec(tokens, ix, Precedence::Function);
	AstNode::Function(FunctionAstNode{name: name, args: args, body: body, loc: loc})
}

fn parse_label(tokens: &Vec<Token>, ix: &mut usize) -> AstNode {
	let loc: Loc = tokens[*ix].loc.clone();
	assert_n_eat(tokens, TokenKind::Label, ix);
	let name: String = eat_iden(tokens, ix);
	assert_n_eat(tokens, TokenKind::Eol, ix);
	let body: Vec<AstNode> = build_ast_prec(tokens, ix, Precedence::Label);
	AstNode::Label(LabelAstNode{name: name, body: body, loc: loc})
}

fn parse_goto(tokens: &Vec<Token>, ix: &mut usize) -> AstNode {
	let loc: Loc = tokens[*ix].loc.clone();
	assert_n_eat(tokens, TokenKind::Goto, ix);
	let name: String = eat_iden(tokens, ix);
	assert_n_eat(tokens, TokenKind::Eol, ix);
	AstNode::Goto(GotoAstNode{name: name, loc: loc})
}

fn parse_comment(tokens: &Vec<Token>, ix: &mut usize) {
	assert_n_eat(tokens, TokenKind::Comment, ix);
	while tokens[*ix].kind != TokenKind::Eol && tokens[*ix].kind != TokenKind::Eof {
		*ix += 1;
	}
	if tokens[*ix].kind == TokenKind::Eol {
		*ix += 1
	}
}

fn parse_assignment(tokens: &Vec<Token>, ix: &mut usize) -> AstNode {
	let loc: Loc = tokens[*ix].loc.clone();
	let id: String = eat_iden(tokens, ix);
	assert_n_eat(tokens, TokenKind::Equal, ix);
	let var_loc: Loc = tokens[*ix].loc.clone();
	if tokens[*ix].is_unary() {
		let op: UnaryOp = UnaryOp::new(&tokens[*ix]);
		*ix += 1; // eat 'unary'
		let var: AstNode = eat_operand(tokens, ix);
		assert_n_eat(tokens, TokenKind::Eol, ix);
		return AstNode::Assignment(AssignmentAstNode{name: id, var: Box::new(
			AstNode::Unary(UnaryAstNode{op: op, var: Box::new(var), loc: var_loc})), loc});
	}
	if tokens[*ix].kind == TokenKind::Call {
		assert_n_eat(tokens, TokenKind::Call, ix);
		let name: String = eat_iden(tokens, ix);
		assert_n_eat(tokens, TokenKind::Comma, ix);
		let param_count_loc: Loc = tokens[*ix].loc.clone();
		let param_count: i32 = eat_num(tokens, ix);
		if param_count != 0 {
			param_count_loc.error(format!("expected param count to be '0'; but found '{}'",
										  param_count));
			unreachable!()
		}
		assert_n_eat(tokens, TokenKind::Eol, ix);
		return AstNode::Call(CallAstNode{id: id, name: name, params: vec![], loc: loc});
	}
	if tokens[*ix].kind == TokenKind::Alloc {
		assert_n_eat(tokens, TokenKind::Alloc, ix);
		let size: AstNode = eat_operand(tokens, ix);
		assert_n_eat(tokens, TokenKind::Eol, ix);
		return AstNode::Assignment(AssignmentAstNode{name: id, var: Box::new(
			AstNode::Alloc(AllocAstNode{size: Box::new(size), loc: var_loc})), loc});
	}
	if tokens[*ix].kind == TokenKind::Load {
		assert_n_eat(tokens, TokenKind::Load, ix);
		let ptr: String = eat_iden(tokens, ix);
		assert_n_eat(tokens, TokenKind::Eol, ix);
		return AstNode::Assignment(AssignmentAstNode{name: id, var: Box::new(
			AstNode::Load(LoadAstNode{ptr: ptr, loc: var_loc})), loc});
	}
	let lhs: AstNode = eat_operand(tokens, ix);
	if tokens[*ix].is_arith() {
		let op: ArithOp = ArithOp::new(&tokens[*ix]);
		*ix += 1; // eat 'arith'
		let rhs: AstNode = eat_operand(tokens, ix);
		assert_n_eat(tokens, TokenKind::Eol, ix);
		return AstNode::Assignment(AssignmentAstNode{name: id, var: Box::new(
			AstNode::Arith(ArithAstNode{op: op, lhs: Box::new(lhs), rhs: Box::new(rhs),
										loc: var_loc})), loc});
	}
	if tokens[*ix].is_relop() {
		let op: RelOp = RelOp::new(&tokens[*ix]);
		*ix += 1; // eat 'relop'
		let rhs: AstNode = eat_operand(tokens, ix);
		assert_n_eat(tokens, TokenKind::Eol, ix);
		return AstNode::Assignment(AssignmentAstNode{name: id, var: Box::new(
			AstNode::Relop(RelopAstNode{op: op, lhs: Box::new(lhs), rhs: Box::new(rhs),
										loc: var_loc})), loc});
	}
	assert_n_eat(tokens, TokenKind::Eol, ix);
	AstNode::Assignment(AssignmentAstNode{name: id, var: Box::new(lhs), loc})
}

fn parse_ret(tokens: &Vec<Token>, ix: &mut usize) -> AstNode {
	let loc: Loc = tokens[*ix].loc.clone();
	assert_n_eat(tokens, TokenKind::Ret, ix);
	let var: AstNode = eat_operand(tokens, ix);
	assert_n_eat(tokens, TokenKind::Eol, ix);
	AstNode::Ret(RetAstNode{var: Box::new(var), loc: loc})
}

fn parse_param(tokens: &Vec<Token>, ix: &mut usize) -> AstNode {
	let mut params: Vec<AstNode> = vec![];
	while tokens[*ix].kind == TokenKind::Param {
		assert_n_eat(tokens, TokenKind::Param, ix);
		params.push(eat_operand(tokens, ix));
		assert_n_eat(tokens, TokenKind::Eol, ix);
	}
	let loc: Loc = tokens[*ix].loc.clone();
	let id: String = eat_iden(tokens, ix);
	assert_n_eat(tokens, TokenKind::Equal, ix);
	assert_n_eat(tokens, TokenKind::Call, ix);
	let name: String = eat_iden(tokens, ix);
	assert_n_eat(tokens, TokenKind::Comma, ix);
	let param_loc: Loc = tokens[*ix].loc.clone();
	let param_count: i32 = eat_num(tokens, ix);
	if params.len() as i32 != param_count {
		param_loc.error(format!("expected param count to be '{}'; but found '{}'",
								params.len(), param_count));
	}
	assert_n_eat(tokens, TokenKind::Eol, ix);
	AstNode::Call(CallAstNode{id: id, name: name, params: params, loc: loc})
}

fn parse_if(tokens: &Vec<Token>, ix: &mut usize) -> AstNode {
	let loc: Loc = tokens[*ix].loc.clone();
	assert_n_eat(tokens, TokenKind::If, ix);
	assert_n_eat(tokens, TokenKind::LParen, ix);
	let relop_loc: Loc = tokens[*ix].loc.clone();
	let lhs: AstNode = eat_operand(tokens, ix);
	let op: RelOp = RelOp::new(&tokens[*ix]);
	*ix += 1; // eat 'relop'
	let rhs: AstNode = eat_operand(tokens, ix);
	let condition: AstNode = AstNode::Relop(RelopAstNode{op: op, lhs: Box::new(lhs),
														 rhs: Box::new(rhs), loc: relop_loc});
	assert_n_eat(tokens, TokenKind::RParen, ix);
	assert_n_eat(tokens, TokenKind::Goto, ix);
	let name: String = eat_iden(tokens, ix);
	assert_n_eat(tokens, TokenKind::Eol, ix);
	return AstNode::If(IfAstNode{condition: Box::new(condition), label: name, loc: loc})

}

fn build_ast_prec(tokens: &Vec<Token>, ix: &mut usize, prec: Precedence) -> Vec<AstNode> {
	let mut nodes: Vec<AstNode> = vec![];
	while tokens[*ix].kind != TokenKind::Eof && token_prec(&tokens[*ix]) < prec {
		match tokens[*ix].kind {
			TokenKind::Function => nodes.push(parse_function(&tokens, ix)),
			TokenKind::Label => nodes.push(parse_label(&tokens, ix)),
			TokenKind::Goto => nodes.push(parse_goto(&tokens, ix)),
			TokenKind::Iden(_) => nodes.push(parse_assignment(&tokens, ix)),
			TokenKind::Comment => parse_comment(&tokens, ix),
			TokenKind::Ret => nodes.push(parse_ret(&tokens, ix)),
			TokenKind::Param => nodes.push(parse_param(&tokens, ix)),
			TokenKind::If => nodes.push(parse_if(&tokens, ix)),
			TokenKind::Eol => {*ix += 1;},
			_ => tokens[*ix].error_token_kind_mismatch(
				vec![TokenKind::Function, TokenKind::Label, TokenKind::Goto,
					 TokenKind::Iden(String::from("")), TokenKind::Ret,
					 TokenKind::Param, TokenKind::If, TokenKind::Eol])
		};
	}
	nodes
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		let mut ix: usize = 0;
		Self{nodes: build_ast_prec(&tokens, &mut ix, Precedence::Start)}
	}
}
