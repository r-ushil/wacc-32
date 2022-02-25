.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "x", Expr(IntLiter(5))), Sequence(Declaration(String, "y", Expr(StrLiter(" Boom!"))), Sequence(While(BinaryApp(Ident("x"), Gt, IntLiter(0)), Sequence(Declaration(String, "y", Expr(StrLiter("counting... "))), Sequence(Print(Ident("y")), Sequence(Println(Ident("x")), Assignment(Ident("x"), Expr(BinaryApp(Ident("x"), Sub, IntLiter(1)))))))), Sequence(Print(Ident("x")), Println(Ident("y")))))) }.generate(_, 4):
