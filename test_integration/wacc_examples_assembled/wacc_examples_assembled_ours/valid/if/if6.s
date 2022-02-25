.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Char, "c1", Expr(CharLiter('f'))), Sequence(Declaration(Char, "c2", Expr(CharLiter('F'))), If(BinaryApp(Ident("c1"), Eq, Ident("c2")), Println(StrLiter("incorrect")), Println(StrLiter("correct"))))) }.generate(_, 4):
