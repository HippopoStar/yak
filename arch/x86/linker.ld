
ENTRY(_start)
OUTPUT_ARCH(i386)
OUTPUT_FORMAT(elf32-i386)

SECTIONS {
	. = 1M;

	.boot :
	{
		/* ensure that the multiboot header is at the beginning */
		KEEP (*(.multiboot_header))
	}

	.text :
	{
		*(.text .text.*)
	}

	.rodata :
	{
		*(.rodata .rodata.*)
	}

	.data.rel.ro :
	{
		*(.data.rel.ro.local*) *(.data.rel.ro .data.rel.ro.*)
	}

	.data :
	{
		*(.data .data.*)
	}

	.bss :
	{
		*(.bss .bss.*)
	}
}

