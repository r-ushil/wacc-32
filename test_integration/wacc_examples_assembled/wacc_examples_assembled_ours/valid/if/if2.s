.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "a", Expr(IntLiter(13))), If(BinaryApp(Ident("a"), Neq, IntLiter(13)), Println(StrLiter("incorrect")), Println(StrLiter("correct")))) }.generate(_, 4):
