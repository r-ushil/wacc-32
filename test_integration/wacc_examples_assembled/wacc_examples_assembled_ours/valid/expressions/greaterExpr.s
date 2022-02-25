.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "x", Expr(IntLiter(2))), Sequence(Declaration(Int, "y", Expr(IntLiter(6))), Sequence(Declaration(Int, "z", Expr(IntLiter(4))), Sequence(Println(BinaryApp(Ident("x"), Gt, Ident("y"))), Println(BinaryApp(Ident("y"), Gt, Ident("z"))))))) }.generate(_, 4):
