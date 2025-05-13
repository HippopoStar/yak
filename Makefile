
ARCH ?= x86

# GNU Make Manual:
# > trailing space characters are not stripped from variable values
# debug | release
BUILD_PROFILE = release

NAME = yak-$(ARCH).iso



ifeq ($(ARCH), x86)
	GRUB_MKRESCUE_OPT = -d /usr/lib/grub/i386-pc
	QEMU_BIN = qemu-system-i386
endif

ifeq ($(BUILD_PROFILE), release)
	CARGO_BUILD_OPT = --release
else
	CARGO_BUILD_OPT =
endif



LIBBOOT_DIR = ./asm
LIBBOOT = $(LIBBOOT_DIR)/libboot.a

LIBYAK_DIR = ./rust/target/$(ARCH)-unknown-none/$(BUILD_PROFILE)
LIBYAK = $(LIBYAK_DIR)/libyak.a

LINKER_SCRIPT = ./arch/$(ARCH)/linker.ld

ROOTFS_DIR = ./rootfs

KERNEL = $(ROOTFS_DIR)/boot/kernel.bin



all: $(NAME)

$(NAME): $(KERNEL)
	grub-mkrescue -o $@ $(GRUB_MKRESCUE_OPT) $(ROOTFS_DIR)

$(KERNEL): $(LINKER_SCRIPT) $(LIBBOOT) $(LIBYAK)
	# ld -o $@ --fatal-warnings -n -T $< -L$(LIBBOOT_DIR) -L$(LIBYAK_DIR) --whole-archive -lboot --no-whole-archive -lyak
	ld -o $@ -n -T $< -L$(LIBYAK_DIR) ./asm/obj/multiboot_header.o ./asm/obj/boot.o -lyak

$(LIBBOOT): FORCE
	@make -C ./asm all

$(LIBYAK): FORCE
	@cargo -Z unstable-options -C ./rust build $(CARGO_BUILD_OPT) --target arch/$(ARCH)/$(ARCH)-unknown-none.json

# Workaround
# FORCE has to be a nonexistent file
# GNU Make Manual:
# - 4.7 Rules without Recipes or Prerequisites
# - 5.9 Using Empty Recipes
# Another possible workaround would imply Double-Colon Rules
FORCE: ;

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

