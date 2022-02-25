.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "x", Expr(IntLiter(2))), Sequence(Declaration(Int, "y", Expr(IntLiter(4))), Sequence(Declaration(Int, "z", Expr(IntLiter(4))), Sequence(Declaration(Bool, "b", Expr(BinaryApp(Ident("x"), Eq, Ident("y")))), Sequence(Println(Ident("b")), Sequence(Println(BinaryApp(Ident("x"), Eq, Ident("y"))), Println(BinaryApp(Ident("y"), Eq, Ident("z"))))))))) }.generate(_, 4):
