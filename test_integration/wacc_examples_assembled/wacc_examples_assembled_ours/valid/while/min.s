.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "i", Expr(IntLiter(0))), Sequence(Declaration(Int, "x", Expr(IntLiter(10))), Sequence(Declaration(Int, "y", Expr(IntLiter(17))), Sequence(While(BinaryApp(BinaryApp(Ident("y"), Gt, IntLiter(0)), And, BinaryApp(Ident("x"), Gt, IntLiter(0))), Sequence(Assignment(Ident("x"), Expr(BinaryApp(Ident("x"), Sub, IntLiter(1)))), Sequence(Assignment(Ident("y"), Expr(BinaryApp(Ident("y"), Sub, IntLiter(1)))), Assignment(Ident("i"), Expr(BinaryApp(Ident("i"), Add, IntLiter(1))))))), Sequence(Print(StrLiter("min value = ")), Println(Ident("i"))))))) }.generate(_, 4):
