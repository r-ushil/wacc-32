.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Pair(Int, Char), "p", Pair(IntLiter(10), CharLiter('a'))), Sequence(Declaration(Int, "f", PairElem(Fst(Ident("p")))), Sequence(Println(Ident("f")), Sequence(Assignment(PairElem(Fst(Ident("p"))), Expr(IntLiter(42))), Sequence(Assignment(Ident("f"), PairElem(Fst(Ident("p")))), Println(Ident("f"))))))) }.generate(_, 4):
