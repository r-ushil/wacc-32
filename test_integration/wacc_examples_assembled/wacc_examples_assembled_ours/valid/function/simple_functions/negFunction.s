.data:
.text:
.global main:
main:
Program { funcs: [Func { ident: "neg", signature: FuncSig { params: [(Bool, "b")], return_type: Bool }, body: Return(UnaryApp(Bang, Ident("b"))) }], statement: Sequence(Declaration(Bool, "b", Expr(BoolLiter(true))), Sequence(Println(Ident("b")), Sequence(Assignment(Ident("b"), Call("neg", [Ident("b")])), Sequence(Println(Ident("b")), Sequence(Assignment(Ident("b"), Call("neg", [Ident("b")])), Sequence(Assignment(Ident("b"), Call("neg", [Ident("b")])), Sequence(Assignment(Ident("b"), Call("neg", [Ident("b")])), Println(Ident("b"))))))))) }.generate(_, 4):
