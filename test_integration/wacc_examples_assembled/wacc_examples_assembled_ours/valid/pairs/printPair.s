.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Pair(Int, Char), "p", Pair(IntLiter(10), CharLiter('a'))), Sequence(Print(Ident("p")), Sequence(Print(StrLiter(" = (")), Sequence(Declaration(Int, "x", PairElem(Fst(Ident("p")))), Sequence(Print(Ident("x")), Sequence(Print(StrLiter(", ")), Sequence(Declaration(Char, "c", PairElem(Snd(Ident("p")))), Sequence(Print(Ident("c")), Println(CharLiter(')')))))))))) }.generate(_, 4):
