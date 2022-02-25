.data:
.text:
.global main:
main:
Program { funcs: [Func { ident: "getPair", signature: FuncSig { params: [], return_type: Pair(Int, Int) }, body: Sequence(Declaration(Pair(Int, Int), "p", Pair(IntLiter(10), IntLiter(15))), Return(Ident("p"))) }], statement: Sequence(Declaration(Pair(Int, Int), "p", Call("getPair", [])), Sequence(Declaration(Int, "x", PairElem(Fst(Ident("p")))), Println(Ident("x")))) }.generate(_, 4):
