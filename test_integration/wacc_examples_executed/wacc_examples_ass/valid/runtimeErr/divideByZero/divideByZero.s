.data

msg_0:
	.word 45
	.ascii	"DivideByZeroError: divide or modulo by zero\n\0"
msg_1:
	.word 3
	.ascii	"%d\0"
msg_2:
	.word 5
	.ascii	"%.*s\0"

.text

.global main
main:
	PUSH {lr}
	SUB sp, sp, #8
	LDR r4, =10
	STR r4, [sp, #4]
	LDR r4, =0
	STR r4, [sp]
	LDR r4, [sp, #4]
	LDR r5, [sp]
	MOV r0, r4
	MOV r1, r5
	BL p_check_divide_by_zero
	BL __aeabi_idiv
	MOV r4, r0
	MOV r0, r4
	BL p_print_int
	ADD sp, sp, #8
	LDR r0, =0
	POP {pc}
	.ltorg
p_check_divide_by_zero:
	PUSH {lr}
	CMP r1, #0
	LDREQ r0, =msg_0
	BLEQ p_throw_runtime_error
	POP {pc}
p_print_int:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_1
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
p_throw_runtime_error:
	BL p_print_string
	MOV r0, #-1
	BL exit
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
