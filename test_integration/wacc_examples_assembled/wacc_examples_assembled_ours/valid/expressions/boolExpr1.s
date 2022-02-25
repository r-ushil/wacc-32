.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Bool, "b", Expr(UnaryApp(Bang, BinaryApp(BinaryApp(BoolLiter(true), And, BoolLiter(false)), Or, BinaryApp(BoolLiter(true), And, BoolLiter(false)))))), If(BinaryApp(Ident("b"), Eq, BoolLiter(true)), Println(StrLiter("Correct")), Println(StrLiter("Wrong")))) }.generate(_, 4):
