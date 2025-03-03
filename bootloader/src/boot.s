
global start

section .text
bits 32
start:
	; print `OK` to screen
	mov dword [0xb8000], 0x00320034
	hlt

