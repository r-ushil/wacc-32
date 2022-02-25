.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Char, "a", Expr(CharLiter('a'))), Sequence(Declaration(Char, "e", Expr(CharLiter('e'))), Sequence(Declaration(Char, "c", Expr(CharLiter('c'))), Sequence(Println(BinaryApp(Ident("a"), Lt, Ident("e"))), Println(BinaryApp(Ident("e"), Lt, Ident("c"))))))) }.generate(_, 4):
