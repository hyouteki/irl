## Intermediate Representation Language
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

### IRL Architecture
![IRL architecture](./assets/irl-architecture.jpg)

### Quick Start
``` bash
cargo run
```
