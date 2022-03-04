.data

msg_0:
	.word 4
	.ascii	" %c\0"

.text

.global main
main:
	PUSH {lr}
	SUB sp, sp, #1
	MOV r4, #'a'
	STRB r4, [sp]
	ADD r4, sp, #0
	MOV r0, r4
	BL p_read_char
	ADD sp, sp, #1
	LDR r0, =0
	POP {pc}
	.ltorg
p_read_char:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_0
	ADD r0, r0, #4
	BL scanf
	POP {pc}
