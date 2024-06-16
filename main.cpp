#include "fe/export.hpp"
#include "validation/export.hpp"

int main(int argc, char* argv[]) {
	std::string filename = std::string(argv[1]);
	Lexer lexer = Lexer(filename);
	std::cout << "=== Tokens ===" << std::endl;
	std::cout << lexer << std::endl;
	std::vector<std::unique_ptr<AstNode>> astNodes =
		ParserBuildAst(lexer.tokens);
	std::cout << "==== AST =====" << std::endl;
	for (const auto &astNode: astNodes) {
		std::cout << *astNode;
	}
	return 0;
}
