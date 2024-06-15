#ifndef AST_HPP_
#define AST_HPP_

#include <memory>
#include <vector>
#include <string>
#include <iostream>

typedef struct AstNode {
public:
	virtual ~AstNode() = default;
	friend std::ostream& operator<<(std::ostream &os, const AstNode &node);
	virtual void Print(std::ostream &os, size_t indent) const = 0;
protected:
    void PrintIndent(std::ostream &os, size_t indent) const;
} AstNode;

typedef struct IdenAstNode: public AstNode {
public:
	IdenAstNode(const std::string &name): name(name) {}
	void Print(std::ostream &os, size_t indent) const override;
private:
	std::string name; 
} IdenAstNode;

typedef struct NumAstNode: public AstNode {
public:
	NumAstNode(double val): val(val) {}
	void Print(std::ostream &os, size_t indent) const override;
private:
	double val; 
} NumAstNode;

typedef struct CallAstNode: public AstNode {
public:
	CallAstNode(std::unique_ptr<AstNode> name, std::unique_ptr<AstNode> id,
				std::vector<std::unique_ptr<AstNode>> params)
		: name(std::move(name)), id(std::move(id)), params(std::move(params)) {}
	void Print(std::ostream &os, size_t indent) const override;
private:
	std::unique_ptr<AstNode> name, id;
	std::vector<std::unique_ptr<AstNode>> params;
} CallAstNode;

typedef struct ArithAstNode: public AstNode {
public:
	ArithAstNode(const std::string &op, std::unique_ptr<AstNode> lhs,
				  std::unique_ptr<AstNode> rhs)
		: op(op), lhs(std::move(lhs)), rhs(std::move(rhs)) {}
	void Print(std::ostream &os, size_t indent) const override;
private:
	std::string op;
	std::unique_ptr<AstNode> lhs, rhs;
} ArithAstNode;

typedef struct RelopAstNode: public AstNode {
public:
	RelopAstNode(const std::string &op, std::unique_ptr<AstNode> lhs,
				  std::unique_ptr<AstNode> rhs)
		: op(op), lhs(std::move(lhs)), rhs(std::move(rhs)) {}
	void Print(std::ostream &os, size_t indent) const override;
private:
	std::string op;
	std::unique_ptr<AstNode> lhs, rhs;
} RelopAstNode;

typedef struct UnaryAstNode: public AstNode {
public:
	UnaryAstNode(const std::string &op, std::unique_ptr<AstNode> iden)
		: op(op), iden(std::move(iden)) {}
	void Print(std::ostream &os, size_t indent) const override;
private:
	std::string op;
	std::unique_ptr<AstNode> iden;
} UnaryAstNode;

typedef struct FunctionAstNode: public AstNode {
public:
	FunctionAstNode(std::unique_ptr<AstNode> name,
				std::vector<std::unique_ptr<AstNode>> args,
				std::vector<std::unique_ptr<AstNode>> body)
		: name(std::move(name)), args(std::move(args)), body(std::move(body)) {}
	void Print(std::ostream &os, size_t indent) const override;
private:
	std::unique_ptr<AstNode> name;
	std::vector<std::unique_ptr<AstNode>> args;
	std::vector<std::unique_ptr<AstNode>> body;
} FunctionAstNode;

typedef struct AssignmentAstNode: public AstNode {
public:
	AssignmentAstNode(std::unique_ptr<AstNode> id,
					  std::unique_ptr<AstNode> op)
		: id(std::move(id)), op(std::move(op)) {}
	void Print(std::ostream &os, size_t indent) const override;
private:
	std::unique_ptr<AstNode> id, op;
} AssignmentAstNode;

typedef struct GotoAstNode: public AstNode {
public:
	GotoAstNode(std::unique_ptr<AstNode> name): name(std::move(name)) {}
	void Print(std::ostream &os, size_t indent) const override;
private:
	std::unique_ptr<AstNode> name;
} GotoAstNode;

typedef struct LabelAstNode: public AstNode {
public:
	LabelAstNode(std::unique_ptr<AstNode> name, std::vector<std::unique_ptr<AstNode>> body)
		: name(std::move(name)), body(std::move(body)) {}
	void Print(std::ostream &os, size_t indent) const override;
private:
	std::unique_ptr<AstNode> name;
	std::vector<std::unique_ptr<AstNode>> body;
} LabelAstNode;

typedef struct IfAstNode: public AstNode {
public:
	IfAstNode(std::unique_ptr<AstNode> condition,
			  std::unique_ptr<AstNode> label)
		: condition(std::move(condition)), label(std::move(label)) {}
	void Print(std::ostream &os, size_t indent) const override;
private:
	std::unique_ptr<AstNode> condition;
	std::unique_ptr<AstNode> label;
} IfAstNode;

typedef struct RetAstNode: public AstNode {
public:
	RetAstNode(std::unique_ptr<AstNode> op) : op(std::move(op)) {}
	void Print(std::ostream &os, size_t indent) const override;
private:
	std::unique_ptr<AstNode> op;
} RetAstNode;

#endif // AST_HPP_
