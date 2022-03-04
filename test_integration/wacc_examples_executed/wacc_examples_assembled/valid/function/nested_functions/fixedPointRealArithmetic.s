.data

msg_0:
	.word 24
	.ascii	"Using fixed-point real: "
msg_1:
	.word 3
	.ascii	" / "
msg_2:
	.word 3
	.ascii	" * "
msg_3:
	.word 3
	.ascii	" = "
msg_4:
	.word 83
	.ascii	"OverflowError: the result is too small/large to store in a 4-byte signed-integer.\n\0"
msg_5:
	.word 45
	.ascii	"DivideByZeroError: divide or modulo by zero\n\0"
msg_6:
	.word 5
	.ascii	"%.*s\0"
msg_7:
	.word 3
	.ascii	"%d\0"
msg_8:
	.word 1
	.ascii	"\0"

.text

.global main
f_q:
	PUSH {lr}
	LDR r4, =14
	MOV r0, r4
	POP {pc}
	POP {pc}
	.ltorg
f_power:
	PUSH {lr}
	SUB sp, sp, #4
	LDR r4, =1
	STR r4, [sp]
	B L0
L1:
	LDR r4, [sp]
	LDR r5, [sp, #8]
	SMULL r4, r5, r4, r5
	CMP r5, r4, ASR #31
	BLNE p_throw_overflow_error
	STR r4, [sp]
	LDR r4, [sp, #12]
	LDR r5, =1
	SUBS r4, r4, r5
	BLVS p_throw_overflow_error
	STR r4, [sp, #12]
L0:
	LDR r4, [sp, #12]
	LDR r5, =0
	CMP r4, r5
	MOVGT r4, #1
	MOVLE r4, #0
	CMP r4, #1
	BEQ L1
	LDR r4, [sp]
	MOV r0, r4
	ADD sp, sp, #4
	POP {pc}
	POP {pc}
	.ltorg
f_f:
	PUSH {lr}
	SUB sp, sp, #8
	BL f_q
	MOV r4, r0
	STR r4, [sp, #4]
	LDR r4, [sp, #4]
	STR r4, [sp, #-4]!
	LDR r4, =2
	STR r4, [sp, #-4]!
	BL f_power
	ADD sp, sp, #8
	MOV r4, r0
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	ADD sp, sp, #8
	POP {pc}
	POP {pc}
	.ltorg
f_intToFixedPoint:
	PUSH {lr}
	SUB sp, sp, #4
	BL f_f
	MOV r4, r0
	STR r4, [sp]
	LDR r4, [sp, #8]
	LDR r5, [sp]
	SMULL r4, r5, r4, r5
	CMP r5, r4, ASR #31
	BLNE p_throw_overflow_error
	MOV r0, r4
	ADD sp, sp, #4
	POP {pc}
	POP {pc}
	.ltorg
f_fixedPointToIntRoundDown:
	PUSH {lr}
	SUB sp, sp, #4
	BL f_f
	MOV r4, r0
	STR r4, [sp]
	LDR r4, [sp, #8]
	LDR r5, [sp]
	MOV r0, r4
	MOV r1, r5
	BL p_check_divide_by_zero
	BL __aeabi_idiv
	MOV r4, r0
	MOV r0, r4
	ADD sp, sp, #4
	POP {pc}
	POP {pc}
	.ltorg
f_fixedPointToIntRoundNear:
	PUSH {lr}
	SUB sp, sp, #4
	BL f_f
	MOV r4, r0
	STR r4, [sp]
	LDR r4, [sp, #8]
	LDR r5, =0
	CMP r4, r5
	MOVGE r4, #1
	MOVLT r4, #0
	CMP r4, #0
	BEQ L2
	LDR r4, [sp, #8]
	LDR r5, [sp]
	LDR r6, =2
	MOV r0, r5
	MOV r1, r6
	BL p_check_divide_by_zero
	BL __aeabi_idiv
	MOV r5, r0
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, [sp]
	MOV r0, r4
	MOV r1, r5
	BL p_check_divide_by_zero
	BL __aeabi_idiv
	MOV r4, r0
	MOV r0, r4
	ADD sp, sp, #4
	POP {pc}
	B L3
L2:
	LDR r4, [sp, #8]
	LDR r5, [sp]
	LDR r6, =2
	MOV r0, r5
	MOV r1, r6
	BL p_check_divide_by_zero
	BL __aeabi_idiv
	MOV r5, r0
	SUBS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, [sp]
	MOV r0, r4
	MOV r1, r5
	BL p_check_divide_by_zero
	BL __aeabi_idiv
	MOV r4, r0
	MOV r0, r4
	ADD sp, sp, #4
	POP {pc}
L3:
	POP {pc}
	.ltorg
f_add:
	PUSH {lr}
	LDR r4, [sp, #4]
	LDR r5, [sp, #8]
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	MOV r0, r4
	POP {pc}
	POP {pc}
	.ltorg
f_subtract:
	PUSH {lr}
	LDR r4, [sp, #4]
	LDR r5, [sp, #8]
	SUBS r4, r4, r5
	BLVS p_throw_overflow_error
	MOV r0, r4
	POP {pc}
	POP {pc}
	.ltorg
f_addByInt:
	PUSH {lr}
	SUB sp, sp, #4
	BL f_f
	MOV r4, r0
	STR r4, [sp]
	LDR r4, [sp, #8]
	LDR r5, [sp, #12]
	LDR r6, [sp]
	SMULL r5, r6, r5, r6
	CMP r6, r5, ASR #31
	BLNE p_throw_overflow_error
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	MOV r0, r4
	ADD sp, sp, #4
	POP {pc}
	POP {pc}
	.ltorg
f_subtractByInt:
	PUSH {lr}
	SUB sp, sp, #4
	BL f_f
	MOV r4, r0
	STR r4, [sp]
	LDR r4, [sp, #8]
	LDR r5, [sp, #12]
	LDR r6, [sp]
	SMULL r5, r6, r5, r6
	CMP r6, r5, ASR #31
	BLNE p_throw_overflow_error
	SUBS r4, r4, r5
	BLVS p_throw_overflow_error
	MOV r0, r4
	ADD sp, sp, #4
	POP {pc}
	POP {pc}
	.ltorg
f_multiply:
	PUSH {lr}
	SUB sp, sp, #4
	BL f_f
	MOV r4, r0
	STR r4, [sp]
	LDR r4, [sp, #8]
	LDR r5, [sp, #12]
	SMULL r4, r5, r4, r5
	CMP r5, r4, ASR #31
	BLNE p_throw_overflow_error
	LDR r5, [sp]
	MOV r0, r4
	MOV r1, r5
	BL p_check_divide_by_zero
	BL __aeabi_idiv
	MOV r4, r0
	MOV r0, r4
	ADD sp, sp, #4
	POP {pc}
	POP {pc}
	.ltorg
f_multiplyByInt:
	PUSH {lr}
	LDR r4, [sp, #4]
	LDR r5, [sp, #8]
	SMULL r4, r5, r4, r5
	CMP r5, r4, ASR #31
	BLNE p_throw_overflow_error
	MOV r0, r4
	POP {pc}
	POP {pc}
	.ltorg
f_divide:
	PUSH {lr}
	SUB sp, sp, #4
	BL f_f
	MOV r4, r0
	STR r4, [sp]
	LDR r4, [sp, #8]
	LDR r5, [sp]
	SMULL r4, r5, r4, r5
	CMP r5, r4, ASR #31
	BLNE p_throw_overflow_error
	LDR r5, [sp, #12]
	MOV r0, r4
	MOV r1, r5
	BL p_check_divide_by_zero
	BL __aeabi_idiv
	MOV r4, r0
	MOV r0, r4
	ADD sp, sp, #4
	POP {pc}
	POP {pc}
	.ltorg
f_divideByInt:
	PUSH {lr}
	LDR r4, [sp, #4]
	LDR r5, [sp, #8]
	MOV r0, r4
	MOV r1, r5
	BL p_check_divide_by_zero
	BL __aeabi_idiv
	MOV r4, r0
	MOV r0, r4
	POP {pc}
	POP {pc}
	.ltorg
main:
	PUSH {lr}
	SUB sp, sp, #16
	LDR r4, =10
	STR r4, [sp, #12]
	LDR r4, =3
	STR r4, [sp, #8]
	LDR r4, =msg_0
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #12]
	MOV r0, r4
	BL p_print_int
	LDR r4, =msg_1
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #8]
	MOV r0, r4
	BL p_print_int
	LDR r4, =msg_2
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #8]
	MOV r0, r4
	BL p_print_int
	LDR r4, =msg_3
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #12]
	STR r4, [sp, #-4]!
	BL f_intToFixedPoint
	ADD sp, sp, #4
	MOV r4, r0
	STR r4, [sp, #4]
	LDR r4, [sp, #8]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #8]
	STR r4, [sp, #-4]!
	BL f_divideByInt
	ADD sp, sp, #8
	MOV r4, r0
	STR r4, [sp, #4]
	LDR r4, [sp, #8]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #8]
	STR r4, [sp, #-4]!
	BL f_multiplyByInt
	ADD sp, sp, #8
	MOV r4, r0
	STR r4, [sp, #4]
	LDR r4, [sp, #4]
	STR r4, [sp, #-4]!
	BL f_fixedPointToIntRoundNear
	ADD sp, sp, #4
	MOV r4, r0
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	BL p_print_int
	BL p_print_ln
	ADD sp, sp, #16
	LDR r0, =0
	POP {pc}
	.ltorg
p_throw_overflow_error:
	LDR r0, =msg_4
	BL p_throw_runtime_error
p_check_divide_by_zero:
	PUSH {lr}
	CMP r1, #0
	LDREQ r0, =msg_5
	BLEQ p_throw_runtime_error
	POP {pc}
p_print_string:
	PUSH {lr}
	LDR r1, [r0]
	ADD r2, r0, #4
	LDR r0, =msg_6
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
p_print_int:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_7
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
p_print_ln:
	PUSH {lr}
	LDR r0, =msg_8
	ADD r0, r0, #4
	BL puts
	MOV r0, #0
	BL fflush
	POP {pc}
p_throw_runtime_error:
	BL p_print_string
	MOV r0, #-1
	BL exit
