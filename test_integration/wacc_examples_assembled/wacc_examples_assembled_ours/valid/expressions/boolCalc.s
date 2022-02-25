.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Bool, "b1", Expr(BoolLiter(true))), Sequence(Declaration(Bool, "b2", Expr(BoolLiter(false))), Sequence(Declaration(Bool, "b3", Expr(BinaryApp(Ident("b1"), And, Ident("b2")))), Println(Ident("b3"))))) }.generate(_, 4):
