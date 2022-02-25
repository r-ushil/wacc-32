.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Char, "a", Expr(CharLiter('a'))), Sequence(Declaration(Int, "i", Expr(IntLiter(99))), Sequence(Print(Ident("a")), Sequence(Print(StrLiter(" is ")), Sequence(Println(UnaryApp(Ord, Ident("a"))), Sequence(Print(Ident("i")), Sequence(Print(StrLiter(" is ")), Println(UnaryApp(Chr, Ident("i")))))))))) }.generate(_, 4):
