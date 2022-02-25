.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "x", Expr(IntLiter(2000000000))), Sequence(Println(Ident("x")), Sequence(Assignment(Ident("x"), Expr(BinaryApp(Ident("x"), Add, IntLiter(2000000000)))), Println(Ident("x"))))) }.generate(_, 4):
