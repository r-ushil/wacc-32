.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "x", Expr(IntLiter(0))), Sequence(Declaration(Int, "y", Expr(IntLiter(0))), Sequence(Print(StrLiter("Enter the first number: ")), Sequence(Read(Ident("x")), Sequence(Print(StrLiter("Enter the second number: ")), Sequence(Read(Ident("y")), Sequence(Print(StrLiter("Initial value of x: ")), Sequence(Println(Ident("x")), Sequence(While(BinaryApp(Ident("y"), Gt, IntLiter(0)), Sequence(Print(StrLiter("(+)")), Sequence(Assignment(Ident("x"), Expr(BinaryApp(Ident("x"), Add, IntLiter(1)))), Assignment(Ident("y"), Expr(BinaryApp(Ident("y"), Sub, IntLiter(1))))))), Sequence(Println(StrLiter("")), Sequence(Print(StrLiter("final value of x: ")), Println(Ident("x"))))))))))))) }.generate(_, 4):
