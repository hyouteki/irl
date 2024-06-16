#ifndef VALIDATION_AST_BASE_PASSES_HPP_
#define VALIDATION_AST_BASE_PASSES_HPP_

#include <unordered_set>
#include "base.hpp"

class LabelValidationPass: public ValidationPass<LabelValidationPass> {
public:
    void RunOnFunction(FunctionAstNode& function) const;
};

class IdentifierValidationPass: public ValidationPass<IdentifierValidationPass> {
public:
    void RunOnFunction(FunctionAstNode& function) const;
};

#endif // VALIDATION_AST_BASE_PASSES_HPP_
