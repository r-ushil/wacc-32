.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Bool, "p", Expr(BoolLiter(false))), Sequence(Declaration(Bool, "q", Expr(BoolLiter(true))), Sequence(Declaration(Bool, "r", Expr(BoolLiter(true))), Sequence(Println(BinaryApp(BinaryApp(Ident("p"), Eq, Ident("q")), Or, Ident("r"))), Println(BinaryApp(Ident("p"), Eq, BinaryApp(Ident("q"), Or, Ident("r")))))))) }.generate(_, 4):
