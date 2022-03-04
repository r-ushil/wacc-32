.data

msg_0:
	.word 3
	.ascii	"%d\0"
msg_1:
	.word 1
	.ascii	"\0"

.text

.global main
main:
	PUSH {lr}
	SUB sp, sp, #8
	LDR r4, =42
	STR r4, [sp, #4]
	LDR r4, =0
	STR r4, [sp]
	LDR r4, [sp, #4]
	MOV r0, r4
	BL p_print_int
	BL p_print_ln
	LDR r4, [sp]
	MOV r0, r4
	BL p_print_int
	BL p_print_ln
	ADD sp, sp, #8
	LDR r0, =0
	POP {pc}
	.ltorg
p_print_int:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_0
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
p_print_ln:
	PUSH {lr}
	LDR r0, =msg_1
	ADD r0, r0, #4
	BL puts
	MOV r0, #0
	BL fflush
	POP {pc}
