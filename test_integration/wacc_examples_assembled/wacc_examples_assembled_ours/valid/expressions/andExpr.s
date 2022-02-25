.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Bool, "a", Expr(BoolLiter(true))), Sequence(Declaration(Bool, "b", Expr(BoolLiter(false))), Sequence(Println(BinaryApp(Ident("a"), And, Ident("b"))), Sequence(Println(BinaryApp(Ident("a"), And, BoolLiter(true))), Println(BinaryApp(Ident("b"), And, BoolLiter(false))))))) }.generate(_, 4):
