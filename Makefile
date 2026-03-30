#####	VARIABLES   #####
NAME = computor

SRCS = src/main.rs src/parsing.rs

debug ?= $(info debug is $(debug))

ifndef debug
	release :=
	target := debug
else
	release := --release
	target := release
endif


########		COLORS		########

COLOR_RESET		=	\033[0m
COLOR_RED		=	\033[1;31m
COLOR_GREEN		=	\033[1;32m
COLOR_YELLOW	=	\033[1;93m
COLOR_BLUE		=	\033[1;94m
COLOR_PINK		=	\033[38;5;206m
COLOR_LBLUE		=	\033[1;94m

########		RULES   	########

all: $(NAME)

$(NAME): $(SRCS) Cargo.toml
	@cargo build $(release)
	@cp target/$(target)/computor ./computor
	@echo ""
	@echo "$(COLOR_GREEN)"
	@echo "   _   _   _   _   _   _   _   _     _   _  "
	@echo "  / \ / \ / \ / \ / \ / \ / \ / \   / \ / \ "
	@echo " ( C | O | M | P | U | T | O | R ) ( v | 1 )"
	@echo "  \_/ \_/ \_/ \_/ \_/ \_/ \_/ \_/   \_/ \_/ "
	@echo "$(COLOR_RESET)"

run: $(NAME)
	@cargo run $(release) -- "5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0"

clean:
	@cargo clean

fclean: clean
	@rm ./computor

re: fclean all

.PHONY: all clean fclean re run

