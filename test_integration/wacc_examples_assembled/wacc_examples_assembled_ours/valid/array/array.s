.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Array(Int), "a", ArrayLiter(ArrayLiter([IntLiter(0), IntLiter(0), IntLiter(0), IntLiter(0), IntLiter(0), IntLiter(0), IntLiter(0), IntLiter(0), IntLiter(0), IntLiter(0)]))), Sequence(Declaration(Int, "i", Expr(IntLiter(0))), Sequence(While(BinaryApp(Ident("i"), Lt, IntLiter(10)), Sequence(Assignment(ArrayElem(ArrayElem("a", [Ident("i")])), Expr(Ident("i"))), Assignment(Ident("i"), Expr(BinaryApp(Ident("i"), Add, IntLiter(1)))))), Sequence(Print(Ident("a")), Sequence(Print(StrLiter(" = {")), Sequence(Assignment(Ident("i"), Expr(IntLiter(0))), Sequence(While(BinaryApp(Ident("i"), Lt, IntLiter(10)), Sequence(Print(ArrayElem(ArrayElem("a", [Ident("i")]))), Sequence(If(BinaryApp(Ident("i"), Lt, IntLiter(9)), Print(StrLiter(", ")), Skip), Assignment(Ident("i"), Expr(BinaryApp(Ident("i"), Add, IntLiter(1))))))), Println(StrLiter("}"))))))))) }.generate(_, 4):
