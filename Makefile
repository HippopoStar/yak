
ARCH ?= x86

NAME = yak-$(ARCH).iso



ifeq ($(ARCH), x86)
	GRUB_MKRESCUE_OPT = -d /usr/lib/grub/i386-pc
	QEMU_BIN = qemu-system-i386
endif



LIBBOOT = ./asm/libboot.a

LINKER_SCRIPT = ./arch/$(ARCH)/linker.ld

ROOTFS_DIR = ./rootfs

KERNEL_DIR = $(ROOTFS_DIR)/boot

KERNEL_NAME = kernel.bin

KERNEL = $(KERNEL_DIR)/$(KERNEL_NAME)



all: $(NAME)

$(NAME): $(KERNEL)
	grub-mkrescue -o $@ $(GRUB_MKRESCUE_OPT) $(ROOTFS_DIR)

$(KERNEL): $(LINKER_SCRIPT) $(LIBBOOT)
	ld -o $@ -n -T $< -L./asm --whole-archive -lboot --no-whole-archive

$(LIBBOOT):
	@make -C ./asm all

clean:
	@make -C ./asm fclean
	rm -f $(KERNEL)

fclean: clean
	rm -f $(NAME)

re: fclean all

run: $(NAME)
	# Quit qemu: Alt+2, then type "q" and press Enter
	$(QEMU_BIN) -display curses -cdrom $<

.PHONY: all clean fclean re run

