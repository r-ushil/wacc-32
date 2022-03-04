.data

msg_0:
	.word 5
	.ascii	"x is "
msg_1:
	.word 9
	.ascii	"x is now "
msg_2:
	.word 5
	.ascii	"y is "
msg_3:
	.word 11
	.ascii	"y is still "
msg_4:
	.word 5
	.ascii	"%.*s\0"
msg_5:
	.word 3
	.ascii	"%d\0"
msg_6:
	.word 1
	.ascii	"\0"

.text

.global main
f_f:
	PUSH {lr}
	LDR r4, =msg_0
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #4]
	MOV r0, r4
	BL p_print_int
	BL p_print_ln
	LDR r4, =5
	STR r4, [sp, #4]
	LDR r4, =msg_1
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #4]
	MOV r0, r4
	BL p_print_int
	BL p_print_ln
	LDR r4, [sp, #4]
	MOV r0, r4
	POP {pc}
	POP {pc}
	.ltorg
main:
	PUSH {lr}
	SUB sp, sp, #8
	LDR r4, =1
	STR r4, [sp, #4]
	LDR r4, =msg_2
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #4]
	MOV r0, r4
	BL p_print_int
	BL p_print_ln
	LDR r4, [sp, #4]
	STR r4, [sp, #-4]!
	BL f_f
	ADD sp, sp, #4
	MOV r4, r0
	STR r4, [sp]
	LDR r4, =msg_3
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #4]
	MOV r0, r4
	BL p_print_int
	BL p_print_ln
	ADD sp, sp, #8
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
p_print_int:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_5
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
p_print_ln:
	PUSH {lr}
	LDR r0, =msg_6
	ADD r0, r0, #4
	BL puts
	MOV r0, #0
	BL fflush
	POP {pc}
