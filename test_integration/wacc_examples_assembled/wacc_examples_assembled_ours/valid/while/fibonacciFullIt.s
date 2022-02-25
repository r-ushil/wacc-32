.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Println(StrLiter("This program calculates the nth fibonacci number iteratively.")), Sequence(Print(StrLiter("Please enter n (should not be too large): ")), Sequence(Declaration(Int, "n", Expr(IntLiter(0))), Sequence(Read(Ident("n")), Sequence(Print(StrLiter("The input n is ")), Sequence(Println(Ident("n")), Sequence(Print(StrLiter("The nth fibonacci number is ")), Sequence(Declaration(Int, "f0", Expr(IntLiter(0))), Sequence(Declaration(Int, "f1", Expr(IntLiter(1))), Sequence(Declaration(Int, "save", Expr(IntLiter(0))), Sequence(While(BinaryApp(Ident("n"), Gt, IntLiter(0)), Sequence(Assignment(Ident("save"), Expr(Ident("f0"))), Sequence(Assignment(Ident("f0"), Expr(Ident("f1"))), Sequence(Assignment(Ident("f1"), Expr(BinaryApp(Ident("save"), Add, Ident("f1")))), Assignment(Ident("n"), Expr(BinaryApp(Ident("n"), Sub, IntLiter(1)))))))), Println(Ident("f0"))))))))))))) }.generate(_, 4):
