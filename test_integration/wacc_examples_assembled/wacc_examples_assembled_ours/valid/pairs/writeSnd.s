.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Pair(Int, Char), "p", Pair(IntLiter(10), CharLiter('a'))), Sequence(Declaration(Char, "s", PairElem(Snd(Ident("p")))), Sequence(Println(Ident("s")), Sequence(Assignment(PairElem(Snd(Ident("p"))), Expr(CharLiter('Z'))), Sequence(Assignment(Ident("s"), PairElem(Snd(Ident("p")))), Println(Ident("s"))))))) }.generate(_, 4):
