.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Char, "x", Expr(CharLiter('a'))), Sequence(Print(Ident("x")), Sequence(Println(StrLiter("input an integer to continue...")), Read(Ident("x"))))) }.generate(_, 4):
