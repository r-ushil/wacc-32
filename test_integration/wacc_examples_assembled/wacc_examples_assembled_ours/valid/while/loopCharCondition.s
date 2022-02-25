.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Char, "c", Expr(CharLiter('\u{0}'))), Sequence(While(BinaryApp(Ident("c"), Eq, CharLiter('\u{0}')), Sequence(Assignment(Ident("c"), Expr(CharLiter('a'))), Println(StrLiter("Change c")))), Println(StrLiter("Should print \"Change c\" once before.")))) }.generate(_, 4):
