.data

msg_0:
	.word 83
	.ascii	"OverflowError: the result is too small/large to store in a 4-byte signed-integer.\n\0"
msg_1:
	.word 45
	.ascii	"DivideByZeroError: divide or modulo by zero\n\0"
msg_2:
	.word 5
	.ascii	"%.*s\0"

.text

.global main
main:
	PUSH {lr}
	SUB sp, sp, #4
	LDR r4, =2
	LDR r5, =3
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =2
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =1
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =1
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =1
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =1
	LDR r6, =2
	ADDS r5, r5, r6
	BLVS p_throw_overflow_error
	LDR r6, =3
	LDR r7, =4
	LDR r8, =6
	MOV r0, r7
	MOV r1, r8
	BL p_check_divide_by_zero
	BL __aeabi_idiv
	MOV r7, r0
	SUBS r6, r6, r7
	BLVS p_throw_overflow_error
	SMULL r5, r6, r5, r6
	CMP r6, r5, ASR #31
	BLNE p_throw_overflow_error
	LDR r6, =2
	LDR r7, =18
	LDR r8, =17
	SUBS r7, r7, r8
	BLVS p_throw_overflow_error
	SMULL r6, r7, r6, r7
	CMP r7, r6, ASR #31
	BLNE p_throw_overflow_error
	LDR r7, =3
	LDR r8, =4
	SMULL r7, r8, r7, r8
	CMP r8, r7, ASR #31
	BLNE p_throw_overflow_error
	LDR r8, =4
	MOV r0, r7
	MOV r1, r8
	BL p_check_divide_by_zero
	BL __aeabi_idiv
	MOV r7, r0
	LDR r8, =6
	ADDS r7, r7, r8
	BLVS p_throw_overflow_error
	ADDS r6, r6, r7
	BLVS p_throw_overflow_error
	MOV r0, r5
	MOV r1, r6
	BL p_check_divide_by_zero
	BL __aeabi_idiv
	MOV r5, r0
	SUBS r4, r4, r5
	BLVS p_throw_overflow_error
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	BL exit
	ADD sp, sp, #4
	LDR r0, =0
	POP {pc}
	.ltorg
p_throw_overflow_error:
	LDR r0, =msg_0
	BL p_throw_runtime_error
p_check_divide_by_zero:
	PUSH {lr}
	CMP r1, #0
	LDREQ r0, =msg_1
	BLEQ p_throw_runtime_error
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
