#include "lexer.hpp"

static int String_StripFront(std::string &str) {
    size_t start = 0;
    while (start < str.size() && std::isspace(static_cast<unsigned char>(str[start]))) {
        ++start;
    }
	str = str.substr(start);
	return start;
}

static bool String_StartsWith(const std::string &str, const std::string &prefix) {
	if (str.size() < prefix.size()) return false;
	for (size_t i = 0; i < prefix.size(); ++i) {
		if (str[i] != prefix[i]) return false;
	}
	return true;
}

static std::vector<std::pair<std::string, Token::Type>> operatorTable = {
	{"==" , Token::Type::Eq},
	{"="  , Token::Type::Equal},
	{"+"  , Token::Type::Plus},
	{"-"  , Token::Type::Minus},
	{"*"  , Token::Type::Mul},
	{"/"  , Token::Type::Div},
	{"!=" , Token::Type::Neq},
	{">=" , Token::Type::Ge},
	{"<=" , Token::Type::Le},
	{">"  , Token::Type::Gt},
	{"<"  , Token::Type::Lt},
	{","  , Token::Type::Comma},
	{"("  , Token::Type::LParen},
	{")"  , Token::Type::RParen},
};

static std::vector<std::pair<std::string, Token::Type>> keywordTable = {
	{"function" , Token::Type::Function},
	{"arg"		, Token::Type::Arg},
	{"goto"		, Token::Type::Goto},
	{"label"	, Token::Type::Label},
	{"if"		, Token::Type::If},
	{"param"	, Token::Type::Param},
	{"call"		, Token::Type::Call},
	{"ret"		, Token::Type::Ret},
};

std::ostream &operator<<(std::ostream &os, const Loc &loc) {
	os << loc.filename << ":" << loc.row << ":" << loc.col;
	return os;
}

std::string Token::TypeToString() const {
	switch (this->type) {
	case Token::Type::Function: return "Function";
	case Token::Type::Iden: return "Iden";
	case Token::Type::Num: return "Num";
	case Token::Type::Comma: return "Comma";
	case Token::Type::Arg: return "Arg";
	case Token::Type::Equal: return "Equal";
	case Token::Type::Plus: return "Plus";
	case Token::Type::Minus: return "Minus";
	case Token::Type::Mul: return "Mul";
	case Token::Type::Div: return "Div";
	case Token::Type::Goto: return "Goto";
	case Token::Type::Label: return "Label";
	case Token::Type::If: return "If";
	case Token::Type::LParen: return "LParen";
	case Token::Type::RParen: return "RParen";
	case Token::Type::Eq: return "Eq";
	case Token::Type::Neq: return "Neq";
	case Token::Type::Gt: return "Gt";
	case Token::Type::Lt: return "Lt";
	case Token::Type::Ge: return "Ge";
	case Token::Type::Le: return "Le";
	case Token::Type::Param: return "Param";
	case Token::Type::Call: return "Call";
	case Token::Type::Ret: return "Ret";
	case Token::Type::Eol: return "Eol";
	case Token::Type::Eof: return "Eof";
	default:
        std::cerr << Func_Loc << std::endl << this->loc << ": error: "
				  << "encountered invalid token type" << std::endl;
		exit(1);
	}
}

void Token::ErrorTypeMismatch(const std::vector<Token::Type> types) const {
    switch (types.size()) {
	case 0:
		std::cerr << this->loc << ": error: expected no token; but got token of type '"
				  << this->TypeToString() << "'" << std::endl;
		break;
	case 1:
		std::cerr << this->loc << ": error: expected token of type '"
				  << (Token){.type=types[0]}.TypeToString() << "'; but got '"
				  << this->TypeToString() << "'" << std::endl;
		break;
	default:
		std::cerr << this->loc << ": error: expected token of types '";
		for (size_t i = 0; i < types.size(); ++i) {
			std::cerr << (Token){.type=types[i]}.TypeToString();
			if (i < types.size()-1) std::cerr << " | "; 
		}
		std::cerr << "'; but got '" << this->TypeToString() << "'" << std::endl;
	}
	exit(1);
} 

void Token::AssertTokenType(const Type type) const {
	if (this->type == type) return;
	this->ErrorTypeMismatch({type});
}

bool Token::IsArith() const {
	switch (this->type) {
	case Token::Type::Plus: return true;
	case Token::Type::Minus: return true;
	case Token::Type::Mul: return true;
	case Token::Type::Div: return true;
	default: return false;
	}
}

bool Token::IsUnary() const {
	switch (this->type) {
	case Token::Type::Minus: return true;
	default: return false;
	} 
}

bool Token::IsRelop() const {
	switch (this->type) {
	case Token::Type::Eq: return true;
	case Token::Type::Neq: return true;
	case Token::Type::Gt: return true;
	case Token::Type::Lt: return true;
	case Token::Type::Le: return true;
	case Token::Type::Ge: return true;
	default: return false;
	} 
}

Lexer::Lexer(std::string &filename) {
	std::ifstream file(filename);
    if (file.fail()) {
        std::cerr << Func_Loc << std::endl << "Error: Filename '"
				  << filename << "' does not exist" << std::endl;
        exit(1);
    }
    std::string line;
    while (getline(file, line)) this->content.push_back(line);
    file.close();

    if (this->content.empty()) return;
	size_t row, col;
    for (size_t i = 0; i < this->content.size(); ++i) {
        row = i+1, col = 1;
		std::string line(this->content[i]);
		
        while (!line.empty()) {
            col += String_StripFront(line);
            if (line.empty()) continue;
			
            bool flag = false;
            for (std::pair<std::string, Token::Type> op: operatorTable) {
                if (String_StartsWith(line, op.first)) {
                    this->tokens.push_back((Token){
							.type = op.second,
							.content = op.first,
							.loc  = (Loc){.row = row, .col = col, .filename=filename}
						});
                    line = line.substr(op.first.size());
                    col += op.first.size();
                    flag = true;
                    break;
                }
            }
            if (flag) continue;

			if (std::isalpha(static_cast<unsigned char>(line[0]))) {
				size_t j = 1;
				while (j < line.size() && std::isalnum(static_cast<unsigned char>(line[j]))) {
					++j;
				}
				std::string iden = line.substr(0, j);
				Token::Type type = Token::Type::Iden;
				for (std::pair<std::string, Token::Type> op: keywordTable) {
					if (iden.compare(op.first) == 0) {
						type = op.second;
						break;
					}
				}
				this->tokens.push_back((Token){
						.type = type,
						.content = iden,
						.loc  = (Loc){.row = row, .col = col, .filename=filename}
					});
				line = line.substr(iden.size());
				col += iden.size();
				continue;
			}
			
			if (std::isdigit(static_cast<unsigned char>(line[0]))) {
				size_t j = 1;
				while (j < line.size() && std::isdigit(static_cast<unsigned char>(line[j]))) {
					++j;
				}
				std::string iden = line.substr(0, j);
				this->tokens.push_back((Token){
						.type = Token::Type::Num,
						.content = iden,
						.loc  = (Loc){.row = row, .col = col, .filename=filename}
					});
				line = line.substr(iden.size());
				col += iden.size();
				continue;
			}

			Loc loc = (Loc){.row=row, .col=col, .filename=filename};
            std::cerr << Func_Loc << std::endl << loc << ": error: unknown token '"
					  << line[0] << "'" << std::endl;
            exit(1);
        }

		this->tokens.push_back((Token){
				.type = Token::Type::Eol,
				.content = "Eol",
				.loc  = (Loc){.row = row, .col = col, .filename=filename}
			});
    }
	
	this->tokens.push_back((Token){
			.type = Token::Type::Eof,
			.content = "Eof",
			.loc  = (Loc){.row = row, .col = col, .filename=filename}
		});
}

std::ostream &operator<<(std::ostream &os, const Lexer &lexer) {
	for (Token token: lexer.tokens) {
		os << token.TypeToString() << " \t " << token.content << std::endl;
	}
	return os;
}
