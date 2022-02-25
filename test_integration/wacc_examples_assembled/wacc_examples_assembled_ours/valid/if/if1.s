.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "a", Expr(IntLiter(13))), If(BinaryApp(Ident("a"), Eq, IntLiter(13)), Println(StrLiter("correct")), Println(StrLiter("incorrect")))) }.generate(_, 4):
