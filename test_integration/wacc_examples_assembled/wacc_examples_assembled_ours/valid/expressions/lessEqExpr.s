.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "x", Expr(IntLiter(2))), Sequence(Declaration(Int, "y", Expr(IntLiter(6))), Sequence(Declaration(Int, "z", Expr(IntLiter(4))), Sequence(Declaration(Int, "a", Expr(IntLiter(4))), Sequence(Println(BinaryApp(Ident("x"), Lte, Ident("y"))), Sequence(Println(BinaryApp(Ident("y"), Lte, Ident("z"))), Println(BinaryApp(Ident("z"), Lte, Ident("a"))))))))) }.generate(_, 4):
