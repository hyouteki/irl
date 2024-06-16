#include "parser.hpp"

#define Prec_Function 20
#define Prec_Label 10
#define Prec_Default 0
#define Prec_Start 100

static std::vector<std::unique_ptr<AstNode>> BuildAstPrec(const std::vector<Token> tokens, size_t &i, int prec);

static int InstructionPrecedence(const Token token) {
	switch (token.type) {
	case Token::Type::Function: return Prec_Function;
	case Token::Type::Label: return Prec_Label;
	default: return Prec_Default;
	}
}

static std::unique_ptr<AstNode> GetOperand(const Token token) {
	switch (token.type) {
	case Token::Type::Iden:
		return std::make_unique<IdenAstNode>(token.content);
	case Token::Type::Num:
		return std::make_unique<NumAstNode>((double)std::stoi(token.content));
	default:
		token.ErrorTypeMismatch({Token::Type::Iden, Token::Type::Num});
		return {};
	}
}

static std::unique_ptr<AstNode> ParseIden(const std::vector<Token> tokens, size_t &i) {
	std::unique_ptr<AstNode> id = std::make_unique<IdenAstNode>(tokens[i].content);
	i++; // eat 'iden' 
	tokens[i].AssertTokenType(Token::Type::Equal);
	i++; // eat '='
	if (tokens[i].IsUnary()) {
		std::string unary = tokens[i].content;
		i++; // eat 'unary'
		std::unique_ptr<AstNode> iop = GetOperand(tokens[i]);
		i++; // eat 'op'
		std::unique_ptr<AstNode> op = std::make_unique<UnaryAstNode>(unary, std::move(iop));
		tokens[i].AssertTokenType(Token::Type::Eol);
		i++; // eat 'EOL'
		return std::make_unique<AssignmentAstNode>(std::move(id), std::move(op));
	}
	if (tokens[i].type == Token::Type::Call) {
		i++; // eat 'call'
		tokens[i].AssertTokenType(Token::Type::Iden);
		std::unique_ptr<AstNode> name = std::make_unique<IdenAstNode>(tokens[i].content);
		i++; // eat 'iden'
		tokens[i].AssertTokenType(Token::Type::Comma);
		i++; // eat ','
		tokens[i].AssertTokenType(Token::Type::Num);
		size_t argCount = (size_t)std::stoi(tokens[i].content);
		if (argCount != 0) {
			std::cerr << Func_Loc << std::endl << tokens[i].loc << ": error: "
					  << "expected argument count to be '0' but found '"
					  << argCount << "'" << std::endl;
			exit(1);
		}
		i++; // eat 'num'
		tokens[i].AssertTokenType(Token::Type::Eol);
		i++; // eat 'EOL'
		std::vector<std::unique_ptr<AstNode>> params = {};
		return std::make_unique<CallAstNode>(std::move(name), std::move(id), std::move(params));
	}
	std::unique_ptr<AstNode> op1 = GetOperand(tokens[i]);
	i++; // eat 'op'
	if (tokens[i].type == Token::Type::Eol) {
		i++; // eat 'EOL'
		return std::make_unique<AssignmentAstNode>(std::move(id), std::move(op1));
	}
	if (!tokens[i].IsArith() && !tokens[i].IsRelop()) {
		tokens[i].ErrorTypeMismatch(Token_Type_ArithRelop);
	}
	std::unique_ptr<AstNode> op;
	if (tokens[i].IsArith()) {
		std::string arith = tokens[i].content;
		i++; // eat 'arith'
		std::unique_ptr<AstNode> op2 = GetOperand(tokens[i]);
		i++; // eat 'op'
		op = std::make_unique<ArithAstNode>(arith, std::move(op1), std::move(op2));
	} else {
		std::string relop = tokens[i].content;
		i++; // eat 'relop'
		std::unique_ptr<AstNode> op2 = GetOperand(tokens[i]);
		i++; // eat 'op'
		op = std::make_unique<RelopAstNode>(relop, std::move(op1), std::move(op2));	
	}
	tokens[i].AssertTokenType(Token::Type::Eol);
	i++; // eat 'EOL'
	return std::make_unique<AssignmentAstNode>(std::move(id), std::move(op));
}

static std::unique_ptr<AstNode> ParseIf(const std::vector<Token> tokens, size_t &i) {
	i++; // eat 'if'
	tokens[i].AssertTokenType(Token::Type::LParen);
	i++; // eat '('
	std::unique_ptr<AstNode> op1 = GetOperand(tokens[i]);
	i++; // eat 'op'
	tokens[i].AssertTokenTypes(Token_Type_Relop);
	std::string relop = tokens[i].content;
	i++; // eat 'relop'
	std::unique_ptr<AstNode> op2 = GetOperand(tokens[i]);
	std::unique_ptr<AstNode> op = std::make_unique<RelopAstNode>(relop, std::move(op1), std::move(op2));
	i++; // eat 'op'
	tokens[i].AssertTokenType(Token::Type::RParen);
	i++; // eat ')'
	tokens[i].AssertTokenType(Token::Type::Goto);
	i++; // eat 'goto'
	tokens[i].AssertTokenType(Token::Type::Iden);
	std::unique_ptr<AstNode> name = std::make_unique<IdenAstNode>(tokens[i].content);
	i++; // eat 'iden'
	std::unique_ptr<AstNode> gotoNode = std::make_unique<GotoAstNode>(std::move(name));
	tokens[i].AssertTokenType(Token::Type::Eol);
	i++; // eat 'EOL'
	return std::make_unique<IfAstNode>(std::move(op), std::move(gotoNode));
}

static std::unique_ptr<AstNode> ParseFunction(const std::vector<Token> tokens, size_t &i) {
	i++; // eat 'function'
	tokens[i].AssertTokenType(Token::Type::Iden);
	std::unique_ptr<AstNode> name = std::make_unique<IdenAstNode>(tokens[i].content);
	i++; // eat 'iden'
	tokens[i].AssertTokenType(Token::Type::Comma);
	i++; // eat ','
	tokens[i].AssertTokenType(Token::Type::Num);
	size_t argCount = (size_t)std::stoi(tokens[i].content);
	i++; // eat 'num'
	tokens[i].AssertTokenType(Token::Type::Eol);
	i++; // eat 'EOL'
	std::vector<std::unique_ptr<AstNode>> args;
	for (size_t j = 0; j < argCount; ++j) {
		tokens[i].AssertTokenType(Token::Type::Arg);
		i++; // eat 'arg'
		tokens[i].AssertTokenType(Token::Type::Iden);
		args.push_back(std::make_unique<IdenAstNode>(tokens[i].content));
		i++; // eat 'iden'
		tokens[i].AssertTokenType(Token::Type::Eol);
		i++; // eat 'EOL'
	}
	std::vector<std::unique_ptr<AstNode>> body = BuildAstPrec(tokens, i, Prec_Function);
	return std::make_unique<FunctionAstNode>(std::move(name), std::move(args), std::move(body));
}

static std::unique_ptr<AstNode> ParseParam(const std::vector<Token> tokens, size_t &i) {
	std::vector<std::unique_ptr<AstNode>> params;
	while (tokens[i].type == Token::Type::Param) {
		i++; // eat 'param'
		params.push_back(GetOperand(tokens[i]));
		i++; // eat 'iden'
		tokens[i].AssertTokenType(Token::Type::Eol);
		i++; // eat 'EOL'
	}
	std::unique_ptr<AstNode> id = std::make_unique<IdenAstNode>(tokens[i].content);
	i++; // eat 'iden' 
	tokens[i].AssertTokenType(Token::Type::Equal);
	i++; // eat '='
	tokens[i].AssertTokenType(Token::Type::Call);
	i++; // eat 'call'
	tokens[i].AssertTokenType(Token::Type::Iden);
	std::unique_ptr<AstNode> name = std::make_unique<IdenAstNode>(tokens[i].content);
	i++; // eat 'iden'
	tokens[i].AssertTokenType(Token::Type::Comma);
	i++; // eat ','
	tokens[i].AssertTokenType(Token::Type::Num);
	size_t argCount = (size_t)std::stoi(tokens[i].content);
	if (params.size() != argCount) {
		std::cerr << Func_Loc << std::endl << tokens[i].loc << ": error: "
				  << "expected argument count to be '" << params.size()
				  << "' but found '" << argCount << "'" << std::endl;
		exit(1);
	}
	i++; // eat 'num'
	tokens[i].AssertTokenType(Token::Type::Eol);
	i++; // eat 'EOL'
	return std::make_unique<CallAstNode>(std::move(name), std::move(id), std::move(params));
}

static std::unique_ptr<AstNode> ParseLabel(const std::vector<Token> tokens, size_t &i) {
	i++; // eat 'label'
	tokens[i].AssertTokenType(Token::Type::Iden);
	std::unique_ptr<AstNode> name = std::make_unique<IdenAstNode>(tokens[i].content);
	i++; // eat 'iden'
	tokens[i].AssertTokenType(Token::Type::Eol);
	i++; // eat 'EOL'
	std::vector<std::unique_ptr<AstNode>> body = BuildAstPrec(tokens, i, Prec_Label);
	return std::make_unique<LabelAstNode>(std::move(name), std::move(body));
}

static std::unique_ptr<AstNode> ParseGoto(const std::vector<Token> tokens, size_t &i) {
	i++; // eat 'goto'
	tokens[i].AssertTokenType(Token::Type::Iden);
	std::unique_ptr<AstNode> name = std::make_unique<IdenAstNode>(tokens[i].content);
	i++; // eat 'iden'
	tokens[i].AssertTokenType(Token::Type::Eol);
	i++; // eat 'EOL'
	return std::make_unique<GotoAstNode>(std::move(name));
}

static std::unique_ptr<AstNode> ParseRet(const std::vector<Token> tokens, size_t &i) {
	i++; // eat 'ret'
	std::unique_ptr<AstNode> op = GetOperand(tokens[i]);
	i++; // eat 'op'
	tokens[i].AssertTokenType(Token::Type::Eol);
	i++; // eat 'EOL'
	return std::make_unique<RetAstNode>(std::move(op));
} 

static std::vector<std::unique_ptr<AstNode>> BuildAstPrec(const std::vector<Token> tokens, size_t &i, int prec) {
	std::vector<std::unique_ptr<AstNode>> astNodes;
	while (tokens[i].type != Token::Type::Eof && InstructionPrecedence(tokens[i]) < prec) {
		switch (tokens[i].type) {
		case Token::Type::Function:
			astNodes.push_back(ParseFunction(tokens, i));
			break;
		case Token::Type::Param:
			astNodes.push_back(ParseParam(tokens, i));
			break;
		case Token::Type::Iden:
			astNodes.push_back(ParseIden(tokens, i));
			break;
		case Token::Type::Label:
			astNodes.push_back(ParseLabel(tokens, i));
			break;
		case Token::Type::Goto:
			astNodes.push_back(ParseGoto(tokens, i));
			break;
		case Token::Type::Ret:
			astNodes.push_back(ParseRet(tokens, i));
			break;
		case Token::Type::If:
			astNodes.push_back(ParseIf(tokens, i));
			break;
		case Token::Type::Eol:
			i++;
			break;
		default:
			std::cerr << Func_Loc << std::endl;
			tokens[i].ErrorTypeMismatch({Token::Type::Iden, Token::Type::Goto,
					Token::Type::Label, Token::Type::Ret, Token::Type::If,
					Token::Type::Param, Token::Type::Eol});
		}
	}
	return astNodes;
}

std::vector<std::unique_ptr<AstNode>> ParserBuildAst(const std::vector<Token> tokens) {
	size_t i = 0;
	return BuildAstPrec(tokens, i, Prec_Start);
}
