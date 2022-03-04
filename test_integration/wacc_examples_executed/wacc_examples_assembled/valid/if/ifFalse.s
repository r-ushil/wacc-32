.data

msg_0:
	.word 8
	.ascii	"not here"
msg_1:
	.word 4
	.ascii	"here"
msg_2:
	.word 5
	.ascii	"%.*s\0"
msg_3:
	.word 1
	.ascii	"\0"

.text

.global main
main:
	PUSH {lr}
	MOV r4, #0
	CMP r4, #0
	BEQ L0
	LDR r4, =msg_0
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	B L1
L0:
	LDR r4, =msg_1
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
L1:
	LDR r0, =0
	POP {pc}
	.ltorg
p_print_string:
	PUSH {lr}
	LDR r1, [r0]
	ADD r2, r0, #4
	LDR r0, =msg_2
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
p_print_ln:
	PUSH {lr}
	LDR r0, =msg_3
	ADD r0, r0, #4
	BL puts
	MOV r0, #0
	BL fflush
	POP {pc}
