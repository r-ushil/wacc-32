.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "x", Expr(IntLiter(-20000))), Sequence(Println(Ident("x")), Sequence(Assignment(Ident("x"), Expr(BinaryApp(Ident("x"), Mul, IntLiter(100000000)))), Println(Ident("x"))))) }.generate(_, 4):
