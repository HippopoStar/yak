
ENTRY(_start)
OUTPUT_ARCH(i386)
OUTPUT_FORMAT(elf32-i386)

SECTIONS {
	. = 1M;

	.boot :
	{
		/* ensure that the multiboot header is at the beginning */
		*(.multiboot_header)
	}

	.text :
	{
		*(.text)
	}

	.data :
	{
		*(.data)
	}

	.bss :
	{
		*(.bss)
	}
}

