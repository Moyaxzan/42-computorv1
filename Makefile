#####	VARIABLES   #####
NAME = computor

SRCS = src/main.rs src/parsing.rs src/math_utils.rs src/solve.rs


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

$(NAME): $(SRCS) Cargo.toml Makefile
	@cargo build --release
	@cp target/release/computor ./computor
	@echo ""
	@echo "$(COLOR_GREEN)"
	@echo "   _   _   _   _   _   _   _   _     _   _  "
	@echo "  / \ / \ / \ / \ / \ / \ / \ / \   / \ / \ "
	@echo " ( C | O | M | P | U | T | O | R ) ( v | 1 )"
	@echo "  \_/ \_/ \_/ \_/ \_/ \_/ \_/ \_/   \_/ \_/ "
	@echo "$(COLOR_RESET)"

bonus: $(SRCS) Cargo.toml
	@RUSTFLAGS="--cfg bonus" cargo build --release
	@cp target/release/computor ./computor
	@echo ""
	@echo "$(COLOR_PINK)"
	@echo "   _   _   _   _   _   _   _   _     _   _  "
	@echo "  / \ / \ / \ / \ / \ / \ / \ / \   / \ / \ "
	@echo " ( C | O | M | P | U | T | O | R ) ( v | 1 )"
	@echo "  \_/ \_/ \_/ \_/ \_/ \_/ \_/ \_/   \_/ \_/ "
	@echo "$(COLOR_RESET)"

run: $(NAME)
	@echo "------------------------------------------------------"
	@echo "                    SUBJECT TESTS"
	@echo "------------------------------------------------------"
	./computor "5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0"
	@echo "------------------------------------------------------"
	./computor "5 * X^0 + 4 * X^1 = 4 * X^0"
	@echo "------------------------------------------------------"
	./computor "8 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 3 * X^0"
	@echo "------------------------------------------------------"
	./computor "6 * X^0 = 6 * X^0"
	@echo "------------------------------------------------------"
	./computor "10 * X^0 = 15 * X^0"
	@echo "------------------------------------------------------"
	./computor "1 * X^0 + 2 * X^1 + 5 * X^2 = 0"
	@echo "------------------------------------------------------"
	./computor "5 + 4 * X + X^2 = X^2"
	@echo "------------------------------------------------------"
	@echo "                      MY TESTS"
	@echo "------------------------------------------------------"
	./computor "1 * X^0 + 4 * X^1 + 4 * X^2 = 0"
	@echo "------------------------------------------------------"
	./computor "3 * X^0 + 4.2 * X^1 + 2.4 * X^2 = 0"
	@echo "------------------------------------------------------"
	./computor "3 * + 4.2 * X^1 + 2.4 * X^2 = 0"
	@echo "------------------------------------------------------"
	./computor "3 + 4.2 * x^ + 2.4 * X^2 = 0"
	@echo "------------------------------------------------------"
	./computor "3 + 4.2 * x^1 + . * X^2 = 0"
	@echo "------------------------------------------------------"
	./computor "="
	@echo "------------------------------------------------------"
	./computor "3"
clean:
	@cargo clean

fclean: clean
	@rm ./computor

re: fclean all

.PHONY: all clean fclean re run

