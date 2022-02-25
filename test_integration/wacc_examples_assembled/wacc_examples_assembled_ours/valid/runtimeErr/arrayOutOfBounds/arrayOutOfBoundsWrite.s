.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Array(Int), "b", ArrayLiter(ArrayLiter([IntLiter(1), IntLiter(2), IntLiter(3)]))), Sequence(Declaration(Array(Int), "a", ArrayLiter(ArrayLiter([IntLiter(43), IntLiter(2), IntLiter(18), IntLiter(1)]))), Sequence(Assignment(ArrayElem(ArrayElem("a", [IntLiter(5)])), Expr(IntLiter(100))), Println(ArrayElem(ArrayElem("a", [IntLiter(5)])))))) }.generate(_, 4):
