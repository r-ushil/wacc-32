.data:
.text:
.global main:
main:
Program { funcs: [Func { ident: "f", signature: FuncSig { params: [], return_type: Int }, body: Return(IntLiter(5)) }], statement: Sequence(Declaration(Int, "f", Call("f", [])), Println(Ident("f"))) }.generate(_, 4):
