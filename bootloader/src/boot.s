
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

put_str:                             ; Max string length = 80x25 bytes (VGA buffer size / 2)
	mov ecx, str_len
	jecxz put_str_end                ; Jump if ECX register is 0
put_str_loop:
	dec ecx                          ; Decrement by 1 (affects ZF)
	mov byte dl, [str + ecx]
	mov byte [0xb80A0 + ecx * 2], dl
	jnz put_str_loop                 ; Jump if not zero (ZF=0)
put_str_end:
	ret

