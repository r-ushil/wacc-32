.data

msg_0:
	.word 61
	.ascii	"This program calculates the nth fibonacci number recursively."
msg_1:
	.word 42
	.ascii	"Please enter n (should not be too large): "
msg_2:
	.word 15
	.ascii	"The input n is "
msg_3:
	.word 28
	.ascii	"The nth fibonacci number is "
msg_4:
	.word 83
	.ascii	"OverflowError: the result is too small/large to store in a 4-byte signed-integer.\n\0"
msg_5:
	.word 5
	.ascii	"%.*s\0"
msg_6:
	.word 1
	.ascii	"\0"
msg_7:
	.word 3
	.ascii	"%d\0"
msg_8:
	.word 3
	.ascii	"%d\0"

.text

.global main
f_fibonacci:
	PUSH {lr}
	SUB sp, sp, #8
	LDR r4, [sp, #12]
	LDR r5, =1
	CMP r4, r5
	MOVLE r4, #1
	MOVGT r4, #0
	CMP r4, #0
	BEQ L0
	LDR r4, [sp, #12]
	MOV r0, r4
	ADD sp, sp, #8
	POP {pc}
	B L1
L0:
L1:
	LDR r4, [sp, #12]
	LDR r5, =1
	SUBS r4, r4, r5
	BLVS p_throw_overflow_error
	STR r4, [sp, #-4]!
	BL f_fibonacci
	ADD sp, sp, #4
	MOV r4, r0
	STR r4, [sp, #4]
	LDR r4, [sp, #12]
	LDR r5, =2
	SUBS r4, r4, r5
	BLVS p_throw_overflow_error
	STR r4, [sp, #-4]!
	BL f_fibonacci
	ADD sp, sp, #4
	MOV r4, r0
	STR r4, [sp]
	LDR r4, [sp, #4]
	LDR r5, [sp]
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	MOV r0, r4
	ADD sp, sp, #8
	POP {pc}
	POP {pc}
	.ltorg
main:
	PUSH {lr}
	SUB sp, sp, #8
	LDR r4, =msg_0
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_1
	MOV r0, r4
	BL p_print_string
	LDR r4, =0
	STR r4, [sp, #4]
	ADD r4, sp, #4
	MOV r0, r4
	BL p_read_int
	LDR r4, =msg_2
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #4]
	MOV r0, r4
	BL p_print_int
	BL p_print_ln
	LDR r4, =msg_3
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #4]
	STR r4, [sp, #-4]!
	BL f_fibonacci
	ADD sp, sp, #4
	MOV r4, r0
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	BL p_print_int
	BL p_print_ln
	ADD sp, sp, #8
	LDR r0, =0
	POP {pc}
	.ltorg
p_throw_overflow_error:
	LDR r0, =msg_4
	BL p_throw_runtime_error
p_print_string:
	PUSH {lr}
	LDR r1, [r0]
	ADD r2, r0, #4
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
p_read_int:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_7
	ADD r0, r0, #4
	BL scanf
	POP {pc}
p_print_int:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_8
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
p_throw_runtime_error:
	BL p_print_string
	MOV r0, #-1
	BL exit
