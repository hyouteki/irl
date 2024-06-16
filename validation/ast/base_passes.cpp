#include "base_passes.hpp"

void LabelValidationPass::RunOnFunction(FunctionAstNode &function) const {}

void AssignmentValidationPass::RunOnFunction(FunctionAstNode &function) const {
	std::unordered_set<std::string> ids;
	// for (const auto &node: function.args) {
	// 	if (auto argNode = dynamic_cast<IdenAstNode *>(node.get())) {
	// 		ids.insert(*argNode);
	// 	}
	// }
}
