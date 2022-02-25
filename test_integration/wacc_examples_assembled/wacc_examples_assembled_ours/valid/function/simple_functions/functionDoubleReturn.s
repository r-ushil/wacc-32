.data:
.text:
.global main:
main:
Program { funcs: [Func { ident: "f", signature: FuncSig { params: [], return_type: Int }, body: Sequence(Return(IntLiter(3)), Return(IntLiter(5))) }], statement: Sequence(Declaration(Int, "ret", Call("f", [])), Println(Ident("ret"))) }.generate(_, 4):
