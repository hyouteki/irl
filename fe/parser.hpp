#ifndef PARSER_HPP_
#define PARSER_HPP_

#include "ast.hpp"
#include "lexer.hpp"

std::vector<std::unique_ptr<AstNode>> ParserBuildAst(const std::vector<Token> tokens);

#endif // PARSER_HPP_
