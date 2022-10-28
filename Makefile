


# BIN = main
# SOURCE = main.c


# aici trebuie sa fie numele binary-ului (main) ca doar atunci cand $(SOURCE) este modified $(SOURCE) sa fie recompilat sa rezulte $(BIN)
# $(BIN): $(SOURCE)
# 	gcc $(SOURCE) -o $(BIN)

# aici este wrapper pentru $(BIN)
# ca doar nu vrei sa scrii in terminal: `> make main` (e aiurea)
# asa ca scrii `> make debug`
# debug: $(BIN)


# aici doar rulezi binary: $(BIN)
# run:
# 	./$(BIN)


# aici rulezi make target $(BIN), adica compilarea
# si apoi rulezi imedidat $(BIN), adica binary in shell
# note: daca rulezi `> make compile_and_run`
# o sa-ti compileze doar o singura data daca codul s-a modificat
# daca codul nu s-a modificat doar ruleaza $(BIN)
# compile_and_run: $(BIN) run
# note: schimba tu numele cum vrei, e doar ca exemplu
# note: acesta este un make target compus din mai multe make targets

# am testat tot, merge as expected
# thats it, enjoy, mon ami


# entire src folder
SRC_FOLDER = src/*
TBIN = target/debug/typeracer

$(TBIN): $(SRC_FOLDER)
	@cargo run -q

run: $(TBIN)

debug:
	ugdb ./target/debug/typeracer --layout "(3s-1c)|(3t)" --gdb=rust-gdb

first_attempt:
	@cargo run --quiet --bin first_attempt

td:
	@cargo run --quiet --package dev-ideas --bin td

mp:
	@cargo run --quiet --package dev-ideas --bin mp



check:
	@cargo check --quiet

c: check

code_stats:
	tokei -C -s code

cc: code_stats



