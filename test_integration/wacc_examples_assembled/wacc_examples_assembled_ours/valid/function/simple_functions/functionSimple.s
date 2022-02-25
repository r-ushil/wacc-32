.data:
.text:
.global main:
main:
Program { funcs: [Func { ident: "f", signature: FuncSig { params: [], return_type: Int }, body: Return(IntLiter(0)) }], statement: Sequence(Declaration(Int, "x", Call("f", [])), Println(Ident("x"))) }.generate(_, 4):
