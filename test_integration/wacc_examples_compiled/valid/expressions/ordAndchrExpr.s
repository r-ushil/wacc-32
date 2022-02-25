.data

msg_0:
	.word 4
	.ascii	" is "
msg_1:
	.word 4
	.ascii	" is "
msg_2:
	.word 5
	.ascii	"%.*s\0"
msg_3:
	.word 3
	.ascii	"%d\0"
msg_4:
	.word 1
	.ascii	"\0"

.text

.global main
main:
	PUSH {lr}
	SUB sp, sp, #5
	MOV r4, #'a'
	STRB r4, [sp, #4]
	LDR r4, =99
	STR r4, [sp]
	LDRSB r4, [sp, #4]
	MOV r0, r4
	BL putchar
	LDR r4, =msg_0
	MOV r0, r4
	BL p_print_string
	LDRSB r4, [sp, #4]
	MOV r0, r4
	BL p_print_int
	BL p_print_ln
	LDR r4, [sp]
	MOV r0, r4
	BL p_print_int
	LDR r4, =msg_1
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp]
	MOV r0, r4
	BL putchar
	BL p_print_ln
	ADD sp, sp, #5
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
p_print_int:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_3
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
p_print_ln:
	PUSH {lr}
	LDR r0, =msg_4
	ADD r0, r0, #4
	BL puts
	MOV r0, #0
	BL fflush
	POP {pc}
