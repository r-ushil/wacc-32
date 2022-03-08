.data

msg_0:
	.word 25
	.ascii	"Please input an integer: "
msg_1:
	.word 12
	.ascii	"echo input: "
msg_2:
	.word 39
	.ascii	"Do you want to continue entering input?"
msg_3:
	.word 34
	.ascii	"(enter Y for \'yes\' and N for \'no\')"
msg_4:
	.word 5
	.ascii	"%.*s\0"
msg_5:
	.word 3
	.ascii	"%d\0"
msg_6:
	.word 3
	.ascii	"%d\0"
msg_7:
	.word 1
	.ascii	"\0"
msg_8:
	.word 4
	.ascii	" %c\0"

.text

.global main
main:
	PUSH {lr}
	SUB sp, sp, #5
	MOV r4, #'Y'
	STRB r4, [sp, #4]
	LDR r4, =0
	STR r4, [sp]
	B L0
L1:
	LDR r4, =msg_0
	MOV r0, r4
	BL p_print_string
	ADD r4, sp, #0
	MOV r0, r4
	BL p_read_int
	LDR r4, =msg_1
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp]
	MOV r0, r4
	BL p_print_int
	BL p_print_ln
	LDR r4, =msg_2
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_3
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	ADD r4, sp, #4
	MOV r0, r4
	BL p_read_char
L0:
	LDRSB r4, [sp, #4]
	MOV r5, #'N'
	CMP r4, r5
	MOVNE r4, #1
	MOVEQ r4, #0
	CMP r4, #1
	BEQ L1
	ADD sp, sp, #5
	LDR r0, =0
	POP {pc}
	.ltorg
p_print_string:
	PUSH {lr}
	LDR r1, [r0]
	ADD r2, r0, #4
	LDR r0, =msg_4
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
p_read_int:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_5
	ADD r0, r0, #4
	BL scanf
	POP {pc}
p_print_int:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_6
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
p_print_ln:
	PUSH {lr}
	LDR r0, =msg_7
	ADD r0, r0, #4
	BL puts
	MOV r0, #0
	BL fflush
	POP {pc}
p_read_char:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_8
	ADD r0, r0, #4
	BL scanf
	POP {pc}
