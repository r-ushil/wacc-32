.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "x", Expr(IntLiter(-2147483647))), Sequence(Println(Ident("x")), Sequence(Assignment(Ident("x"), Expr(BinaryApp(Ident("x"), Sub, IntLiter(1)))), Sequence(Println(Ident("x")), Sequence(Assignment(Ident("x"), Expr(BinaryApp(Ident("x"), Sub, IntLiter(1)))), Println(Ident("x"))))))) }.generate(_, 4):
