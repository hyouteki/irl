#ifndef VALIDATION_AST_BASE_HPP_
#define VALIDATION_AST_BASE_HPP_

#include "../../fe/export.hpp"

template<typename Derived>
class ValidationPass {
public:
	ValidationPass(std::vector<std::unique_ptr<AstNode>> astNodes) {
		for (const auto &node: astNodes) {
			if (auto functionNode = dynamic_cast<FunctionAstNode *>(node.get())) {
				static_cast<Derived *>(this)->RunOnFunction(*functionNode);
			}
		}
	}
	virtual ~ValidationPass() = default;
	virtual void RunOnFunction(std::unique_ptr<FunctionAstNode> &function) const;
};

#endif // VALIDATION_AST_BASE_HPP_
