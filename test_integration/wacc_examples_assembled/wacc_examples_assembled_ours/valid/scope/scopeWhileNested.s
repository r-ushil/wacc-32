.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "x", Expr(IntLiter(5))), Sequence(While(BinaryApp(Ident("x"), Gt, IntLiter(0)), Sequence(Scope(Sequence(Declaration(String, "x", Expr(StrLiter("counting... "))), Print(Ident("x")))), Sequence(Println(Ident("x")), Assignment(Ident("x"), Expr(BinaryApp(Ident("x"), Sub, IntLiter(1))))))), Sequence(Print(Ident("x")), Println(StrLiter(" Boom!"))))) }.generate(_, 4):
