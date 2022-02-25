.data

msg_0:
	.word 29
	.ascii	"How on Earth did we get here?"
msg_1:
	.word 5
	.ascii	"%.*s\0"
msg_2:
	.word 1
	.ascii	"\0"
msg_3:
	.word 3
	.ascii	"%d\0"

.text

.global main
f_returnInWhile:
	PUSH {lr}
	B L0
L1:
	LDR r4, =1
	MOV r0, r4
	POP {pc}
	LDR r4, =msg_0
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
L0:
	MOV r4, #1
	CMP r4, #1
	BEQ L1
	LDR r4, =2
	MOV r0, r4
	POP {pc}
	POP {pc}
	.ltorg
main:
	PUSH {lr}
	SUB sp, sp, #4
	BL f_returnInWhile
	MOV r4, r0
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	BL p_print_int
	BL p_print_ln
	ADD sp, sp, #4
	LDR r0, =0
	POP {pc}
	.ltorg
p_print_string:
	PUSH {lr}
	LDR r1, [r0]
	ADD r2, r0, #4
	LDR r0, =msg_1
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
p_print_ln:
	PUSH {lr}
	LDR r0, =msg_2
	ADD r0, r0, #4
	BL puts
	MOV r0, #0
	BL fflush
	POP {pc}
p_print_int:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_3
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
