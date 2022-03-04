.data

msg_0:
	.word 38
	.ascii	"========= Tic Tac Toe ================"
msg_1:
	.word 38
	.ascii	"=  Because we know you want to win   ="
msg_2:
	.word 38
	.ascii	"======================================"
msg_3:
	.word 38
	.ascii	"=                                    ="
msg_4:
	.word 38
	.ascii	"= Who would you like to be?          ="
msg_5:
	.word 38
	.ascii	"=   x  (play first)                  ="
msg_6:
	.word 38
	.ascii	"=   o  (play second)                 ="
msg_7:
	.word 38
	.ascii	"=   q  (quit)                        ="
msg_8:
	.word 38
	.ascii	"=                                    ="
msg_9:
	.word 38
	.ascii	"======================================"
msg_10:
	.word 39
	.ascii	"Which symbol you would like to choose: "
msg_11:
	.word 15
	.ascii	"Goodbye safety."
msg_12:
	.word 16
	.ascii	"Invalid symbol: "
msg_13:
	.word 17
	.ascii	"Please try again."
msg_14:
	.word 17
	.ascii	"You have chosen: "
msg_15:
	.word 6
	.ascii	" 1 2 3"
msg_16:
	.word 1
	.ascii	"1"
msg_17:
	.word 6
	.ascii	" -+-+-"
msg_18:
	.word 1
	.ascii	"2"
msg_19:
	.word 6
	.ascii	" -+-+-"
msg_20:
	.word 1
	.ascii	"3"
msg_21:
	.word 0
	.ascii	""
msg_22:
	.word 0
	.ascii	""
msg_23:
	.word 23
	.ascii	"What is your next move?"
msg_24:
	.word 12
	.ascii	" row (1-3): "
msg_25:
	.word 15
	.ascii	" column (1-3): "
msg_26:
	.word 0
	.ascii	""
msg_27:
	.word 39
	.ascii	"Your move is invalid. Please try again."
msg_28:
	.word 21
	.ascii	"The AI played at row "
msg_29:
	.word 8
	.ascii	" column "
msg_30:
	.word 31
	.ascii	"AI is cleaning up its memory..."
msg_31:
	.word 52
	.ascii	"Internal Error: cannot find the next move for the AI"
msg_32:
	.word 31
	.ascii	"AI is cleaning up its memory..."
msg_33:
	.word 50
	.ascii	"Internal Error: symbol given is neither \'x\' or \'o\'"
msg_34:
	.word 58
	.ascii	"Initialising AI. Please wait, this may take a few minutes."
msg_35:
	.word 9
	.ascii	" has won!"
msg_36:
	.word 10
	.ascii	"Stalemate!"
msg_37:
	.word 5
	.ascii	"%.*s\0"
msg_38:
	.word 1
	.ascii	"\0"
msg_39:
	.word 4
	.ascii	" %c\0"
msg_40:
	.word 50
	.ascii	"NullReferenceError: dereference a null reference\n\0"
msg_41:
	.word 3
	.ascii	"%d\0"
msg_42:
	.word 44
	.ascii	"ArrayIndexOutOfBoundsError: negative index\n\0"
msg_43:
	.word 45
	.ascii	"ArrayIndexOutOfBoundsError: index too large\n\0"
msg_44:
	.word 3
	.ascii	"%d\0"
msg_45:
	.word 50
	.ascii	"NullReferenceError: dereference a null reference\n\0"
msg_46:
	.word 83
	.ascii	"OverflowError: the result is too small/large to store in a 4-byte signed-integer.\n\0"

.text

.global main
f_chooseSymbol:
	PUSH {lr}
	SUB sp, sp, #1
	LDR r4, =msg_0
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
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
	MOV r4, #0
	STRB r4, [sp]
	B L0
L1:
	SUB sp, sp, #1
	LDR r4, =msg_10
	MOV r0, r4
	BL p_print_string
	MOV r4, #0
	STRB r4, [sp]
	ADD r4, sp, #0
	MOV r0, r4
	BL p_read_char
	LDRSB r4, [sp]
	MOV r5, #'x'
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	LDRSB r5, [sp]
	MOV r6, #'X'
	CMP r5, r6
	MOVEQ r5, #1
	MOVNE r5, #0
	ORR r4, r4, r5
	CMP r4, #0
	BEQ L2
	MOV r4, #'x'
	STRB r4, [sp, #1]
	B L3
L2:
	LDRSB r4, [sp]
	MOV r5, #'o'
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	LDRSB r5, [sp]
	MOV r6, #'O'
	CMP r5, r6
	MOVEQ r5, #1
	MOVNE r5, #0
	ORR r4, r4, r5
	CMP r4, #0
	BEQ L4
	MOV r4, #'o'
	STRB r4, [sp, #1]
	B L5
L4:
	LDRSB r4, [sp]
	MOV r5, #'q'
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	LDRSB r5, [sp]
	MOV r6, #'Q'
	CMP r5, r6
	MOVEQ r5, #1
	MOVNE r5, #0
	ORR r4, r4, r5
	CMP r4, #0
	BEQ L6
	LDR r4, =msg_11
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =0
	MOV r0, r4
	BL exit
	B L7
L6:
	LDR r4, =msg_12
	MOV r0, r4
	BL p_print_string
	LDRSB r4, [sp]
	MOV r0, r4
	BL putchar
	BL p_print_ln
	LDR r4, =msg_13
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
L7:
L5:
L3:
	ADD sp, sp, #1
L0:
	LDRSB r4, [sp]
	MOV r5, #0
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #1
	BEQ L1
	LDR r4, =msg_14
	MOV r0, r4
	BL p_print_string
	LDRSB r4, [sp]
	MOV r0, r4
	BL putchar
	BL p_print_ln
	LDRSB r4, [sp]
	MOV r0, r4
	ADD sp, sp, #1
	POP {pc}
	POP {pc}
	.ltorg
f_printBoard:
	PUSH {lr}
	SUB sp, sp, #17
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #13]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, =msg_15
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_16
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	BL f_printRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, =msg_17
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_18
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	BL f_printRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, =msg_19
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_20
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	BL f_printRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, =msg_21
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #17
	POP {pc}
	POP {pc}
	.ltorg
f_printRow:
	PUSH {lr}
	SUB sp, sp, #8
	LDR r4, [sp, #12]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #4]
	LDR r4, [sp, #4]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDRSB r4, [r4]
	STRB r4, [sp, #3]
	LDR r4, [sp, #4]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDRSB r4, [r4]
	STRB r4, [sp, #2]
	LDR r4, [sp, #12]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDRSB r4, [r4]
	STRB r4, [sp, #1]
	LDRSB r4, [sp, #3]
	STRB r4, [sp, #-1]!
	BL f_printCell
	ADD sp, sp, #1
	MOV r4, r0
	STRB r4, [sp]
	MOV r4, #'|'
	MOV r0, r4
	BL putchar
	LDRSB r4, [sp, #2]
	STRB r4, [sp, #-1]!
	BL f_printCell
	ADD sp, sp, #1
	MOV r4, r0
	STRB r4, [sp]
	MOV r4, #'|'
	MOV r0, r4
	BL putchar
	LDRSB r4, [sp, #1]
	STRB r4, [sp, #-1]!
	BL f_printCell
	ADD sp, sp, #1
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, =msg_22
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #8
	POP {pc}
	POP {pc}
	.ltorg
f_printCell:
	PUSH {lr}
	LDRSB r4, [sp, #4]
	MOV r5, #0
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L8
	MOV r4, #' '
	MOV r0, r4
	BL putchar
	B L9
L8:
	LDRSB r4, [sp, #4]
	MOV r0, r4
	BL putchar
L9:
	MOV r4, #1
	MOV r0, r4
	POP {pc}
	POP {pc}
	.ltorg
f_askForAMoveHuman:
	PUSH {lr}
	SUB sp, sp, #9
	MOV r4, #0
	STRB r4, [sp, #8]
	LDR r4, =0
	STR r4, [sp, #4]
	LDR r4, =0
	STR r4, [sp]
	B L10
L11:
	LDR r4, =msg_23
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_24
	MOV r0, r4
	BL p_print_string
	ADD r4, sp, #4
	MOV r0, r4
	BL p_read_int
	LDR r4, =msg_25
	MOV r0, r4
	BL p_print_string
	ADD r4, sp, #0
	MOV r0, r4
	BL p_read_int
	LDR r4, [sp]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #8]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #21]
	STR r4, [sp, #-4]!
	BL f_validateMove
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp, #8]
	LDRSB r4, [sp, #8]
	CMP r4, #0
	BEQ L12
	LDR r4, =msg_26
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, [sp, #4]
	ADD r5, sp, #17
	LDR r6, =0
	LDR r5, [r5]
	MOV r0, r6
	MOV r1, r5
	BL p_check_array_bounds
	ADD r5, r5, #4
	ADD r5, r5, r6, LSL #2
	STR r4, [r5]
	LDR r4, [sp]
	ADD r6, sp, #17
	LDR r7, =1
	LDR r6, [r6]
	MOV r0, r7
	MOV r1, r6
	BL p_check_array_bounds
	ADD r6, r6, #4
	ADD r6, r6, r7, LSL #2
	STR r4, [r6]
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #9
	POP {pc}
	B L13
L12:
	LDR r4, =msg_27
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
L13:
L10:
	LDRSB r4, [sp, #8]
	EOR r4, r4, #1
	CMP r4, #1
	BEQ L11
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #9
	POP {pc}
	POP {pc}
	.ltorg
f_validateMove:
	PUSH {lr}
	LDR r4, =1
	LDR r5, [sp, #8]
	CMP r4, r5
	MOVLE r4, #1
	MOVGT r4, #0
	LDR r5, [sp, #8]
	LDR r6, =3
	CMP r5, r6
	MOVLE r5, #1
	MOVGT r5, #0
	AND r4, r4, r5
	LDR r5, =1
	LDR r6, [sp, #12]
	CMP r5, r6
	MOVLE r5, #1
	MOVGT r5, #0
	AND r4, r4, r5
	LDR r5, [sp, #12]
	LDR r6, =3
	CMP r5, r6
	MOVLE r5, #1
	MOVGT r5, #0
	AND r4, r4, r5
	CMP r4, #0
	BEQ L14
	SUB sp, sp, #1
	LDR r4, [sp, #13]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #13]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #13]
	STR r4, [sp, #-4]!
	BL f_symbolAt
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	MOV r5, #0
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	MOV r0, r4
	ADD sp, sp, #1
	POP {pc}
	ADD sp, sp, #1
	B L15
L14:
	MOV r4, #0
	MOV r0, r4
	POP {pc}
L15:
	POP {pc}
	.ltorg
f_notifyMoveHuman:
	PUSH {lr}
	LDR r4, =msg_28
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #10]
	MOV r0, r4
	BL p_print_int
	LDR r4, =msg_29
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #14]
	MOV r0, r4
	BL p_print_int
	BL p_print_ln
	MOV r4, #1
	MOV r0, r4
	POP {pc}
	POP {pc}
	.ltorg
f_initAI:
	PUSH {lr}
	SUB sp, sp, #16
	LDR r0, =8
	BL malloc
	MOV r4, r0
	LDRSB r5, [sp, #20]
	LDR r0, =1
	BL malloc
	STRB r5, [r0]
	STR r0, [r4]
	LDR r5, =0
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp, #12]
	LDRSB r4, [sp, #20]
	STRB r4, [sp, #-1]!
	BL f_generateAllPossibleStates
	ADD sp, sp, #1
	MOV r4, r0
	STR r4, [sp, #8]
	MOV r4, #'x'
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #21]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #10]
	STR r4, [sp, #-4]!
	BL f_setValuesForAllStates
	ADD sp, sp, #6
	MOV r4, r0
	STR r4, [sp, #4]
	LDR r0, =8
	BL malloc
	MOV r4, r0
	LDR r5, [sp, #12]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4]
	LDR r5, [sp, #8]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	ADD sp, sp, #16
	POP {pc}
	POP {pc}
	.ltorg
f_generateAllPossibleStates:
	PUSH {lr}
	SUB sp, sp, #8
	BL f_allocateNewBoard
	MOV r4, r0
	STR r4, [sp, #4]
	LDR r4, [sp, #4]
	STR r4, [sp, #-4]!
	BL f_convertFromBoardToState
	ADD sp, sp, #4
	MOV r4, r0
	STR r4, [sp]
	MOV r4, #'x'
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	BL f_generateNextStates
	ADD sp, sp, #5
	MOV r4, r0
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	ADD sp, sp, #8
	POP {pc}
	POP {pc}
	.ltorg
f_convertFromBoardToState:
	PUSH {lr}
	SUB sp, sp, #12
	BL f_generateEmptyPointerBoard
	MOV r4, r0
	STR r4, [sp, #8]
	LDR r0, =8
	BL malloc
	MOV r4, r0
	LDR r5, [sp, #16]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4]
	LDR r5, [sp, #8]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp, #4]
	LDR r0, =8
	BL malloc
	MOV r4, r0
	LDR r5, [sp, #4]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4]
	LDR r5, =0
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	ADD sp, sp, #12
	POP {pc}
	POP {pc}
	.ltorg
f_generateEmptyPointerBoard:
	PUSH {lr}
	SUB sp, sp, #20
	BL f_generateEmptyPointerRow
	MOV r4, r0
	STR r4, [sp, #16]
	BL f_generateEmptyPointerRow
	MOV r4, r0
	STR r4, [sp, #12]
	BL f_generateEmptyPointerRow
	MOV r4, r0
	STR r4, [sp, #8]
	LDR r0, =8
	BL malloc
	MOV r4, r0
	LDR r5, [sp, #16]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4]
	LDR r5, [sp, #12]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp, #4]
	LDR r0, =8
	BL malloc
	MOV r4, r0
	LDR r5, [sp, #4]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4]
	LDR r5, [sp, #8]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	ADD sp, sp, #20
	POP {pc}
	POP {pc}
	.ltorg
f_generateEmptyPointerRow:
	PUSH {lr}
	SUB sp, sp, #8
	LDR r0, =8
	BL malloc
	MOV r4, r0
	LDR r5, =0
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4]
	LDR r5, =0
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp, #4]
	LDR r0, =8
	BL malloc
	MOV r4, r0
	LDR r5, [sp, #4]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4]
	LDR r5, =0
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	ADD sp, sp, #8
	POP {pc}
	POP {pc}
	.ltorg
f_generateNextStates:
	PUSH {lr}
	SUB sp, sp, #14
	LDR r4, [sp, #18]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #10]
	LDR r4, [sp, #10]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #6]
	LDR r4, [sp, #10]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #2]
	LDRSB r4, [sp, #22]
	STRB r4, [sp, #-1]!
	BL f_oppositeSymbol
	ADD sp, sp, #1
	MOV r4, r0
	STRB r4, [sp, #1]
	LDRSB r4, [sp, #1]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #7]
	STR r4, [sp, #-4]!
	BL f_hasWon
	ADD sp, sp, #5
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	CMP r4, #0
	BEQ L16
	LDR r4, [sp, #18]
	MOV r0, r4
	ADD sp, sp, #14
	POP {pc}
	B L17
L16:
	SUB sp, sp, #1
	LDRSB r4, [sp, #23]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #4]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #12]
	STR r4, [sp, #-4]!
	BL f_generateNextStatesBoard
	ADD sp, sp, #9
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #19]
	MOV r0, r4
	ADD sp, sp, #15
	POP {pc}
	ADD sp, sp, #1
L17:
	POP {pc}
	.ltorg
f_generateNextStatesBoard:
	PUSH {lr}
	SUB sp, sp, #33
	LDR r4, [sp, #37]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #29]
	LDR r4, [sp, #29]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #25]
	LDR r4, [sp, #29]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #21]
	LDR r4, [sp, #37]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #17]
	LDR r4, [sp, #41]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #13]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #41]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, =1
	STR r4, [sp, #-4]!
	LDRSB r4, [sp, #49]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #14]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #34]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #50]
	STR r4, [sp, #-4]!
	BL f_generateNextStatesRow
	ADD sp, sp, #17
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, =2
	STR r4, [sp, #-4]!
	LDRSB r4, [sp, #49]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #10]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #30]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #50]
	STR r4, [sp, #-4]!
	BL f_generateNextStatesRow
	ADD sp, sp, #17
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, =3
	STR r4, [sp, #-4]!
	LDRSB r4, [sp, #49]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #6]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #26]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #50]
	STR r4, [sp, #-4]!
	BL f_generateNextStatesRow
	ADD sp, sp, #17
	MOV r4, r0
	STRB r4, [sp]
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #33
	POP {pc}
	POP {pc}
	.ltorg
f_generateNextStatesRow:
	PUSH {lr}
	SUB sp, sp, #11
	LDR r4, [sp, #19]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #7]
	LDR r4, [sp, #7]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDRSB r4, [r4]
	STRB r4, [sp, #6]
	LDR r4, [sp, #7]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDRSB r4, [r4]
	STRB r4, [sp, #5]
	LDR r4, [sp, #19]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDRSB r4, [r4]
	STRB r4, [sp, #4]
	LDR r4, [sp, #23]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp]
	LDR r4, =1
	STR r4, [sp, #-4]!
	LDR r4, [sp, #32]
	STR r4, [sp, #-4]!
	LDRSB r4, [sp, #35]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #15]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #25]
	STR r4, [sp, #-4]!
	BL f_generateNextStatesCell
	ADD sp, sp, #14
	MOV r4, r0
	LDR r5, [sp]
	MOV r0, r5
	BL p_check_null_pointer
	LDR r5, [r5]
	STR r4, [r5]
	LDR r4, =2
	STR r4, [sp, #-4]!
	LDR r4, [sp, #32]
	STR r4, [sp, #-4]!
	LDRSB r4, [sp, #35]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #14]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #25]
	STR r4, [sp, #-4]!
	BL f_generateNextStatesCell
	ADD sp, sp, #14
	MOV r4, r0
	LDR r5, [sp]
	MOV r0, r5
	BL p_check_null_pointer
	LDR r5, [r5, #4]
	STR r4, [r5]
	LDR r4, =3
	STR r4, [sp, #-4]!
	LDR r4, [sp, #32]
	STR r4, [sp, #-4]!
	LDRSB r4, [sp, #35]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #13]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #25]
	STR r4, [sp, #-4]!
	BL f_generateNextStatesCell
	ADD sp, sp, #14
	MOV r4, r0
	LDR r5, [sp, #23]
	MOV r0, r5
	BL p_check_null_pointer
	LDR r5, [r5, #4]
	STR r4, [r5]
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #11
	POP {pc}
	POP {pc}
	.ltorg
f_generateNextStatesCell:
	PUSH {lr}
	LDRSB r4, [sp, #8]
	MOV r5, #0
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L18
	SUB sp, sp, #10
	LDR r4, [sp, #14]
	STR r4, [sp, #-4]!
	BL f_cloneBoard
	ADD sp, sp, #4
	MOV r4, r0
	STR r4, [sp, #6]
	LDR r4, [sp, #24]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #24]
	STR r4, [sp, #-4]!
	LDRSB r4, [sp, #27]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #15]
	STR r4, [sp, #-4]!
	BL f_placeMove
	ADD sp, sp, #13
	MOV r4, r0
	STRB r4, [sp, #5]
	LDR r4, [sp, #6]
	STR r4, [sp, #-4]!
	BL f_convertFromBoardToState
	ADD sp, sp, #4
	MOV r4, r0
	STR r4, [sp, #1]
	LDRSB r4, [sp, #19]
	STRB r4, [sp, #-1]!
	BL f_oppositeSymbol
	ADD sp, sp, #1
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #2]
	STR r4, [sp, #-4]!
	BL f_generateNextStates
	ADD sp, sp, #5
	MOV r4, r0
	STR r4, [sp, #1]
	LDR r4, [sp, #1]
	MOV r0, r4
	ADD sp, sp, #10
	POP {pc}
	ADD sp, sp, #10
	B L19
L18:
	LDR r4, =0
	MOV r0, r4
	POP {pc}
L19:
	POP {pc}
	.ltorg
f_cloneBoard:
	PUSH {lr}
	SUB sp, sp, #5
	BL f_allocateNewBoard
	MOV r4, r0
	STR r4, [sp, #1]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #13]
	STR r4, [sp, #-4]!
	BL f_copyBoard
	ADD sp, sp, #8
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #1]
	MOV r0, r4
	ADD sp, sp, #5
	POP {pc}
	POP {pc}
	.ltorg
f_copyBoard:
	PUSH {lr}
	SUB sp, sp, #33
	LDR r4, [sp, #37]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #29]
	LDR r4, [sp, #29]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #25]
	LDR r4, [sp, #29]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #21]
	LDR r4, [sp, #37]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #17]
	LDR r4, [sp, #41]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #13]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #41]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #29]
	STR r4, [sp, #-4]!
	BL f_copyRow
	ADD sp, sp, #8
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #25]
	STR r4, [sp, #-4]!
	BL f_copyRow
	ADD sp, sp, #8
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #21]
	STR r4, [sp, #-4]!
	BL f_copyRow
	ADD sp, sp, #8
	MOV r4, r0
	STRB r4, [sp]
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #33
	POP {pc}
	POP {pc}
	.ltorg
f_copyRow:
	PUSH {lr}
	SUB sp, sp, #8
	LDR r4, [sp, #12]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #4]
	LDR r4, [sp, #16]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp]
	LDR r4, [sp, #4]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDRSB r4, [r4]
	LDR r5, [sp]
	MOV r0, r5
	BL p_check_null_pointer
	LDR r5, [r5]
	STRB r4, [r5]
	LDR r4, [sp, #4]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDRSB r4, [r4]
	LDR r5, [sp]
	MOV r0, r5
	BL p_check_null_pointer
	LDR r5, [r5, #4]
	STRB r4, [r5]
	LDR r4, [sp, #12]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDRSB r4, [r4]
	LDR r5, [sp, #16]
	MOV r0, r5
	BL p_check_null_pointer
	LDR r5, [r5, #4]
	STRB r4, [r5]
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #8
	POP {pc}
	POP {pc}
	.ltorg
f_setValuesForAllStates:
	PUSH {lr}
	SUB sp, sp, #4
	LDR r4, =0
	STR r4, [sp]
	LDR r4, [sp, #8]
	LDR r5, =0
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L20
	LDRSB r4, [sp, #13]
	LDRSB r5, [sp, #12]
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L22
	LDR r4, =101
	STR r4, [sp]
	B L23
L22:
	LDR r4, =-101
	STR r4, [sp]
L23:
	B L21
L20:
	SUB sp, sp, #14
	LDR r4, [sp, #22]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #10]
	LDR r4, [sp, #10]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #6]
	LDR r4, [sp, #10]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #2]
	LDRSB r4, [sp, #27]
	STRB r4, [sp, #-1]!
	BL f_oppositeSymbol
	ADD sp, sp, #1
	MOV r4, r0
	STRB r4, [sp, #1]
	LDRSB r4, [sp, #1]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #7]
	STR r4, [sp, #-4]!
	BL f_hasWon
	ADD sp, sp, #5
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	CMP r4, #0
	BEQ L24
	LDRSB r4, [sp, #1]
	LDRSB r5, [sp, #26]
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L26
	LDR r4, =100
	STR r4, [sp, #14]
	B L27
L26:
	LDR r4, =-100
	STR r4, [sp, #14]
L27:
	B L25
L24:
	SUB sp, sp, #1
	LDR r4, [sp, #7]
	STR r4, [sp, #-4]!
	BL f_containEmptyCell
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	CMP r4, #0
	BEQ L28
	LDRSB r4, [sp, #2]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #28]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	BL f_calculateValuesFromNextStates
	ADD sp, sp, #6
	MOV r4, r0
	STR r4, [sp, #15]
	LDR r4, [sp, #15]
	LDR r5, =100
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L30
	LDR r4, =90
	STR r4, [sp, #15]
	B L31
L30:
L31:
	B L29
L28:
	LDR r4, =0
	STR r4, [sp, #15]
L29:
	ADD sp, sp, #1
L25:
	LDR r4, [sp, #14]
	LDR r5, [sp, #22]
	MOV r0, r5
	BL p_check_null_pointer
	LDR r5, [r5, #4]
	STR r4, [r5]
	ADD sp, sp, #14
L21:
	LDR r4, [sp]
	MOV r0, r4
	ADD sp, sp, #4
	POP {pc}
	POP {pc}
	.ltorg
f_calculateValuesFromNextStates:
	PUSH {lr}
	SUB sp, sp, #32
	LDR r4, [sp, #36]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #28]
	LDR r4, [sp, #28]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #24]
	LDR r4, [sp, #28]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #20]
	LDR r4, [sp, #36]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #16]
	LDRSB r4, [sp, #41]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #41]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #26]
	STR r4, [sp, #-4]!
	BL f_calculateValuesFromNextStatesRow
	ADD sp, sp, #6
	MOV r4, r0
	STR r4, [sp, #12]
	LDRSB r4, [sp, #41]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #41]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #22]
	STR r4, [sp, #-4]!
	BL f_calculateValuesFromNextStatesRow
	ADD sp, sp, #6
	MOV r4, r0
	STR r4, [sp, #8]
	LDRSB r4, [sp, #41]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #41]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #18]
	STR r4, [sp, #-4]!
	BL f_calculateValuesFromNextStatesRow
	ADD sp, sp, #6
	MOV r4, r0
	STR r4, [sp, #4]
	LDR r4, [sp, #4]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #12]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #20]
	STR r4, [sp, #-4]!
	LDRSB r4, [sp, #53]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #53]
	STRB r4, [sp, #-1]!
	BL f_combineValue
	ADD sp, sp, #14
	MOV r4, r0
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	ADD sp, sp, #32
	POP {pc}
	POP {pc}
	.ltorg
f_calculateValuesFromNextStatesRow:
	PUSH {lr}
	SUB sp, sp, #32
	LDR r4, [sp, #36]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #28]
	LDR r4, [sp, #28]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #24]
	LDR r4, [sp, #28]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #20]
	LDR r4, [sp, #36]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #16]
	LDRSB r4, [sp, #41]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #41]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #26]
	STR r4, [sp, #-4]!
	BL f_setValuesForAllStates
	ADD sp, sp, #6
	MOV r4, r0
	STR r4, [sp, #12]
	LDRSB r4, [sp, #41]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #41]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #22]
	STR r4, [sp, #-4]!
	BL f_setValuesForAllStates
	ADD sp, sp, #6
	MOV r4, r0
	STR r4, [sp, #8]
	LDRSB r4, [sp, #41]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #41]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #18]
	STR r4, [sp, #-4]!
	BL f_setValuesForAllStates
	ADD sp, sp, #6
	MOV r4, r0
	STR r4, [sp, #4]
	LDR r4, [sp, #4]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #12]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #20]
	STR r4, [sp, #-4]!
	LDRSB r4, [sp, #53]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #53]
	STRB r4, [sp, #-1]!
	BL f_combineValue
	ADD sp, sp, #14
	MOV r4, r0
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	ADD sp, sp, #32
	POP {pc}
	POP {pc}
	.ltorg
f_combineValue:
	PUSH {lr}
	SUB sp, sp, #4
	LDR r4, =0
	STR r4, [sp]
	LDRSB r4, [sp, #8]
	LDRSB r5, [sp, #9]
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L32
	LDR r4, [sp, #18]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #18]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #18]
	STR r4, [sp, #-4]!
	BL f_min3
	ADD sp, sp, #12
	MOV r4, r0
	STR r4, [sp]
	B L33
L32:
	LDR r4, [sp, #18]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #18]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #18]
	STR r4, [sp, #-4]!
	BL f_max3
	ADD sp, sp, #12
	MOV r4, r0
	STR r4, [sp]
L33:
	LDR r4, [sp]
	MOV r0, r4
	ADD sp, sp, #4
	POP {pc}
	POP {pc}
	.ltorg
f_min3:
	PUSH {lr}
	LDR r4, [sp, #4]
	LDR r5, [sp, #8]
	CMP r4, r5
	MOVLT r4, #1
	MOVGE r4, #0
	CMP r4, #0
	BEQ L34
	LDR r4, [sp, #4]
	LDR r5, [sp, #12]
	CMP r4, r5
	MOVLT r4, #1
	MOVGE r4, #0
	CMP r4, #0
	BEQ L36
	LDR r4, [sp, #4]
	MOV r0, r4
	POP {pc}
	B L37
L36:
	LDR r4, [sp, #12]
	MOV r0, r4
	POP {pc}
L37:
	B L35
L34:
	LDR r4, [sp, #8]
	LDR r5, [sp, #12]
	CMP r4, r5
	MOVLT r4, #1
	MOVGE r4, #0
	CMP r4, #0
	BEQ L38
	LDR r4, [sp, #8]
	MOV r0, r4
	POP {pc}
	B L39
L38:
	LDR r4, [sp, #12]
	MOV r0, r4
	POP {pc}
L39:
L35:
	POP {pc}
	.ltorg
f_max3:
	PUSH {lr}
	LDR r4, [sp, #4]
	LDR r5, [sp, #8]
	CMP r4, r5
	MOVGT r4, #1
	MOVLE r4, #0
	CMP r4, #0
	BEQ L40
	LDR r4, [sp, #4]
	LDR r5, [sp, #12]
	CMP r4, r5
	MOVGT r4, #1
	MOVLE r4, #0
	CMP r4, #0
	BEQ L42
	LDR r4, [sp, #4]
	MOV r0, r4
	POP {pc}
	B L43
L42:
	LDR r4, [sp, #12]
	MOV r0, r4
	POP {pc}
L43:
	B L41
L40:
	LDR r4, [sp, #8]
	LDR r5, [sp, #12]
	CMP r4, r5
	MOVGT r4, #1
	MOVLE r4, #0
	CMP r4, #0
	BEQ L44
	LDR r4, [sp, #8]
	MOV r0, r4
	POP {pc}
	B L45
L44:
	LDR r4, [sp, #12]
	MOV r0, r4
	POP {pc}
L45:
L41:
	POP {pc}
	.ltorg
f_destroyAI:
	PUSH {lr}
	SUB sp, sp, #9
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	BL f_deleteStateTreeRecursively
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #5]
	MOV r0, r4
	BL p_free_pair
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_free_pair
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #9
	POP {pc}
	POP {pc}
	.ltorg
f_askForAMoveAI:
	PUSH {lr}
	SUB sp, sp, #21
	LDR r4, [sp, #31]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #17]
	LDR r4, [sp, #31]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #13]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #9]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #35]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #13]
	STR r4, [sp, #-4]!
	BL f_findTheBestMove
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, =msg_30
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	ADD r4, sp, #35
	LDR r5, =1
	LDR r4, [r4]
	MOV r0, r5
	MOV r1, r4
	BL p_check_array_bounds
	ADD r4, r4, #4
	ADD r4, r4, r5, LSL #2
	LDR r4, [r4]
	STR r4, [sp, #-4]!
	ADD r4, sp, #39
	LDR r5, =0
	LDR r4, [r4]
	MOV r0, r5
	MOV r1, r4
	BL p_check_array_bounds
	ADD r4, r4, #4
	ADD r4, r4, r5, LSL #2
	LDR r4, [r4]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #13]
	STR r4, [sp, #-4]!
	BL f_deleteAllOtherChildren
	ADD sp, sp, #12
	MOV r4, r0
	LDR r5, [sp, #31]
	MOV r0, r5
	BL p_check_null_pointer
	LDR r5, [r5, #4]
	STR r4, [r5]
	LDR r4, [sp, #13]
	STR r4, [sp, #-4]!
	BL f_deleteThisStateOnly
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #21
	POP {pc}
	POP {pc}
	.ltorg
f_findTheBestMove:
	PUSH {lr}
	SUB sp, sp, #1
	LDR r4, [sp, #9]
	LDR r5, =90
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L46
	SUB sp, sp, #1
	LDR r4, [sp, #14]
	STR r4, [sp, #-4]!
	LDR r4, =100
	STR r4, [sp, #-4]!
	LDR r4, [sp, #14]
	STR r4, [sp, #-4]!
	BL f_findMoveWithGivenValue
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	CMP r4, #0
	BEQ L48
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #2
	POP {pc}
	B L49
L48:
L49:
	ADD sp, sp, #1
	B L47
L46:
L47:
	LDR r4, [sp, #13]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #13]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #13]
	STR r4, [sp, #-4]!
	BL f_findMoveWithGivenValue
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	CMP r4, #0
	BEQ L50
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #1
	POP {pc}
	B L51
L50:
	LDR r4, =msg_31
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =-1
	MOV r0, r4
	BL exit
L51:
	POP {pc}
	.ltorg
f_findMoveWithGivenValue:
	PUSH {lr}
	SUB sp, sp, #17
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #13]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #29]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #29]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #17]
	STR r4, [sp, #-4]!
	BL f_findMoveWithGivenValueRow
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	CMP r4, #0
	BEQ L52
	LDR r4, =1
	ADD r5, sp, #29
	LDR r6, =0
	LDR r5, [r5]
	MOV r0, r6
	MOV r1, r5
	BL p_check_array_bounds
	ADD r5, r5, #4
	ADD r5, r5, r6, LSL #2
	STR r4, [r5]
	B L53
L52:
	LDR r4, [sp, #29]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #29]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #13]
	STR r4, [sp, #-4]!
	BL f_findMoveWithGivenValueRow
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	CMP r4, #0
	BEQ L54
	LDR r4, =2
	ADD r6, sp, #29
	LDR r7, =0
	LDR r6, [r6]
	MOV r0, r7
	MOV r1, r6
	BL p_check_array_bounds
	ADD r6, r6, #4
	ADD r6, r6, r7, LSL #2
	STR r4, [r6]
	B L55
L54:
	LDR r4, [sp, #29]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #29]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	BL f_findMoveWithGivenValueRow
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	CMP r4, #0
	BEQ L56
	LDR r4, =3
	ADD r7, sp, #29
	LDR r8, =0
	LDR r7, [r7]
	MOV r0, r8
	MOV r1, r7
	BL p_check_array_bounds
	ADD r7, r7, #4
	ADD r7, r7, r8, LSL #2
	STR r4, [r7]
	B L57
L56:
	MOV r4, #0
	MOV r0, r4
	ADD sp, sp, #17
	POP {pc}
L57:
L55:
L53:
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #17
	POP {pc}
	POP {pc}
	.ltorg
f_findMoveWithGivenValueRow:
	PUSH {lr}
	SUB sp, sp, #17
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #13]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #25]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #13]
	STR r4, [sp, #-4]!
	BL f_hasGivenStateValue
	ADD sp, sp, #8
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	CMP r4, #0
	BEQ L58
	LDR r4, =1
	ADD r5, sp, #29
	LDR r6, =1
	LDR r5, [r5]
	MOV r0, r6
	MOV r1, r5
	BL p_check_array_bounds
	ADD r5, r5, #4
	ADD r5, r5, r6, LSL #2
	STR r4, [r5]
	B L59
L58:
	LDR r4, [sp, #25]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	BL f_hasGivenStateValue
	ADD sp, sp, #8
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	CMP r4, #0
	BEQ L60
	LDR r4, =2
	ADD r6, sp, #29
	LDR r7, =1
	LDR r6, [r6]
	MOV r0, r7
	MOV r1, r6
	BL p_check_array_bounds
	ADD r6, r6, #4
	ADD r6, r6, r7, LSL #2
	STR r4, [r6]
	B L61
L60:
	LDR r4, [sp, #25]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	BL f_hasGivenStateValue
	ADD sp, sp, #8
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	CMP r4, #0
	BEQ L62
	LDR r4, =3
	ADD r7, sp, #29
	LDR r8, =1
	LDR r7, [r7]
	MOV r0, r8
	MOV r1, r7
	BL p_check_array_bounds
	ADD r7, r7, #4
	ADD r7, r7, r8, LSL #2
	STR r4, [r7]
	B L63
L62:
	MOV r4, #0
	MOV r0, r4
	ADD sp, sp, #17
	POP {pc}
L63:
L61:
L59:
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #17
	POP {pc}
	POP {pc}
	.ltorg
f_hasGivenStateValue:
	PUSH {lr}
	LDR r4, [sp, #4]
	LDR r5, =0
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L64
	MOV r4, #0
	MOV r0, r4
	POP {pc}
	B L65
L64:
	SUB sp, sp, #4
	LDR r4, [sp, #8]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp]
	LDR r4, [sp]
	LDR r5, [sp, #12]
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	MOV r0, r4
	ADD sp, sp, #4
	POP {pc}
	ADD sp, sp, #4
L65:
	POP {pc}
	.ltorg
f_notifyMoveAI:
	PUSH {lr}
	SUB sp, sp, #13
	LDR r4, [sp, #23]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #9]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #5]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, =msg_32
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, [sp, #31]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #31]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	BL f_deleteAllOtherChildren
	ADD sp, sp, #12
	MOV r4, r0
	LDR r5, [sp, #23]
	MOV r0, r5
	BL p_check_null_pointer
	LDR r5, [r5, #4]
	STR r4, [r5]
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	BL f_deleteThisStateOnly
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #13
	POP {pc}
	POP {pc}
	.ltorg
f_deleteAllOtherChildren:
	PUSH {lr}
	SUB sp, sp, #33
	LDR r4, [sp, #37]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #29]
	LDR r4, [sp, #29]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #25]
	LDR r4, [sp, #29]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #21]
	LDR r4, [sp, #37]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #17]
	LDR r4, =0
	STR r4, [sp, #13]
	LDR r4, =0
	STR r4, [sp, #9]
	LDR r4, =0
	STR r4, [sp, #5]
	LDR r4, [sp, #41]
	LDR r5, =1
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L66
	LDR r4, [sp, #25]
	STR r4, [sp, #13]
	LDR r4, [sp, #21]
	STR r4, [sp, #9]
	LDR r4, [sp, #17]
	STR r4, [sp, #5]
	B L67
L66:
	LDR r4, [sp, #25]
	STR r4, [sp, #9]
	LDR r4, [sp, #41]
	LDR r5, =2
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L68
	LDR r4, [sp, #21]
	STR r4, [sp, #13]
	LDR r4, [sp, #17]
	STR r4, [sp, #5]
	B L69
L68:
	LDR r4, [sp, #17]
	STR r4, [sp, #13]
	LDR r4, [sp, #21]
	STR r4, [sp, #5]
L69:
L67:
	LDR r4, [sp, #45]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #17]
	STR r4, [sp, #-4]!
	BL f_deleteAllOtherChildrenRow
	ADD sp, sp, #8
	MOV r4, r0
	STR r4, [sp, #1]
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	BL f_deleteChildrenStateRecursivelyRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	BL f_deleteChildrenStateRecursivelyRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #1]
	MOV r0, r4
	ADD sp, sp, #33
	POP {pc}
	POP {pc}
	.ltorg
f_deleteAllOtherChildrenRow:
	PUSH {lr}
	SUB sp, sp, #29
	LDR r4, [sp, #33]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #25]
	LDR r4, [sp, #25]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #21]
	LDR r4, [sp, #25]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #17]
	LDR r4, [sp, #33]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #13]
	LDR r4, =0
	STR r4, [sp, #9]
	LDR r4, =0
	STR r4, [sp, #5]
	LDR r4, =0
	STR r4, [sp, #1]
	LDR r4, [sp, #37]
	LDR r5, =1
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L70
	LDR r4, [sp, #21]
	STR r4, [sp, #9]
	LDR r4, [sp, #17]
	STR r4, [sp, #5]
	LDR r4, [sp, #13]
	STR r4, [sp, #1]
	B L71
L70:
	LDR r4, [sp, #21]
	STR r4, [sp, #5]
	LDR r4, [sp, #37]
	LDR r5, =2
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L72
	LDR r4, [sp, #17]
	STR r4, [sp, #9]
	LDR r4, [sp, #13]
	STR r4, [sp, #1]
	B L73
L72:
	LDR r4, [sp, #13]
	STR r4, [sp, #9]
	LDR r4, [sp, #17]
	STR r4, [sp, #1]
L73:
L71:
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	BL f_deleteStateTreeRecursively
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	BL f_deleteStateTreeRecursively
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #9]
	MOV r0, r4
	ADD sp, sp, #29
	POP {pc}
	POP {pc}
	.ltorg
f_deleteStateTreeRecursively:
	PUSH {lr}
	LDR r4, [sp, #4]
	LDR r5, =0
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L74
	MOV r4, #1
	MOV r0, r4
	POP {pc}
	B L75
L74:
	SUB sp, sp, #13
	LDR r4, [sp, #17]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #9]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #9]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	BL f_deleteChildrenStateRecursively
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #17]
	STR r4, [sp, #-4]!
	BL f_deleteThisStateOnly
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #13
	POP {pc}
	ADD sp, sp, #13
L75:
	POP {pc}
	.ltorg
f_deleteThisStateOnly:
	PUSH {lr}
	SUB sp, sp, #13
	LDR r4, [sp, #17]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #9]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #9]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	BL f_freeBoard
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	BL f_freePointers
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #9]
	MOV r0, r4
	BL p_free_pair
	LDR r4, [sp, #17]
	MOV r0, r4
	BL p_free_pair
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #13
	POP {pc}
	POP {pc}
	.ltorg
f_freePointers:
	PUSH {lr}
	SUB sp, sp, #17
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #13]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	BL f_freePointersRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	BL f_freePointersRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	BL f_freePointersRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_free_pair
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_free_pair
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #17
	POP {pc}
	POP {pc}
	.ltorg
f_freePointersRow:
	PUSH {lr}
	SUB sp, sp, #4
	LDR r4, [sp, #8]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	BL p_free_pair
	LDR r4, [sp, #8]
	MOV r0, r4
	BL p_free_pair
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #4
	POP {pc}
	POP {pc}
	.ltorg
f_deleteChildrenStateRecursively:
	PUSH {lr}
	SUB sp, sp, #17
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #13]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	BL f_deleteChildrenStateRecursivelyRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	BL f_deleteChildrenStateRecursivelyRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	BL f_deleteChildrenStateRecursivelyRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #17
	POP {pc}
	POP {pc}
	.ltorg
f_deleteChildrenStateRecursivelyRow:
	PUSH {lr}
	SUB sp, sp, #17
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #13]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	BL f_deleteStateTreeRecursively
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	BL f_deleteStateTreeRecursively
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	BL f_deleteStateTreeRecursively
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #17
	POP {pc}
	POP {pc}
	.ltorg
f_askForAMove:
	PUSH {lr}
	LDRSB r4, [sp, #8]
	LDRSB r5, [sp, #9]
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L76
	SUB sp, sp, #1
	LDR r4, [sp, #15]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	BL f_askForAMoveHuman
	ADD sp, sp, #8
	MOV r4, r0
	STRB r4, [sp]
	ADD sp, sp, #1
	B L77
L76:
	SUB sp, sp, #1
	LDR r4, [sp, #15]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #15]
	STR r4, [sp, #-4]!
	LDRSB r4, [sp, #18]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #18]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #15]
	STR r4, [sp, #-4]!
	BL f_askForAMoveAI
	ADD sp, sp, #14
	MOV r4, r0
	STRB r4, [sp]
	ADD sp, sp, #1
L77:
	MOV r4, #1
	MOV r0, r4
	POP {pc}
	POP {pc}
	.ltorg
f_placeMove:
	PUSH {lr}
	SUB sp, sp, #4
	LDR r4, =0
	STR r4, [sp]
	LDR r4, [sp, #13]
	LDR r5, =2
	CMP r4, r5
	MOVLE r4, #1
	MOVGT r4, #0
	CMP r4, #0
	BEQ L78
	SUB sp, sp, #4
	LDR r4, [sp, #12]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp]
	LDR r4, [sp, #17]
	LDR r5, =1
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L80
	LDR r4, [sp]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #4]
	B L81
L80:
	LDR r4, [sp]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #4]
L81:
	ADD sp, sp, #4
	B L79
L78:
	LDR r4, [sp, #8]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp]
L79:
	LDR r4, [sp, #17]
	LDR r5, =2
	CMP r4, r5
	MOVLE r4, #1
	MOVGT r4, #0
	CMP r4, #0
	BEQ L82
	SUB sp, sp, #4
	LDR r4, [sp, #4]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp]
	LDR r4, [sp, #21]
	LDR r5, =1
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L84
	LDRSB r4, [sp, #16]
	LDR r5, [sp]
	MOV r0, r5
	BL p_check_null_pointer
	LDR r5, [r5]
	STRB r4, [r5]
	B L85
L84:
	LDRSB r4, [sp, #16]
	LDR r5, [sp]
	MOV r0, r5
	BL p_check_null_pointer
	LDR r5, [r5, #4]
	STRB r4, [r5]
L85:
	ADD sp, sp, #4
	B L83
L82:
	LDRSB r4, [sp, #12]
	LDR r5, [sp]
	MOV r0, r5
	BL p_check_null_pointer
	LDR r5, [r5, #4]
	STRB r4, [r5]
L83:
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #4
	POP {pc}
	POP {pc}
	.ltorg
f_notifyMove:
	PUSH {lr}
	LDRSB r4, [sp, #8]
	LDRSB r5, [sp, #9]
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L86
	SUB sp, sp, #1
	LDR r4, [sp, #19]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #19]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #19]
	STR r4, [sp, #-4]!
	LDRSB r4, [sp, #22]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #22]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #19]
	STR r4, [sp, #-4]!
	BL f_notifyMoveAI
	ADD sp, sp, #18
	MOV r4, r0
	STRB r4, [sp]
	ADD sp, sp, #1
	B L87
L86:
	SUB sp, sp, #1
	LDR r4, [sp, #19]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #19]
	STR r4, [sp, #-4]!
	LDRSB r4, [sp, #18]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #18]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #15]
	STR r4, [sp, #-4]!
	BL f_notifyMoveHuman
	ADD sp, sp, #14
	MOV r4, r0
	STRB r4, [sp]
	ADD sp, sp, #1
L87:
	MOV r4, #1
	MOV r0, r4
	POP {pc}
	POP {pc}
	.ltorg
f_oppositeSymbol:
	PUSH {lr}
	LDRSB r4, [sp, #4]
	MOV r5, #'x'
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L88
	MOV r4, #'o'
	MOV r0, r4
	POP {pc}
	B L89
L88:
	LDRSB r4, [sp, #4]
	MOV r5, #'o'
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L90
	MOV r4, #'x'
	MOV r0, r4
	POP {pc}
	B L91
L90:
	LDR r4, =msg_33
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =-1
	MOV r0, r4
	BL exit
L91:
L89:
	POP {pc}
	.ltorg
f_symbolAt:
	PUSH {lr}
	SUB sp, sp, #5
	LDR r4, =0
	STR r4, [sp, #1]
	LDR r4, [sp, #13]
	LDR r5, =2
	CMP r4, r5
	MOVLE r4, #1
	MOVGT r4, #0
	CMP r4, #0
	BEQ L92
	SUB sp, sp, #4
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp]
	LDR r4, [sp, #17]
	LDR r5, =1
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L94
	LDR r4, [sp]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	B L95
L94:
	LDR r4, [sp]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #5]
L95:
	ADD sp, sp, #4
	B L93
L92:
	LDR r4, [sp, #9]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
L93:
	MOV r4, #0
	STRB r4, [sp]
	LDR r4, [sp, #17]
	LDR r5, =2
	CMP r4, r5
	MOVLE r4, #1
	MOVGT r4, #0
	CMP r4, #0
	BEQ L96
	SUB sp, sp, #4
	LDR r4, [sp, #5]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp]
	LDR r4, [sp, #21]
	LDR r5, =1
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L98
	LDR r4, [sp]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDRSB r4, [r4]
	STRB r4, [sp, #4]
	B L99
L98:
	LDR r4, [sp]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDRSB r4, [r4]
	STRB r4, [sp, #4]
L99:
	ADD sp, sp, #4
	B L97
L96:
	LDR r4, [sp, #1]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDRSB r4, [r4]
	STRB r4, [sp]
L97:
	LDRSB r4, [sp]
	MOV r0, r4
	ADD sp, sp, #5
	POP {pc}
	POP {pc}
	.ltorg
f_containEmptyCell:
	PUSH {lr}
	SUB sp, sp, #19
	LDR r4, [sp, #23]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #15]
	LDR r4, [sp, #15]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #11]
	LDR r4, [sp, #15]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #7]
	LDR r4, [sp, #23]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #3]
	LDR r4, [sp, #11]
	STR r4, [sp, #-4]!
	BL f_containEmptyCellRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp, #2]
	LDR r4, [sp, #7]
	STR r4, [sp, #-4]!
	BL f_containEmptyCellRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp, #1]
	LDR r4, [sp, #3]
	STR r4, [sp, #-4]!
	BL f_containEmptyCellRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp, #2]
	LDRSB r5, [sp, #1]
	ORR r4, r4, r5
	LDRSB r5, [sp]
	ORR r4, r4, r5
	MOV r0, r4
	ADD sp, sp, #19
	POP {pc}
	POP {pc}
	.ltorg
f_containEmptyCellRow:
	PUSH {lr}
	SUB sp, sp, #7
	LDR r4, [sp, #11]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #3]
	LDR r4, [sp, #3]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDRSB r4, [r4]
	STRB r4, [sp, #2]
	LDR r4, [sp, #3]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDRSB r4, [r4]
	STRB r4, [sp, #1]
	LDR r4, [sp, #11]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDRSB r4, [r4]
	STRB r4, [sp]
	LDRSB r4, [sp, #2]
	MOV r5, #0
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	LDRSB r5, [sp, #1]
	MOV r6, #0
	CMP r5, r6
	MOVEQ r5, #1
	MOVNE r5, #0
	ORR r4, r4, r5
	LDRSB r5, [sp]
	MOV r6, #0
	CMP r5, r6
	MOVEQ r5, #1
	MOVNE r5, #0
	ORR r4, r4, r5
	MOV r0, r4
	ADD sp, sp, #7
	POP {pc}
	POP {pc}
	.ltorg
f_hasWon:
	PUSH {lr}
	SUB sp, sp, #9
	LDR r4, =1
	STR r4, [sp, #-4]!
	LDR r4, =1
	STR r4, [sp, #-4]!
	LDR r4, [sp, #21]
	STR r4, [sp, #-4]!
	BL f_symbolAt
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp, #8]
	LDR r4, =2
	STR r4, [sp, #-4]!
	LDR r4, =1
	STR r4, [sp, #-4]!
	LDR r4, [sp, #21]
	STR r4, [sp, #-4]!
	BL f_symbolAt
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp, #7]
	LDR r4, =3
	STR r4, [sp, #-4]!
	LDR r4, =1
	STR r4, [sp, #-4]!
	LDR r4, [sp, #21]
	STR r4, [sp, #-4]!
	BL f_symbolAt
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp, #6]
	LDR r4, =1
	STR r4, [sp, #-4]!
	LDR r4, =2
	STR r4, [sp, #-4]!
	LDR r4, [sp, #21]
	STR r4, [sp, #-4]!
	BL f_symbolAt
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp, #5]
	LDR r4, =2
	STR r4, [sp, #-4]!
	LDR r4, =2
	STR r4, [sp, #-4]!
	LDR r4, [sp, #21]
	STR r4, [sp, #-4]!
	BL f_symbolAt
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp, #4]
	LDR r4, =3
	STR r4, [sp, #-4]!
	LDR r4, =2
	STR r4, [sp, #-4]!
	LDR r4, [sp, #21]
	STR r4, [sp, #-4]!
	BL f_symbolAt
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp, #3]
	LDR r4, =1
	STR r4, [sp, #-4]!
	LDR r4, =3
	STR r4, [sp, #-4]!
	LDR r4, [sp, #21]
	STR r4, [sp, #-4]!
	BL f_symbolAt
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp, #2]
	LDR r4, =2
	STR r4, [sp, #-4]!
	LDR r4, =3
	STR r4, [sp, #-4]!
	LDR r4, [sp, #21]
	STR r4, [sp, #-4]!
	BL f_symbolAt
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp, #1]
	LDR r4, =3
	STR r4, [sp, #-4]!
	LDR r4, =3
	STR r4, [sp, #-4]!
	LDR r4, [sp, #21]
	STR r4, [sp, #-4]!
	BL f_symbolAt
	ADD sp, sp, #12
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp, #8]
	LDRSB r5, [sp, #17]
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	LDRSB r5, [sp, #7]
	LDRSB r6, [sp, #17]
	CMP r5, r6
	MOVEQ r5, #1
	MOVNE r5, #0
	AND r4, r4, r5
	LDRSB r5, [sp, #6]
	LDRSB r6, [sp, #17]
	CMP r5, r6
	MOVEQ r5, #1
	MOVNE r5, #0
	AND r4, r4, r5
	LDRSB r5, [sp, #5]
	LDRSB r6, [sp, #17]
	CMP r5, r6
	MOVEQ r5, #1
	MOVNE r5, #0
	LDRSB r6, [sp, #4]
	LDRSB r7, [sp, #17]
	CMP r6, r7
	MOVEQ r6, #1
	MOVNE r6, #0
	AND r5, r5, r6
	LDRSB r6, [sp, #3]
	LDRSB r7, [sp, #17]
	CMP r6, r7
	MOVEQ r6, #1
	MOVNE r6, #0
	AND r5, r5, r6
	ORR r4, r4, r5
	LDRSB r5, [sp, #2]
	LDRSB r6, [sp, #17]
	CMP r5, r6
	MOVEQ r5, #1
	MOVNE r5, #0
	LDRSB r6, [sp, #1]
	LDRSB r7, [sp, #17]
	CMP r6, r7
	MOVEQ r6, #1
	MOVNE r6, #0
	AND r5, r5, r6
	LDRSB r6, [sp]
	LDRSB r7, [sp, #17]
	CMP r6, r7
	MOVEQ r6, #1
	MOVNE r6, #0
	AND r5, r5, r6
	ORR r4, r4, r5
	LDRSB r5, [sp, #8]
	LDRSB r6, [sp, #17]
	CMP r5, r6
	MOVEQ r5, #1
	MOVNE r5, #0
	LDRSB r6, [sp, #5]
	LDRSB r7, [sp, #17]
	CMP r6, r7
	MOVEQ r6, #1
	MOVNE r6, #0
	AND r5, r5, r6
	LDRSB r6, [sp, #2]
	LDRSB r7, [sp, #17]
	CMP r6, r7
	MOVEQ r6, #1
	MOVNE r6, #0
	AND r5, r5, r6
	ORR r4, r4, r5
	LDRSB r5, [sp, #7]
	LDRSB r6, [sp, #17]
	CMP r5, r6
	MOVEQ r5, #1
	MOVNE r5, #0
	LDRSB r6, [sp, #4]
	LDRSB r7, [sp, #17]
	CMP r6, r7
	MOVEQ r6, #1
	MOVNE r6, #0
	AND r5, r5, r6
	LDRSB r6, [sp, #1]
	LDRSB r7, [sp, #17]
	CMP r6, r7
	MOVEQ r6, #1
	MOVNE r6, #0
	AND r5, r5, r6
	ORR r4, r4, r5
	LDRSB r5, [sp, #6]
	LDRSB r6, [sp, #17]
	CMP r5, r6
	MOVEQ r5, #1
	MOVNE r5, #0
	LDRSB r6, [sp, #3]
	LDRSB r7, [sp, #17]
	CMP r6, r7
	MOVEQ r6, #1
	MOVNE r6, #0
	AND r5, r5, r6
	LDRSB r6, [sp]
	LDRSB r7, [sp, #17]
	CMP r6, r7
	MOVEQ r6, #1
	MOVNE r6, #0
	AND r5, r5, r6
	ORR r4, r4, r5
	LDRSB r5, [sp, #8]
	LDRSB r6, [sp, #17]
	CMP r5, r6
	MOVEQ r5, #1
	MOVNE r5, #0
	LDRSB r6, [sp, #4]
	LDRSB r7, [sp, #17]
	CMP r6, r7
	MOVEQ r6, #1
	MOVNE r6, #0
	AND r5, r5, r6
	LDRSB r6, [sp]
	LDRSB r7, [sp, #17]
	CMP r6, r7
	MOVEQ r6, #1
	MOVNE r6, #0
	AND r5, r5, r6
	ORR r4, r4, r5
	LDRSB r5, [sp, #6]
	LDRSB r6, [sp, #17]
	CMP r5, r6
	MOVEQ r5, #1
	MOVNE r5, #0
	LDRSB r6, [sp, #4]
	LDRSB r7, [sp, #17]
	CMP r6, r7
	MOVEQ r6, #1
	MOVNE r6, #0
	AND r5, r5, r6
	LDRSB r6, [sp, #2]
	LDRSB r7, [sp, #17]
	CMP r6, r7
	MOVEQ r6, #1
	MOVNE r6, #0
	AND r5, r5, r6
	ORR r4, r4, r5
	MOV r0, r4
	ADD sp, sp, #9
	POP {pc}
	POP {pc}
	.ltorg
f_allocateNewBoard:
	PUSH {lr}
	SUB sp, sp, #20
	BL f_allocateNewRow
	MOV r4, r0
	STR r4, [sp, #16]
	BL f_allocateNewRow
	MOV r4, r0
	STR r4, [sp, #12]
	BL f_allocateNewRow
	MOV r4, r0
	STR r4, [sp, #8]
	LDR r0, =8
	BL malloc
	MOV r4, r0
	LDR r5, [sp, #16]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4]
	LDR r5, [sp, #12]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp, #4]
	LDR r0, =8
	BL malloc
	MOV r4, r0
	LDR r5, [sp, #4]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4]
	LDR r5, [sp, #8]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	ADD sp, sp, #20
	POP {pc}
	POP {pc}
	.ltorg
f_allocateNewRow:
	PUSH {lr}
	SUB sp, sp, #8
	LDR r0, =8
	BL malloc
	MOV r4, r0
	MOV r5, #0
	LDR r0, =1
	BL malloc
	STRB r5, [r0]
	STR r0, [r4]
	MOV r5, #0
	LDR r0, =1
	BL malloc
	STRB r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp, #4]
	LDR r0, =8
	BL malloc
	MOV r4, r0
	LDR r5, [sp, #4]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4]
	MOV r5, #0
	LDR r0, =1
	BL malloc
	STRB r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	ADD sp, sp, #8
	POP {pc}
	POP {pc}
	.ltorg
f_freeBoard:
	PUSH {lr}
	SUB sp, sp, #17
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #13]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	BL f_freeRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	BL f_freeRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	BL f_freeRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_free_pair
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_free_pair
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #17
	POP {pc}
	POP {pc}
	.ltorg
f_freeRow:
	PUSH {lr}
	SUB sp, sp, #4
	LDR r4, [sp, #8]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	BL p_free_pair
	LDR r4, [sp, #8]
	MOV r0, r4
	BL p_free_pair
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #4
	POP {pc}
	POP {pc}
	.ltorg
f_printAiData:
	PUSH {lr}
	SUB sp, sp, #9
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	BL f_printStateTreeRecursively
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, =0
	MOV r0, r4
	BL exit
	POP {pc}
	.ltorg
f_printStateTreeRecursively:
	PUSH {lr}
	LDR r4, [sp, #4]
	LDR r5, =0
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L100
	MOV r4, #1
	MOV r0, r4
	POP {pc}
	B L101
L100:
	SUB sp, sp, #17
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #13]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	MOV r4, #'v'
	MOV r0, r4
	BL putchar
	MOV r4, #'='
	MOV r0, r4
	BL putchar
	LDR r4, [sp, #1]
	MOV r0, r4
	BL p_print_int
	BL p_print_ln
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	BL f_printBoard
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	BL f_printChildrenStateTree
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	MOV r4, #'p'
	MOV r0, r4
	BL putchar
	BL p_print_ln
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #17
	POP {pc}
	ADD sp, sp, #17
L101:
	POP {pc}
	.ltorg
f_printChildrenStateTree:
	PUSH {lr}
	SUB sp, sp, #17
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #13]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	BL f_printChildrenStateTreeRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	BL f_printChildrenStateTreeRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	BL f_printChildrenStateTreeRow
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #17
	POP {pc}
	POP {pc}
	.ltorg
f_printChildrenStateTreeRow:
	PUSH {lr}
	SUB sp, sp, #17
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #13]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #9]
	LDR r4, [sp, #13]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #5]
	LDR r4, [sp, #21]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	BL f_printStateTreeRecursively
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #5]
	STR r4, [sp, #-4]!
	BL f_printStateTreeRecursively
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	BL f_printStateTreeRecursively
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #17
	POP {pc}
	POP {pc}
	.ltorg
main:
	PUSH {lr}
	SUB sp, sp, #17
	BL f_chooseSymbol
	MOV r4, r0
	STRB r4, [sp, #16]
	LDRSB r4, [sp, #16]
	STRB r4, [sp, #-1]!
	BL f_oppositeSymbol
	ADD sp, sp, #1
	MOV r4, r0
	STRB r4, [sp, #15]
	MOV r4, #'x'
	STRB r4, [sp, #14]
	BL f_allocateNewBoard
	MOV r4, r0
	STR r4, [sp, #10]
	LDR r4, =msg_34
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDRSB r4, [sp, #15]
	STRB r4, [sp, #-1]!
	BL f_initAI
	ADD sp, sp, #1
	MOV r4, r0
	STR r4, [sp, #6]
	LDR r4, =0
	STR r4, [sp, #2]
	MOV r4, #0
	STRB r4, [sp, #1]
	LDR r4, [sp, #10]
	STR r4, [sp, #-4]!
	BL f_printBoard
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	B L102
L103:
	SUB sp, sp, #5
	LDR r0, =12
	BL malloc
	MOV r4, r0
	LDR r5, =0
	STR r5, [r4, #4]
	LDR r5, =0
	STR r5, [r4, #8]
	LDR r5, =2
	STR r5, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #15]
	STR r4, [sp, #-4]!
	LDRSB r4, [sp, #29]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #28]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #25]
	STR r4, [sp, #-4]!
	BL f_askForAMove
	ADD sp, sp, #14
	MOV r4, r0
	STRB r4, [sp, #5]
	ADD r4, sp, #1
	LDR r5, =1
	LDR r4, [r4]
	MOV r0, r5
	MOV r1, r4
	BL p_check_array_bounds
	ADD r4, r4, #4
	ADD r4, r4, r5, LSL #2
	LDR r4, [r4]
	STR r4, [sp, #-4]!
	ADD r4, sp, #5
	LDR r5, =0
	LDR r4, [r4]
	MOV r0, r5
	MOV r1, r4
	BL p_check_array_bounds
	ADD r4, r4, #4
	ADD r4, r4, r5, LSL #2
	LDR r4, [r4]
	STR r4, [sp, #-4]!
	LDRSB r4, [sp, #27]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #24]
	STR r4, [sp, #-4]!
	BL f_placeMove
	ADD sp, sp, #13
	MOV r4, r0
	STRB r4, [sp, #5]
	ADD r4, sp, #1
	LDR r5, =1
	LDR r4, [r4]
	MOV r0, r5
	MOV r1, r4
	BL p_check_array_bounds
	ADD r4, r4, #4
	ADD r4, r4, r5, LSL #2
	LDR r4, [r4]
	STR r4, [sp, #-4]!
	ADD r4, sp, #5
	LDR r5, =0
	LDR r4, [r4]
	MOV r0, r5
	MOV r1, r4
	BL p_check_array_bounds
	ADD r4, r4, #4
	ADD r4, r4, r5, LSL #2
	LDR r4, [r4]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #19]
	STR r4, [sp, #-4]!
	LDRSB r4, [sp, #33]
	STRB r4, [sp, #-1]!
	LDRSB r4, [sp, #32]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #29]
	STR r4, [sp, #-4]!
	BL f_notifyMove
	ADD sp, sp, #18
	MOV r4, r0
	STRB r4, [sp, #5]
	LDR r4, [sp, #15]
	STR r4, [sp, #-4]!
	BL f_printBoard
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp, #5]
	LDRSB r4, [sp, #19]
	STRB r4, [sp, #-1]!
	LDR r4, [sp, #16]
	STR r4, [sp, #-4]!
	BL f_hasWon
	ADD sp, sp, #5
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp]
	CMP r4, #0
	BEQ L104
	LDRSB r4, [sp, #19]
	STRB r4, [sp, #6]
	B L105
L104:
L105:
	LDRSB r4, [sp, #19]
	STRB r4, [sp, #-1]!
	BL f_oppositeSymbol
	ADD sp, sp, #1
	MOV r4, r0
	STRB r4, [sp, #19]
	LDR r4, [sp, #7]
	LDR r5, =1
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	STR r4, [sp, #7]
	ADD sp, sp, #5
L102:
	LDRSB r4, [sp, #1]
	MOV r5, #0
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	LDR r5, [sp, #2]
	LDR r6, =9
	CMP r5, r6
	MOVLT r5, #1
	MOVGE r5, #0
	AND r4, r4, r5
	CMP r4, #1
	BEQ L103
	LDR r4, [sp, #10]
	STR r4, [sp, #-4]!
	BL f_freeBoard
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, [sp, #6]
	STR r4, [sp, #-4]!
	BL f_destroyAI
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp]
	LDRSB r4, [sp, #1]
	MOV r5, #0
	CMP r4, r5
	MOVNE r4, #1
	MOVEQ r4, #0
	CMP r4, #0
	BEQ L106
	LDRSB r4, [sp, #1]
	MOV r0, r4
	BL putchar
	LDR r4, =msg_35
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	B L107
L106:
	LDR r4, =msg_36
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
L107:
	ADD sp, sp, #17
	LDR r0, =0
	POP {pc}
	.ltorg
p_print_string:
	PUSH {lr}
	LDR r1, [r0]
	ADD r2, r0, #4
	LDR r0, =msg_37
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
p_print_ln:
	PUSH {lr}
	LDR r0, =msg_38
	ADD r0, r0, #4
	BL puts
	MOV r0, #0
	BL fflush
	POP {pc}
p_read_char:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_39
	ADD r0, r0, #4
	BL scanf
	POP {pc}
p_check_null_pointer:
	PUSH {lr}
	CMP r0, #0
	LDREQ r0, =msg_40
	BLEQ p_throw_runtime_error
	POP {pc}
p_read_int:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_41
	ADD r0, r0, #4
	BL scanf
	POP {pc}
p_check_array_bounds:
	PUSH {lr}
	CMP r0, #0
	LDRLT r0, =msg_42
	BLLT p_throw_runtime_error
	LDR r1, [r1]
	CMP r0, r1
	LDRCS r0, =msg_43
	BLCS p_throw_runtime_error
	POP {pc}
p_print_int:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_44
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
p_free_pair:
	PUSH {lr}
	CMP r0, #0
	LDREQ r0, =msg_45
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
p_throw_overflow_error:
	LDR r0, =msg_46
	BL p_throw_runtime_error
p_throw_runtime_error:
	BL p_print_string
	MOV r0, #-1
	BL exit
