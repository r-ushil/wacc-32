.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "x", Expr(IntLiter(2147483))), Sequence(Println(Ident("x")), Sequence(Assignment(Ident("x"), Expr(BinaryApp(Ident("x"), Mul, IntLiter(1000)))), Sequence(Println(Ident("x")), Sequence(Assignment(Ident("x"), Expr(BinaryApp(Ident("x"), Mul, IntLiter(1000)))), Sequence(Println(Ident("x")), Sequence(Assignment(Ident("x"), Expr(BinaryApp(Ident("x"), Mul, IntLiter(1000)))), Println(Ident("x"))))))))) }.generate(_, 4):
