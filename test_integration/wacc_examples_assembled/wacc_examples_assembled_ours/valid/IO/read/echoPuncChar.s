.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Char, "c", Expr(CharLiter('\u{0}'))), Sequence(Println(StrLiter("enter a character to echo")), Sequence(Read(Ident("c")), Println(Ident("c"))))) }.generate(_, 4):
