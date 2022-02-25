.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "i", Expr(IntLiter(0))), Sequence(Declaration(Int, "f0", Expr(IntLiter(0))), Sequence(Declaration(Int, "f1", Expr(IntLiter(1))), Sequence(Declaration(Int, "save", Expr(IntLiter(0))), Sequence(Println(StrLiter("The first 20 fibonacci numbers are:")), Sequence(While(BinaryApp(Ident("i"), Lt, IntLiter(20)), Sequence(Print(Ident("f0")), Sequence(Print(StrLiter(", ")), Sequence(Assignment(Ident("save"), Expr(Ident("f0"))), Sequence(Assignment(Ident("f0"), Expr(Ident("f1"))), Sequence(Assignment(Ident("f1"), Expr(BinaryApp(Ident("save"), Add, Ident("f1")))), Assignment(Ident("i"), Expr(BinaryApp(Ident("i"), Add, IntLiter(1)))))))))), Println(StrLiter("...")))))))) }.generate(_, 4):
