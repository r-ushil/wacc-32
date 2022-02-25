.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Bool, "p", Expr(BoolLiter(true))), Sequence(Declaration(Bool, "q", Expr(BoolLiter(true))), Sequence(Declaration(Bool, "r", Expr(BoolLiter(false))), Sequence(Declaration(Bool, "s", Expr(BoolLiter(true))), Sequence(Println(BinaryApp(BinaryApp(Ident("p"), And, BinaryApp(Ident("q"), Neq, Ident("r"))), Or, Ident("s"))), Println(BinaryApp(BinaryApp(Ident("p"), And, Ident("q")), Neq, BinaryApp(Ident("r"), Or, Ident("s"))))))))) }.generate(_, 4):
