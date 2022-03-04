.data

msg_0:
	.word 83
	.ascii	"OverflowError: the result is too small/large to store in a 4-byte signed-integer.\n\0"
msg_1:
	.word 5
	.ascii	"%.*s\0"

.text

.global main
f_rec:
	PUSH {lr}
	LDR r4, [sp, #4]
	LDR r5, =0
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L0
	B L1
L0:
	SUB sp, sp, #4
	LDR r4, [sp, #8]
	LDR r5, =1
	SUBS r4, r4, r5
	BLVS p_throw_overflow_error
	STR r4, [sp, #-4]!
	BL f_rec
	ADD sp, sp, #4
	MOV r4, r0
	STR r4, [sp]
	ADD sp, sp, #4
L1:
	LDR r4, =42
	MOV r0, r4
	POP {pc}
	POP {pc}
	.ltorg
main:
	PUSH {lr}
	SUB sp, sp, #4
	LDR r4, =0
	STR r4, [sp]
	LDR r4, =8
	STR r4, [sp, #-4]!
	BL f_rec
	ADD sp, sp, #4
	MOV r4, r0
	STR r4, [sp]
	ADD sp, sp, #4
	LDR r0, =0
	POP {pc}
	.ltorg
p_throw_overflow_error:
	LDR r0, =msg_0
	BL p_throw_runtime_error
p_throw_runtime_error:
	BL p_print_string
	MOV r0, #-1
	BL exit
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
