.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Array(Char), "str", ArrayLiter(ArrayLiter([CharLiter('h'), CharLiter('e'), CharLiter('l'), CharLiter('l'), CharLiter('o'), CharLiter(' '), CharLiter('w'), CharLiter('o'), CharLiter('r'), CharLiter('l'), CharLiter('d'), CharLiter('!')]))), Sequence(Println(Ident("str")), Sequence(Assignment(ArrayElem(ArrayElem("str", [IntLiter(0)])), Expr(CharLiter('H'))), Sequence(Println(Ident("str")), Sequence(Assignment(Ident("str"), ArrayLiter(ArrayLiter([CharLiter('H'), CharLiter('i'), CharLiter('!')]))), Println(Ident("str"))))))) }.generate(_, 4):
