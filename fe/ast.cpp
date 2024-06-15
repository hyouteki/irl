#include "ast.hpp"

void AstNode::PrintIndent(std::ostream &os, size_t indent) const {
	for (size_t i = 0; i < indent; ++i) os << "    ";
}


std::ostream& operator<<(std::ostream &os, const AstNode &node) {
	node.Print(os, 0);
	return os;
}

void IdenAstNode::Print(std::ostream &os, size_t _) const {
	os << this->name;
}

void NumAstNode::Print(std::ostream &os, size_t _) const {
	os << this->val;
}

void CallAstNode::Print(std::ostream &os, size_t indent) const {
	for (const auto &param : this->params) {
		this->PrintIndent(os, indent);
        os << "param " << *param << std::endl;
    }
	this->PrintIndent(os, indent);
	os << *this->id << " = call " << *this->name << ", "
	   << this->params.size() << std::endl;
}

void ArithAstNode::Print(std::ostream &os, size_t _) const {
	os << *this->lhs << " " << this->op << " " << *this->rhs;
}

void RelopAstNode::Print(std::ostream &os, size_t _) const {
	os << *this->lhs << " " << this->op << " " << *this->rhs;
}

void UnaryAstNode::Print(std::ostream &os, size_t _) const {
	os << this->op << " " << *this->iden;
}

void FunctionAstNode::Print(std::ostream &os, size_t indent) const {
	this->PrintIndent(os, indent);
	os << "function " << *this->name << ", " << this->args.size() << std::endl;
	for (const auto &arg: this->args) {
		os << "arg " << *arg << std::endl;
	}
	for (const auto &inst: this->body) {
		inst->Print(os, indent+1);
		os << std::endl;
	}
}

void AssignmentAstNode::Print(std::ostream &os, size_t indent) const {
	this->PrintIndent(os, indent);
	os << *this->id << " = " << *this->op << std::endl;
}

void GotoAstNode::Print(std::ostream &os, size_t indent) const {
	this->PrintIndent(os, indent);
	os << "goto " << *this->name << std::endl;
}

void LabelAstNode::Print(std::ostream &os, size_t indent) const {
	this->PrintIndent(os, indent);
	os << "label " << *this->name << std::endl;
	for (const auto &inst: this->body) {
		inst->Print(os, indent+1);
	}
}

void IfAstNode::Print(std::ostream &os, size_t indent) const {
	this->PrintIndent(os, indent);
	os << "if (" << *this->condition << ") " << *this->label;
}

void RetAstNode::Print(std::ostream &os, size_t indent) const {
	this->PrintIndent(os, indent);
	os << "ret " << *this->op << std::endl;
}
