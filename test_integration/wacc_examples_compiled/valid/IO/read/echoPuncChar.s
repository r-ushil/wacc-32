.data

msg_0:
	.word 25
	.ascii	"enter a character to echo"
msg_1:
	.word 5
	.ascii	"%.*s\0"
msg_2:
	.word 1
	.ascii	"\0"
msg_3:
	.word 4
	.ascii	" %c\0"

.text

.global main
main:
	PUSH {lr}
	SUB sp, sp, #1
	MOV r4, #0
	STRB r4, [sp]
	LDR r4, =msg_0
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	ADD r4, sp, #0
	MOV r0, r4
	BL p_read_char
	LDRSB r4, [sp]
	MOV r0, r4
	BL putchar
	BL p_print_ln
	ADD sp, sp, #1
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
p_read_char:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_3
	ADD r0, r0, #4
	BL scanf
	POP {pc}
