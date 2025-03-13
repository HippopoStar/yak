
global start

section .data
str: db 'Hello world! (Quit qemu: Alt+2, then type "q" and press Enter)'
str_len: equ $ - str

section .text
bits 32
start:
	call put_str
	mov dword [0xb8000], 0x00320034  ; print '42' to screen
	hlt

put_str:                             ; Max string length = 80x25 bytes (VGA buffer size)
	mov ecx, 0
put_str_loop:
	cmp ecx, str_len                 ; compare
	jae put_str_end                  ; jump if above or equal
	mov byte dl, [str + ecx]
	mov byte [0xb80A0 + ecx * 2], dl
	add ecx, 1                       ; increment counter
	jmp put_str_loop
put_str_end:
	ret

