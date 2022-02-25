	println "=                                         =" ;
	println "= Please choose the following options:    =" ;
	println "=                                         =" ;
	println "= a: insert an integer                    =" ;
	println "= b: find an integer                      =" ;
	println "= c: count the integers                   =" ;
	println "= d: print all integers                   =" ;
	println "= e: remove an integer                    =" ;
	println "= f: remove all integers                  =" ;
	println "= g: exit                                 =" ;
	println "=                                         =" ;
.data

msg_0:
	.word 0
	.ascii	""
msg_1:
	.word 43
msg_3:
	.word 43
msg_16:
	.word 15
	.ascii	"Your decision: "
msg_17:
	.word 18
	.ascii	"You have entered: "
msg_18:
	.word 36
	.ascii	" which is invalid, please try again."
msg_19:
	.word 18
	.ascii	"You have entered: "
msg_20:
	.word 35
	.ascii	"Please enter an integer to insert: "
msg_21:
	.word 43
	.ascii	"Successfully insert it. The integer is new."
msg_22:
	.word 51
	.ascii	"The integer is already there. No insertion is made."
msg_23:
	.word 33
	.ascii	"Please enter an integer to find: "
msg_24:
	.word 17
	.ascii	"Find the integer."
msg_25:
	.word 25
	.ascii	"The integer is not found."
msg_26:
	.word 24
	.ascii	"There is only 1 integer."
msg_27:
	.word 10
	.ascii	"There are "
msg_28:
	.word 10
	.ascii	" integers."
msg_29:
	.word 23
	.ascii	"Here are the integers: "
msg_30:
	.word 35
	.ascii	"Please enter an integer to remove: "
msg_31:
	.word 29
	.ascii	"The integer has been removed."
msg_32:
	.word 25
	.ascii	"The integer is not found."
msg_33:
	.word 31
	.ascii	"All integers have been removed."
msg_34:
	.word 13
	.ascii	"Goodbye Human"
msg_35:
	.word 23
	.ascii	"Error: unknown choice ("
msg_36:
	.word 1
	.ascii	")"
msg_37:
	.word 44
	.ascii	"ArrayIndexOutOfBoundsError: negative index\n\0"
msg_38:
	.word 45
	.ascii	"ArrayIndexOutOfBoundsError: index too large\n\0"
msg_39:
	.word 83
	.ascii	"OverflowError: the result is too small/large to store in a 4-byte signed-integer.\n\0"
msg_40:
	.word 50
	.ascii	"NullReferenceError: dereference a null reference\n\0"
msg_41:
	.word 50
	.ascii	"NullReferenceError: dereference a null reference\n\0"
msg_42:
	.word 5
	.ascii	"%.*s\0"
msg_43:
	.word 1
	.ascii	"\0"
msg_44:
	.word 45
	.ascii	"DivideByZeroError: divide or modulo by zero\n\0"
msg_45:
	.word 3
	.ascii	"%d\0"
msg_46:
	.word 4
	.ascii	" %c\0"
msg_47:
	.word 3
	.ascii	"%d\0"

.text

.global main
f_init:
	PUSH {lr}
	SUB sp, sp, #8
	LDR r4, [sp, #12]
	LDR r4, [r4]
	STR r4, [sp, #4]
	LDR r4, =0
	STR r4, [sp]
	B L0
L1:
	LDR r4, =0
	ADD r5, sp, #12
	LDR r6, [sp]
	LDR r5, [r5]
	MOV r0, r6
	MOV r1, r5
	BL p_check_array_bounds
	ADD r5, r5, #4
	ADD r5, r5, r6, LSL #2
	STR r4, [r5]
	LDR r4, [sp]
	LDR r6, =1
	ADDS r4, r4, r6
	BLVS p_throw_overflow_error
	STR r4, [sp]
L0:
	LDR r4, [sp]
	LDR r6, [sp, #4]
	CMP r4, r6
	MOVLT r4, #1
	MOVGE r4, #0
	CMP r4, #1
	BEQ L1
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #8
	POP {pc}
	POP {pc}
	.ltorg
f_contain:
	PUSH {lr}
	SUB sp, sp, #8
	LDR r4, [sp, #16]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #16]
	STR r4, [sp, #-4]!
	BL f_calculateIndex
	ADD sp, sp, #8
	MOV r4, r0
	STR r4, [sp, #4]
	LDR r4, [sp, #16]
	STR r4, [sp, #-4]!
	ADD r4, sp, #16
	LDR r5, [sp, #8]
	LDR r4, [r4]
	MOV r0, r5
	MOV r1, r4
	BL p_check_array_bounds
	ADD r4, r4, #4
	ADD r4, r4, r5, LSL #2
	LDR r4, [r4]
	STR r4, [sp, #-4]!
	BL f_findNode
	ADD sp, sp, #8
	MOV r4, r0
	STR r4, [sp]
	LDR r4, [sp]
	LDR r5, =0
	CMP r4, r5
	MOVNE r4, #1
	MOVEQ r4, #0
	MOV r0, r4
	ADD sp, sp, #8
	POP {pc}
	POP {pc}
	.ltorg
f_insertIfNotContain:
	PUSH {lr}
	SUB sp, sp, #8
	LDR r4, [sp, #16]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #16]
	STR r4, [sp, #-4]!
	BL f_calculateIndex
	ADD sp, sp, #8
	MOV r4, r0
	STR r4, [sp, #4]
	LDR r4, [sp, #16]
	STR r4, [sp, #-4]!
	ADD r4, sp, #16
	LDR r5, [sp, #8]
	LDR r4, [r4]
	MOV r0, r5
	MOV r1, r4
	BL p_check_array_bounds
	ADD r4, r4, #4
	ADD r4, r4, r5, LSL #2
	LDR r4, [r4]
	STR r4, [sp, #-4]!
	BL f_findNode
	ADD sp, sp, #8
	MOV r4, r0
	STR r4, [sp]
	LDR r4, [sp]
	LDR r5, =0
	CMP r4, r5
	MOVNE r4, #1
	MOVEQ r4, #0
	CMP r4, #0
	BEQ L2
	MOV r4, #0
	MOV r0, r4
	ADD sp, sp, #8
	POP {pc}
	B L3
L2:
	SUB sp, sp, #4
	LDR r0, =8
	BL malloc
	MOV r4, r0
	LDR r5, [sp, #20]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4]
	ADD r5, sp, #16
	LDR r6, [sp, #8]
	LDR r5, [r5]
	MOV r0, r6
	MOV r1, r5
	BL p_check_array_bounds
	ADD r5, r5, #4
	ADD r5, r5, r6, LSL #2
	LDR r5, [r5]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp]
	LDR r4, [sp]
	ADD r5, sp, #16
	LDR r6, [sp, #8]
	LDR r5, [r5]
	MOV r0, r6
	MOV r1, r5
	BL p_check_array_bounds
	ADD r5, r5, #4
	ADD r5, r5, r6, LSL #2
	STR r4, [r5]
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #12
	POP {pc}
	ADD sp, sp, #4
L3:
	POP {pc}
	.ltorg
f_remove:
	PUSH {lr}
	SUB sp, sp, #8
	LDR r4, [sp, #16]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #16]
	STR r4, [sp, #-4]!
	BL f_calculateIndex
	ADD sp, sp, #8
	MOV r4, r0
	STR r4, [sp, #4]
	LDR r4, [sp, #16]
	STR r4, [sp, #-4]!
	ADD r4, sp, #16
	LDR r5, [sp, #8]
	LDR r4, [r4]
	MOV r0, r5
	MOV r1, r4
	BL p_check_array_bounds
	ADD r4, r4, #4
	ADD r4, r4, r5, LSL #2
	LDR r4, [r4]
	STR r4, [sp, #-4]!
	BL f_findNode
	ADD sp, sp, #8
	MOV r4, r0
	STR r4, [sp]
	LDR r4, [sp]
	LDR r5, =0
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L4
	MOV r4, #0
	MOV r0, r4
	ADD sp, sp, #8
	POP {pc}
	B L5
L4:
	LDR r4, [sp]
	STR r4, [sp, #-4]!
	ADD r4, sp, #16
	LDR r5, [sp, #8]
	LDR r4, [r4]
	MOV r0, r5
	MOV r1, r4
	BL p_check_array_bounds
	ADD r4, r4, #4
	ADD r4, r4, r5, LSL #2
	LDR r4, [r4]
	STR r4, [sp, #-4]!
	BL f_removeNode
	ADD sp, sp, #8
	MOV r4, r0
	ADD r5, sp, #12
	LDR r6, [sp, #4]
	LDR r5, [r5]
	MOV r0, r6
	MOV r1, r5
	BL p_check_array_bounds
	ADD r5, r5, #4
	ADD r5, r5, r6, LSL #2
	STR r4, [r5]
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #8
	POP {pc}
L5:
	POP {pc}
	.ltorg
f_removeAll:
	PUSH {lr}
	SUB sp, sp, #8
	LDR r4, [sp, #12]
	LDR r4, [r4]
	STR r4, [sp, #4]
	LDR r4, =0
	STR r4, [sp]
	B L6
L7:
	SUB sp, sp, #4
	ADD r4, sp, #16
	LDR r5, [sp, #4]
	LDR r4, [r4]
	MOV r0, r5
	MOV r1, r4
	BL p_check_array_bounds
	ADD r4, r4, #4
	ADD r4, r4, r5, LSL #2
	LDR r4, [r4]
	STR r4, [sp]
	B L8
L9:
	SUB sp, sp, #4
	LDR r4, [sp, #4]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp]
	LDR r4, [sp, #4]
	MOV r0, r4
	BL p_free_pair
	LDR r4, [sp]
	STR r4, [sp, #4]
	ADD sp, sp, #4
L8:
	LDR r4, [sp]
	LDR r5, =0
	CMP r4, r5
	MOVNE r4, #1
	MOVEQ r4, #0
	CMP r4, #1
	BEQ L9
	LDR r4, =0
	ADD r5, sp, #16
	LDR r6, [sp, #4]
	LDR r5, [r5]
	MOV r0, r6
	MOV r1, r5
	BL p_check_array_bounds
	ADD r5, r5, #4
	ADD r5, r5, r6, LSL #2
	STR r4, [r5]
	LDR r4, [sp, #4]
	LDR r6, =1
	ADDS r4, r4, r6
	BLVS p_throw_overflow_error
	STR r4, [sp, #4]
	ADD sp, sp, #4
L6:
	LDR r4, [sp]
	LDR r6, [sp, #4]
	CMP r4, r6
	MOVLT r4, #1
	MOVGE r4, #0
	CMP r4, #1
	BEQ L7
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #8
	POP {pc}
	POP {pc}
	.ltorg
f_count:
	PUSH {lr}
	SUB sp, sp, #12
	LDR r4, [sp, #16]
	LDR r4, [r4]
	STR r4, [sp, #8]
	LDR r4, =0
	STR r4, [sp, #4]
	LDR r4, =0
	STR r4, [sp]
	B L10
L11:
	SUB sp, sp, #4
	ADD r4, sp, #20
	LDR r5, [sp, #4]
	LDR r4, [r4]
	MOV r0, r5
	MOV r1, r4
	BL p_check_array_bounds
	ADD r4, r4, #4
	ADD r4, r4, r5, LSL #2
	LDR r4, [r4]
	STR r4, [sp, #-4]!
	BL f_countNodes
	ADD sp, sp, #4
	MOV r4, r0
	STR r4, [sp]
	LDR r4, [sp, #8]
	LDR r5, [sp]
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	STR r4, [sp, #8]
	LDR r4, [sp, #4]
	LDR r5, =1
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	STR r4, [sp, #4]
	ADD sp, sp, #4
L10:
	LDR r4, [sp]
	LDR r5, [sp, #8]
	CMP r4, r5
	MOVLT r4, #1
	MOVGE r4, #0
	CMP r4, #1
	BEQ L11
	LDR r4, [sp, #4]
	MOV r0, r4
	ADD sp, sp, #12
	POP {pc}
	POP {pc}
	.ltorg
f_printAll:
	PUSH {lr}
	SUB sp, sp, #8
	LDR r4, [sp, #12]
	LDR r4, [r4]
	STR r4, [sp, #4]
	LDR r4, =0
	STR r4, [sp]
	B L12
L13:
	SUB sp, sp, #1
	ADD r4, sp, #13
	LDR r5, [sp, #1]
	LDR r4, [r4]
	MOV r0, r5
	MOV r1, r4
	BL p_check_array_bounds
	ADD r4, r4, #4
	ADD r4, r4, r5, LSL #2
	LDR r4, [r4]
	STR r4, [sp, #-4]!
	BL f_printAllNodes
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #1]
	LDR r5, =1
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	STR r4, [sp, #1]
	ADD sp, sp, #1
L12:
	LDR r4, [sp]
	LDR r5, [sp, #4]
	CMP r4, r5
	MOVLT r4, #1
	MOVGE r4, #0
	CMP r4, #1
	BEQ L13
	LDR r4, =msg_0
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #8
	POP {pc}
	POP {pc}
	.ltorg
f_calculateIndex:
	PUSH {lr}
	SUB sp, sp, #4
	LDR r4, [sp, #8]
	LDR r4, [r4]
	STR r4, [sp]
	LDR r4, [sp, #12]
	LDR r5, [sp]
	MOV r0, r4
	MOV r1, r5
	BL p_check_divide_by_zero
	BL __aeabi_idivmod
	MOV r4, r1
	MOV r0, r4
	ADD sp, sp, #4
	POP {pc}
	POP {pc}
	.ltorg
f_findNode:
	PUSH {lr}
	B L14
L15:
	SUB sp, sp, #4
	LDR r4, [sp, #8]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp]
	LDR r4, [sp]
	LDR r5, [sp, #12]
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L16
	LDR r4, [sp, #8]
	MOV r0, r4
	ADD sp, sp, #4
	POP {pc}
	B L17
L16:
	LDR r4, [sp, #8]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #8]
L17:
	ADD sp, sp, #4
L14:
	LDR r4, [sp, #4]
	LDR r5, =0
	CMP r4, r5
	MOVNE r4, #1
	MOVEQ r4, #0
	CMP r4, #1
	BEQ L15
	LDR r4, =0
	MOV r0, r4
	POP {pc}
	POP {pc}
	.ltorg
f_removeNode:
	PUSH {lr}
	LDR r4, [sp, #4]
	LDR r5, =0
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L18
	LDR r4, =0
	MOV r0, r4
	POP {pc}
	B L19
L18:
	LDR r4, [sp, #4]
	LDR r5, [sp, #8]
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L20
	LDR r4, [sp, #4]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #4]
	LDR r4, [sp, #8]
	MOV r0, r4
	BL p_free_pair
	LDR r4, [sp, #4]
	MOV r0, r4
	POP {pc}
	B L21
L20:
	SUB sp, sp, #4
	LDR r4, [sp, #8]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp]
	LDR r4, [sp, #12]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #4]
	STR r4, [sp, #-4]!
	BL f_removeNode
	ADD sp, sp, #8
	MOV r4, r0
	LDR r5, [sp, #8]
	MOV r0, r5
	BL p_check_null_pointer
	LDR r5, [r5, #4]
	STR r4, [r5]
	LDR r4, [sp, #8]
	MOV r0, r4
	ADD sp, sp, #4
	POP {pc}
	ADD sp, sp, #4
L21:
L19:
	POP {pc}
	.ltorg
f_countNodes:
	PUSH {lr}
	SUB sp, sp, #4
	LDR r4, =0
	STR r4, [sp]
	B L22
L23:
	LDR r4, [sp]
	LDR r5, =1
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	STR r4, [sp]
	LDR r4, [sp, #8]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #8]
L22:
	LDR r4, [sp, #8]
	LDR r5, =0
	CMP r4, r5
	MOVNE r4, #1
	MOVEQ r4, #0
	CMP r4, #1
	BEQ L23
	LDR r4, [sp]
	MOV r0, r4
	ADD sp, sp, #4
	POP {pc}
	POP {pc}
	.ltorg
f_printAllNodes:
	PUSH {lr}
	B L24
L25:
	SUB sp, sp, #4
	LDR r4, [sp, #8]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	BL p_print_int
	MOV r4, #' '
	MOV r0, r4
	BL putchar
	LDR r4, [sp, #8]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #8]
	ADD sp, sp, #4
L24:
	LDR r4, [sp, #4]
	LDR r5, =0
	CMP r4, r5
	MOVNE r4, #1
	MOVEQ r4, #0
	CMP r4, #1
	BEQ L25
	MOV r4, #1
	MOV r0, r4
	POP {pc}
	POP {pc}
	.ltorg
f_printMenu:
	PUSH {lr}
	SUB sp, sp, #8
	LDR r4, =msg_1
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_2
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_3
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_4
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_5
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_6
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_7
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_8
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_9
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_10
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_11
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_12
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_13
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_14
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_15
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	MOV r4, #'a'
	STR r4, [sp, #4]
	MOV r4, #'g'
	STR r4, [sp]
	B L26
L27:
	SUB sp, sp, #5
	LDR r4, =msg_16
	MOV r0, r4
	BL p_print_string
	MOV r4, #0
	STRB r4, [sp, #4]
	ADD r4, sp, #4
	MOV r0, r4
	BL p_read_char
	LDRSB r4, [sp, #4]
	STR r4, [sp]
	LDR r4, [sp, #9]
	LDR r5, [sp]
	CMP r4, r5
	MOVLE r4, #1
	MOVGT r4, #0
	LDR r5, [sp]
	LDR r6, [sp, #5]
	CMP r5, r6
	MOVLE r5, #1
	MOVGT r5, #0
	AND r4, r4, r5
	CMP r4, #0
	BEQ L28
	LDRSB r4, [sp, #4]
	MOV r0, r4
	ADD sp, sp, #13
	POP {pc}
	B L29
L28:
	LDR r4, =msg_17
	MOV r0, r4
	BL p_print_string
	LDRSB r4, [sp, #4]
	MOV r0, r4
	BL putchar
	LDR r4, =msg_18
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
L29:
	ADD sp, sp, #5
L26:
	MOV r4, #1
	CMP r4, #1
	BEQ L27
	MOV r4, #0
	MOV r0, r4
	ADD sp, sp, #8
	POP {pc}
	POP {pc}
	.ltorg
f_askForInt:
	PUSH {lr}
	SUB sp, sp, #4
	LDR r4, [sp, #8]
	MOV r0, r4
	BL p_print_string
	LDR r4, =0
	STR r4, [sp]
	ADD r4, sp, #0
	MOV r0, r4
	BL p_read_int
	LDR r4, =msg_19
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp]
	MOV r0, r4
	BL p_print_int
	BL p_print_ln
	LDR r4, [sp]
	MOV r0, r4
	ADD sp, sp, #4
	POP {pc}
	POP {pc}
	.ltorg
f_handleMenuInsert:
	PUSH {lr}
	SUB sp, sp, #5
	LDR r4, =msg_20
	STR r4, [sp, #-4]!
	BL f_askForInt
	ADD sp, sp, #4
	MOV r4, r0
	STR r4, [sp, #1]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #13]
	STR r4, [sp, #-4]!
	BL f_insertIfNotContain
	ADD sp, sp, #8
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	CMP r4, #0
	BEQ L30
	LDR r4, =msg_21
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	B L31
L30:
	LDR r4, =msg_22
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
L31:
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #5
	POP {pc}
	POP {pc}
	.ltorg
f_handleMenuFind:
	PUSH {lr}
	SUB sp, sp, #5
	LDR r4, =msg_23
	STR r4, [sp, #-4]!
	BL f_askForInt
	ADD sp, sp, #4
	MOV r4, r0
	STR r4, [sp, #1]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #13]
	STR r4, [sp, #-4]!
	BL f_contain
	ADD sp, sp, #8
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	CMP r4, #0
	BEQ L32
	LDR r4, =msg_24
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	B L33
L32:
	LDR r4, =msg_25
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
L33:
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #5
	POP {pc}
	POP {pc}
	.ltorg
f_handleMenuCount:
	PUSH {lr}
	SUB sp, sp, #4
	LDR r4, [sp, #8]
	STR r4, [sp, #-4]!
	BL f_count
	ADD sp, sp, #4
	MOV r4, r0
	STR r4, [sp]
	LDR r4, [sp]
	LDR r5, =1
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L34
	LDR r4, =msg_26
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	B L35
L34:
	LDR r4, =msg_27
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp]
	MOV r0, r4
	BL p_print_int
	LDR r4, =msg_28
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
L35:
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #4
	POP {pc}
	POP {pc}
	.ltorg
f_handleMenuPrint:
	PUSH {lr}
	SUB sp, sp, #1
	LDR r4, =msg_29
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	BL f_printAll
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #1
	POP {pc}
	POP {pc}
	.ltorg
f_handleMenuRemove:
	PUSH {lr}
	SUB sp, sp, #5
	LDR r4, =msg_30
	STR r4, [sp, #-4]!
	BL f_askForInt
	ADD sp, sp, #4
	MOV r4, r0
	STR r4, [sp, #1]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #13]
	STR r4, [sp, #-4]!
	BL f_remove
	ADD sp, sp, #8
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	CMP r4, #0
	BEQ L36
	LDR r4, =msg_31
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	B L37
L36:
	LDR r4, =msg_32
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
L37:
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #5
	POP {pc}
	POP {pc}
	.ltorg
f_handleMenuRemoveAll:
	PUSH {lr}
	SUB sp, sp, #1
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	BL f_removeAll
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, =msg_33
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #1
	POP {pc}
	POP {pc}
	.ltorg
main:
	PUSH {lr}
	SUB sp, sp, #6
	LDR r0, =56
	BL malloc
	MOV r4, r0
	LDR r5, =0
	STR r5, [r4, #4]
	LDR r5, =0
	STR r5, [r4, #8]
	LDR r5, =0
	STR r5, [r4, #12]
	LDR r5, =0
	STR r5, [r4, #16]
	LDR r5, =0
	STR r5, [r4, #20]
	LDR r5, =0
	STR r5, [r4, #24]
	LDR r5, =0
	STR r5, [r4, #28]
	LDR r5, =0
	STR r5, [r4, #32]
	LDR r5, =0
	STR r5, [r4, #36]
	LDR r5, =0
	STR r5, [r4, #40]
	LDR r5, =0
	STR r5, [r4, #44]
	LDR r5, =0
	STR r5, [r4, #48]
	LDR r5, =0
	STR r5, [r4, #52]
	LDR r5, =13
	STR r5, [r4]
	STR r4, [sp, #2]
	LDR r4, [sp, #2]
	STR r4, [sp, #-4]!
	BL f_init
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp, #1]
	MOV r4, #1
	STRB r4, [sp]
	B L38
L39:
	SUB sp, sp, #1
	BL f_printMenu
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	MOV r5, #'a'
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L40
	SUB sp, sp, #1
	LDR r4, [sp, #4]
	STR r4, [sp, #-4]!
	BL f_handleMenuInsert
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	ADD sp, sp, #1
	B L41
L40:
	LDRSB r4, [sp]
	MOV r5, #'b'
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L42
	SUB sp, sp, #1
	LDR r4, [sp, #4]
	STR r4, [sp, #-4]!
	BL f_handleMenuFind
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	ADD sp, sp, #1
	B L43
L42:
	LDRSB r4, [sp]
	MOV r5, #'c'
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L44
	SUB sp, sp, #1
	LDR r4, [sp, #4]
	STR r4, [sp, #-4]!
	BL f_handleMenuCount
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	ADD sp, sp, #1
	B L45
L44:
	LDRSB r4, [sp]
	MOV r5, #'d'
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L46
	SUB sp, sp, #1
	LDR r4, [sp, #4]
	STR r4, [sp, #-4]!
	BL f_handleMenuPrint
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	ADD sp, sp, #1
	B L47
L46:
	LDRSB r4, [sp]
	MOV r5, #'e'
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L48
	SUB sp, sp, #1
	LDR r4, [sp, #4]
	STR r4, [sp, #-4]!
	BL f_handleMenuRemove
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	ADD sp, sp, #1
	B L49
L48:
	LDRSB r4, [sp]
	MOV r5, #'f'
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L50
	SUB sp, sp, #1
	LDR r4, [sp, #4]
	STR r4, [sp, #-4]!
	BL f_handleMenuRemoveAll
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	ADD sp, sp, #1
	B L51
L50:
	LDRSB r4, [sp]
	MOV r5, #'g'
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L52
	LDR r4, =msg_34
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	MOV r4, #0
	STRB r4, [sp, #1]
	B L53
L52:
	LDR r4, =msg_35
	MOV r0, r4
	BL p_print_string
	LDRSB r4, [sp]
	MOV r0, r4
	BL putchar
	LDR r4, =msg_36
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =-1
	MOV r0, r4
	BL exit
L53:
L51:
L49:
L47:
L45:
L43:
L41:
	ADD sp, sp, #1
L38:
	LDRSB r4, [sp]
	CMP r4, #1
	BEQ L39
	ADD sp, sp, #6
	LDR r0, =0
	POP {pc}
	.ltorg
p_check_array_bounds:
	PUSH {lr}
	CMP r0, #0
	LDRLT r0, =msg_37
	BLLT p_throw_runtime_error
	LDR r1, [r1]
	CMP r0, r1
	LDRCS r0, =msg_38
	BLCS p_throw_runtime_error
	POP {pc}
p_throw_overflow_error:
	LDR r0, =msg_39
	BL p_throw_runtime_error
p_check_null_pointer:
	PUSH {lr}
	CMP r0, #0
	LDREQ r0, =msg_40
	BLEQ p_throw_runtime_error
	POP {pc}
p_free_pair:
	PUSH {lr}
	CMP r0, #0
	LDREQ r0, =msg_41
	BEQ p_throw_runtime_error
	PUSH {r0}
	LDR r0, [r0]
	BL free
	LDR r0, [sp]
	LDR r0, [r0, #4]
	BL free
	POP {r0}
	BL free
	POP {pc}
p_print_string:
	PUSH {lr}
	LDR r1, [r0]
	ADD r2, r0, #4
	LDR r0, =msg_42
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
p_print_ln:
	PUSH {lr}
	LDR r0, =msg_43
	ADD r0, r0, #4
	BL puts
	MOV r0, #0
	BL fflush
	POP {pc}
p_check_divide_by_zero:
	PUSH {lr}
	CMP r1, #0
	LDREQ r0, =msg_44
	BLEQ p_throw_runtime_error
	POP {pc}
p_print_int:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_45
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
p_read_char:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_46
	ADD r0, r0, #4
	BL scanf
	POP {pc}
p_read_int:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_47
	ADD r0, r0, #4
	BL scanf
	POP {pc}
p_throw_runtime_error:
	BL p_print_string
	MOV r0, #-1
	BL exit
