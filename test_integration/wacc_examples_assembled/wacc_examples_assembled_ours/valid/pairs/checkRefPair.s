.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Pair(Int, Char), "p", Pair(IntLiter(10), CharLiter('a'))), Sequence(Declaration(Pair(Int, Char), "q", Expr(Ident("p"))), Sequence(Println(Ident("p")), Sequence(Println(Ident("q")), Sequence(Println(BinaryApp(Ident("p"), Eq, Ident("q"))), Sequence(Declaration(Int, "x", PairElem(Fst(Ident("p")))), Sequence(Declaration(Int, "y", PairElem(Fst(Ident("q")))), Sequence(Println(Ident("x")), Sequence(Println(Ident("y")), Sequence(Println(BinaryApp(Ident("x"), Eq, Ident("y"))), Sequence(Declaration(Char, "c1", PairElem(Snd(Ident("p")))), Sequence(Declaration(Char, "c2", PairElem(Snd(Ident("q")))), Sequence(Println(Ident("c1")), Sequence(Println(Ident("c2")), Println(BinaryApp(Ident("c1"), Eq, Ident("c2"))))))))))))))))) }.generate(_, 4):
