.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Print(StrLiter("Printing an array variable gives an address, such as ")), Sequence(Declaration(Array(Int), "a", ArrayLiter(ArrayLiter([IntLiter(1), IntLiter(2), IntLiter(3)]))), Println(Ident("a")))) }.generate(_, 4):
