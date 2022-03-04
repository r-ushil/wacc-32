.data

msg_0:
	.word 83
	.ascii	"OverflowError: the result is too small/large to store in a 4-byte signed-integer.\n\0"
msg_1:
	.word 45
	.ascii	"DivideByZeroError: divide or modulo by zero\n\0"
msg_2:
	.word 3
	.ascii	"%d\0"
msg_3:
	.word 1
	.ascii	"\0"
msg_4:
	.word 5
	.ascii	"%.*s\0"

.text

.global main
main:
	PUSH {lr}
	SUB sp, sp, #16
	LDR r4, =1
	LDR r5, =2
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =3
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =4
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =5
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =6
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =7
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =8
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =9
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =10
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =11
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =12
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =13
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =14
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =15
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =16
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =17
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	STR r4, [sp, #12]
	LDR r4, =-1
	LDR r5, =2
	SUBS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =3
	SUBS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =4
	SUBS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =5
	SUBS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =6
	SUBS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =7
	SUBS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =8
	SUBS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =9
	SUBS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =10
	SUBS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =11
	SUBS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =12
	SUBS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =13
	SUBS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =14
	SUBS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =15
	SUBS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =16
	SUBS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =17
	SUBS r4, r4, r5
	BLVS p_throw_overflow_error
	STR r4, [sp, #8]
	LDR r4, =1
	LDR r5, =2
	SMULL r4, r5, r4, r5
	CMP r5, r4, ASR #31
	BLNE p_throw_overflow_error
	LDR r5, =3
	SMULL r4, r5, r4, r5
	CMP r5, r4, ASR #31
	BLNE p_throw_overflow_error
	LDR r5, =4
	SMULL r4, r5, r4, r5
	CMP r5, r4, ASR #31
	BLNE p_throw_overflow_error
	LDR r5, =5
	SMULL r4, r5, r4, r5
	CMP r5, r4, ASR #31
	BLNE p_throw_overflow_error
	LDR r5, =6
	SMULL r4, r5, r4, r5
	CMP r5, r4, ASR #31
	BLNE p_throw_overflow_error
	LDR r5, =7
	SMULL r4, r5, r4, r5
	CMP r5, r4, ASR #31
	BLNE p_throw_overflow_error
	LDR r5, =8
	SMULL r4, r5, r4, r5
	CMP r5, r4, ASR #31
	BLNE p_throw_overflow_error
	LDR r5, =9
	SMULL r4, r5, r4, r5
	CMP r5, r4, ASR #31
	BLNE p_throw_overflow_error
	LDR r5, =10
	SMULL r4, r5, r4, r5
	CMP r5, r4, ASR #31
	BLNE p_throw_overflow_error
	STR r4, [sp, #4]
	LDR r4, =10
	STR r4, [sp]
	LDR r4, [sp, #12]
	LDR r5, [sp, #8]
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, [sp, #4]
	LDR r6, [sp]
	MOV r0, r5
	MOV r1, r6
	BL p_check_divide_by_zero
	BL __aeabi_idiv
	MOV r5, r0
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	MOV r0, r4
	BL p_print_int
	BL p_print_ln
	LDR r4, [sp, #12]
	LDR r5, [sp, #8]
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, [sp, #4]
	LDR r6, [sp]
	MOV r0, r5
	MOV r1, r6
	BL p_check_divide_by_zero
	BL __aeabi_idiv
	MOV r5, r0
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, =256
	MOV r0, r4
	MOV r1, r5
	BL p_check_divide_by_zero
	BL __aeabi_idivmod
	MOV r4, r1
	MOV r0, r4
	BL p_print_int
	BL p_print_ln
	LDR r4, [sp, #12]
	LDR r5, [sp, #8]
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, [sp, #4]
	LDR r6, [sp]
	MOV r0, r5
	MOV r1, r6
	BL p_check_divide_by_zero
	BL __aeabi_idiv
	MOV r5, r0
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	MOV r0, r4
	BL exit
	ADD sp, sp, #16
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
p_print_int:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_2
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
p_print_ln:
	PUSH {lr}
	LDR r0, =msg_3
	ADD r0, r0, #4
	BL puts
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
	LDR r0, =msg_4
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
