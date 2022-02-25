.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "a", Expr(BinaryApp(BinaryApp(IntLiter(10), Mul, IntLiter(1)), Add, BinaryApp(IntLiter(2), Mul, IntLiter(15))))), If(BinaryApp(Ident("a"), Eq, IntLiter(40)), Println(StrLiter("Correct")), Println(StrLiter("Wrong")))) }.generate(_, 4):
