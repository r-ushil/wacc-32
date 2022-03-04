.text

.global main
main:
	PUSH {lr}
	MOV r4, #1
	CMP r4, #0
	BEQ L0
	B L1
L0:
L1:
	LDR r0, =0
	POP {pc}
	.ltorg
