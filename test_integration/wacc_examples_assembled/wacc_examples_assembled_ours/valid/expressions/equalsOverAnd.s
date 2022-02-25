.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Bool, "p", Expr(BoolLiter(false))), Sequence(Declaration(Bool, "q", Expr(BoolLiter(true))), Sequence(Declaration(Bool, "r", Expr(BoolLiter(false))), Sequence(Println(BinaryApp(BinaryApp(Ident("p"), Eq, Ident("q")), And, Ident("r"))), Println(BinaryApp(Ident("p"), Eq, BinaryApp(Ident("q"), And, Ident("r")))))))) }.generate(_, 4):
