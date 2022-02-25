.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "x", Expr(BinaryApp(BinaryApp(IntLiter(2), Add, BinaryApp(IntLiter(3), Add, BinaryApp(IntLiter(2), Add, BinaryApp(IntLiter(1), Add, BinaryApp(IntLiter(1), Add, IntLiter(1)))))), Sub, BinaryApp(BinaryApp(IntLiter(1), Add, IntLiter(2)), Mul, BinaryApp(BinaryApp(IntLiter(3), Sub, BinaryApp(IntLiter(4), Div, IntLiter(6))), Div, BinaryApp(BinaryApp(IntLiter(2), Mul, BinaryApp(IntLiter(18), Sub, IntLiter(17))), Add, BinaryApp(BinaryApp(IntLiter(3), Mul, BinaryApp(IntLiter(4), Div, IntLiter(4))), Add, IntLiter(6)))))))), Exit(Ident("x"))) }.generate(_, 4):
