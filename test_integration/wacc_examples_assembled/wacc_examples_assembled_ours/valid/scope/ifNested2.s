.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "a", Expr(IntLiter(13))), If(BinaryApp(Ident("a"), Eq, IntLiter(13)), If(BinaryApp(Ident("a"), Gt, IntLiter(5)), If(BinaryApp(Ident("a"), Lt, IntLiter(10)), Println(StrLiter("incorrect")), If(BinaryApp(Ident("a"), Gt, IntLiter(12)), If(BinaryApp(Ident("a"), Gt, IntLiter(13)), Println(StrLiter("incorrect")), Println(StrLiter("correct"))), Println(StrLiter("incorrect")))), Println(StrLiter("incorrect"))), Println(StrLiter("incorrect")))) }.generate(_, 4):
