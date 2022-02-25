.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "a", Expr(IntLiter(13))), Sequence(If(BinaryApp(Ident("a"), Eq, IntLiter(13)), Assignment(Ident("a"), Expr(IntLiter(1))), Assignment(Ident("a"), Expr(IntLiter(0)))), Println(Ident("a")))) }.generate(_, 4):
