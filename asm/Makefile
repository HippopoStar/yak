
ARCH ?= x86

NAME = libboot.a



ifeq ($(ARCH), x86)
	NASM_OPT = -f elf32
endif



ASM_SRC_DIR = ./src

ASM_SRC_NAME = \
	multiboot_header.s \
	boot.s

ASM_SRC = $(addprefix $(ASM_SRC_DIR)/,$(ASM_SRC_NAME))

ASM_OBJ_DIR = ./obj

ASM_OBJ_NAME = $(ASM_SRC_NAME:.s=.o)

ASM_OBJ = $(addprefix $(ASM_OBJ_DIR)/,$(ASM_OBJ_NAME))



all: $(NAME)

$(NAME): $(ASM_OBJ)
	ar rc $(AR_OPT) $@ $^
	ranlib $@

$(ASM_OBJ_DIR)/%.o: $(ASM_SRC_DIR)/%.s | $(ASM_OBJ_DIR)
	nasm -o $@ -Wall -Werror $(NASM_OPT) $<

$(ASM_OBJ_DIR):
	mkdir $(ASM_OBJ_DIR)

clean:
	rm -f $(ASM_OBJ)
	rm -df $(ASM_OBJ_DIR)

fclean: clean
	rm -f $(NAME)

re: fclean all

.PHONY: all clean fclean re

