## Intermedite Representation Language
> Planned to be a simpler version of what LLVM is at its core; "An optimizer and transpiler of its very own LLVM IR to various architecture's ISA".

### Grammar

``` asm
function L, n
arg id
id = op
id = op1 arith op2
id = unary op
goto L
label L
if (op1 relop op2) goto L
id = op1 relop op2
param op
id = call L, n
ret op
```
Below are the tokenized syntax mentioned above and these are the only valid token combination to exist.
``` f90
FUNCTION IDEN COMMA NUM EOL
ARG IDEN EOL
IDEN EQUAL IDEN* EOL
IDEN EQUAL IDEN* ARITH IDEN* EOL
IDEN EQUAL UNARY IDEN* EOL
GOTO IDEN EOL
LABEL IDEN EOL
IF LPAREN IDEN* RELOP IDEN* RPAREN GOTO IDEN EOL
IDEN EQUAL IDEN* RELOP IDEN* EOL
PARAM IDEN EOL
IDEN EQUAL CALL IDEN COMMA NUM EOL
RET IDEN* EOL EOF
```
| **Category** | **Symbols**                    |
|--------------|--------------------------------|
| NUM          | (0-9)+                         |
| IDEN         | (a-zA-Z)(a-zA-Z0-9)*           |
| IDEN\*       | IDEN \| NUM                    |
| ARITH        | + \| - \| * \| /               |
| UNARY        | -                              |
| RELOP        | == \| != \| <= \| < \| > \| >= |

### Quick Start
``` bash
chmod +x build.sh
./build.sh
```
