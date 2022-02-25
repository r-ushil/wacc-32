.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Pair(Pair(Any, Any), Pair(Any, Any)), "p", Pair(PairLiter, PairLiter)), Sequence(Print(Ident("p")), Sequence(Print(StrLiter(" = (")), Sequence(Declaration(Pair(Pair(Any, Any), Pair(Any, Any)), "q", PairElem(Fst(Ident("p")))), Sequence(Print(Ident("q")), Sequence(Print(StrLiter(",")), Sequence(Declaration(Pair(Int, Bool), "r", PairElem(Snd(Ident("p")))), Sequence(Print(Ident("r")), Println(StrLiter(")")))))))))) }.generate(_, 4):
