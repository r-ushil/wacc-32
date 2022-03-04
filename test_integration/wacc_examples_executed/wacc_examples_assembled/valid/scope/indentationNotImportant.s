.text

.global main
main:
	PUSH {lr}
	B L0
L1:
L0:
	MOV r4, #0
	CMP r4, #1
	BEQ L1
	LDR r0, =0
	POP {pc}
	.ltorg
