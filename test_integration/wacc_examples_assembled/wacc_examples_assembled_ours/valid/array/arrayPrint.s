.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Array(Int), "a", ArrayLiter(ArrayLiter([IntLiter(0), IntLiter(1), IntLiter(2), IntLiter(3), IntLiter(4), IntLiter(5), IntLiter(6), IntLiter(7), IntLiter(8), IntLiter(9)]))), Sequence(Declaration(Int, "i", Expr(IntLiter(0))), Sequence(Print(Ident("a")), Sequence(Print(StrLiter(" = {")), Sequence(Assignment(Ident("i"), Expr(IntLiter(0))), Sequence(While(BinaryApp(Ident("i"), Lt, IntLiter(10)), Sequence(Print(ArrayElem(ArrayElem("a", [Ident("i")]))), Sequence(If(BinaryApp(Ident("i"), Lt, IntLiter(9)), Print(StrLiter(", ")), Skip), Assignment(Ident("i"), Expr(BinaryApp(Ident("i"), Add, IntLiter(1))))))), Println(StrLiter("}")))))))) }.generate(_, 4):
