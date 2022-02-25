.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "a", Expr(IntLiter(13))), If(BinaryApp(Ident("a"), Eq, IntLiter(13)), If(BinaryApp(Ident("a"), Gt, IntLiter(5)), Println(StrLiter("correct")), Println(StrLiter("incorrect"))), Println(StrLiter("incorrect")))) }.generate(_, 4):
