.data:
.text:
.global main:
main:
Program { funcs: [Func { ident: "returnInWhile", signature: FuncSig { params: [], return_type: Int }, body: Sequence(While(BoolLiter(true), Sequence(Return(IntLiter(1)), Println(StrLiter("How on Earth did we get here?")))), Return(IntLiter(2))) }], statement: Sequence(Declaration(Int, "x", Call("returnInWhile", [])), Println(Ident("x"))) }.generate(_, 4):
