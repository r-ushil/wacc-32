.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "n", Expr(IntLiter(0))), Sequence(While(BinaryApp(Ident("n"), Neq, IntLiter(1)), Sequence(Assignment(Ident("n"), Expr(IntLiter(1))), Println(StrLiter("Change n")))), Println(StrLiter("Should print \"Change n\" once before.")))) }.generate(_, 4):
