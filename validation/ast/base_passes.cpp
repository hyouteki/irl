#include "base_passes.hpp"

void LabelValidationPass::RunOnFunction(FunctionAstNode &function) const {}

void IdentifierValidationPass::RunOnFunction(FunctionAstNode &function) const {
	std::unordered_set<std::string> ids;
	for (const auto &node: function.args) {
		if (auto argNode = dynamic_cast<IdenAstNode *>(node.get())) {
			ids.insert(argNode->name);
		}
	}
	for (const auto &node: function.body) {
		if (auto derivedNode = dynamic_cast<ArithAstNode *>(node.get())) {
			if (ids.find(dynamic_cast<IdenAstNode *>(derivedNode->lhs.get())->name)
				== ids.end()) {
				std::cerr << "lhs not present" << std::endl;
			}
		}
	}
}
