#ifndef LEXER_HPP_
#define LEXER_HPP_

#include <vector>
#include <iostream>
#include <unistd.h>
#include <fstream>
#include "../helper.hpp"

typedef struct Loc {
	size_t row;
	size_t col;
	std::string filename;
public:
	friend std::ostream &operator<<(std::ostream &os, const Loc &loc);
} Loc;

typedef struct Token {	
	enum class Type {
		Function,
		Iden,
		Num,
		Comma, 
		Arg,
		Equal,
		Plus,
		Minus,
		Mul,
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
	};
	Type type;
	std::string content;
	Loc loc;
public:
	std::string TypeToString() const;	
	void ErrorTypeMismatch(const std::vector<Type> types) const;
	void AssertTokenType(const Type type) const;
	bool IsArith() const;
	bool IsUnary() const;
	bool IsRelop() const;
} Token;

typedef struct Lexer {
public:
	std::vector<Token> tokens;
	Lexer(std::string &filename);
	friend std::ostream &operator<<(std::ostream &os, const Lexer &lexer);
private:
	std::vector<std::string> content;
} Lexer;

#define Token_Type_ArithRelop { Token::Type::Plus,				\
			Token::Type::Minus, Token::Type::Mul,				\
			Token::Type::Div, Token::Type::Eq,					\
			Token::Type::Neq, Token::Type::Gt,					\
			Token::Type::Lt, Token::Type::Ge, Token::Type::Le}

#endif // LEXER_HPP_
