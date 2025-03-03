
ARCH = x86

NAME = yak-$(ARCH).iso



ASM_SRC_DIR = ./src

ASM_SRC_NAME = \
	multiboot_header.s \
	boot.s

ASM_SRC = $(addprefix $(ASM_SRC_DIR)/,$(ASM_SRC_NAME))

ASM_OBJ_DIR = ./obj

ASM_OBJ_NAME = $(ASM_SRC_NAME:.s=.o)

ASM_OBJ = $(addprefix $(ASM_OBJ_DIR)/,$(ASM_OBJ_NAME))

ROOTFS_DIR = ./rootfs

KERNEL_DIR = $(ROOTFS_DIR)/boot

KERNEL_NAME = kernel.bin

KERNEL = $(KERNEL_DIR)/$(KERNEL_NAME)



all: $(NAME)

$(NAME): $(KERNEL)
	grub-mkrescue -o $@ -d /usr/lib/grub/i386-pc $(ROOTFS_DIR)

$(KERNEL): $(ASM_OBJ)
	ld -o $@ -n -T ./arch/$(ARCH)/linker.ld $^

$(ASM_OBJ_DIR)/%.o: $(ASM_SRC_DIR)/%.s | $(ASM_OBJ_DIR)
	nasm -o $@ -Wall -Werror -@ ./arch/$(ARCH)/nasm.opt $<

$(ASM_OBJ_DIR):
	mkdir $(ASM_OBJ_DIR)

clean:
	rm -f $(ASM_OBJ)
	rm -df $(ASM_OBJ_DIR)
	rm -f $(KERNEL)

fclean: clean
	rm -f $(NAME)

re: fclean all

run: $(NAME)
	qemu-system-i386 -display curses -cdrom $(NAME)

.PHONY: all clean fclean re run

