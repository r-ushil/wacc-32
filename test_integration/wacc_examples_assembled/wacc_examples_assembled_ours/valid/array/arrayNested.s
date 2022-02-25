.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Array(Int), "a", ArrayLiter(ArrayLiter([IntLiter(1), IntLiter(2), IntLiter(3)]))), Sequence(Declaration(Array(Int), "b", ArrayLiter(ArrayLiter([IntLiter(3), IntLiter(4)]))), Sequence(Declaration(Array(Array(Int)), "c", ArrayLiter(ArrayLiter([Ident("a"), Ident("b")]))), Sequence(Println(ArrayElem(ArrayElem("c", [IntLiter(0), IntLiter(2)]))), Println(ArrayElem(ArrayElem("c", [IntLiter(1), IntLiter(0)]))))))) }.generate(_, 4):
