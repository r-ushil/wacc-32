.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "i", Expr(IntLiter(1))), Sequence(Println(StrLiter("Can you count to 10?")), While(BinaryApp(Ident("i"), Lte, IntLiter(10)), Sequence(Println(Ident("i")), Assignment(Ident("i"), Expr(BinaryApp(Ident("i"), Add, IntLiter(1)))))))) }.generate(_, 4):
