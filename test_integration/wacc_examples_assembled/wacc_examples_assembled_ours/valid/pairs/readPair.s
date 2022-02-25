.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Pair(Char, Int), "p", Pair(CharLiter('\u{0}'), IntLiter(0))), Sequence(Print(StrLiter("Please enter the first element (char): ")), Sequence(Declaration(Char, "c", Expr(CharLiter('0'))), Sequence(Read(Ident("c")), Sequence(Assignment(PairElem(Fst(Ident("p"))), Expr(Ident("c"))), Sequence(Print(StrLiter("Please enter the second element (int): ")), Sequence(Declaration(Int, "i", Expr(IntLiter(0))), Sequence(Read(Ident("i")), Sequence(Assignment(PairElem(Snd(Ident("p"))), Expr(Ident("i"))), Sequence(Assignment(Ident("c"), Expr(CharLiter('\u{0}'))), Sequence(Assignment(Ident("i"), Expr(IntLiter(-1))), Sequence(Print(StrLiter("The first element was ")), Sequence(Assignment(Ident("c"), PairElem(Fst(Ident("p")))), Sequence(Println(Ident("c")), Sequence(Print(StrLiter("The second element was ")), Sequence(Assignment(Ident("i"), PairElem(Snd(Ident("p")))), Println(Ident("i")))))))))))))))))) }.generate(_, 4):
