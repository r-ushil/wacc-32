.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "x", Expr(IntLiter(-2147483648))), Sequence(Println(Ident("x")), Sequence(Assignment(Ident("x"), Expr(UnaryApp(Neg, Ident("x")))), Println(Ident("x"))))) }.generate(_, 4):
