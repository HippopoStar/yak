
ARCH ?= x86

NAME = yak-$(ARCH).iso



ifeq ($(ARCH), x86)
	GRUB_MKRESCUE_OPT = -d /usr/lib/grub/i386-pc
	QEMU_BIN = qemu-system-i386
endif


LIBBOOT_DIR = ./asm
LIBBOOT = $(LIBBOOT_DIR)/libboot.a

LIBYAK_DIR = ./rust/target/$(ARCH)-unknown-none/release
LIBYAK = $(LIBYAK_DIR)/libyak.a

LINKER_SCRIPT = ./arch/$(ARCH)/linker.ld

ROOTFS_DIR = ./rootfs

KERNEL = $(ROOTFS_DIR)/boot/kernel.bin



all: $(NAME)

$(NAME): $(KERNEL)
	grub-mkrescue -o $@ $(GRUB_MKRESCUE_OPT) $(ROOTFS_DIR)

$(KERNEL): $(LINKER_SCRIPT) $(LIBBOOT) $(LIBYAK)
	ld -o $@ --cref --fatal-warnings -n -T $< -L$(LIBBOOT_DIR) -L$(LIBYAK_DIR) --whole-archive -lboot --no-whole-archive -lyak

$(LIBBOOT):
	@make -C ./asm all

$(LIBYAK):
	@cargo -Z unstable-options -C ./rust build --release

clean:
	@make -C ./asm fclean
	@cargo -Z unstable-options -C ./rust clean
	rm -f $(KERNEL)

fclean: clean
	rm -f $(NAME)

re: fclean all

run: $(NAME)
	# Quit qemu: Alt+2, then type "q" and press Enter
	$(QEMU_BIN) -display curses -cdrom $<

.PHONY: all clean fclean re run

