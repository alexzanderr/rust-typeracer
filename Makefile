


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


ch:
	@cargo run --quiet --package dev-ideas --example chan --features channels

cm:
	@cargo run --quiet --package dev-ideas --example cm --features channels


check:
	@cargo check --quiet

c: check

code_stats:
	tokei -C -s code

cc: code_stats


show_threads:
	ps Haux | rg typeracer

st: show_threads


# remember this for Makefile
# you cant have tabs and spaces at the same time
# i guess this also happens in rust

#  74   │ ␊
#  75   │ ␊
#  76   │ show_threads:␊
#  77   │ ├──┤ps·Haux·|·rg·typeracer␊
#  78   │ ␊
#  79   │ st:·show_threads␊
#  80   │ ␊
#  81 + │ check_continuous:␊
#  82 + │ ····@time·cargo·watch·\␊
#  83 + │ ········--clear·\␊
#  84 + │ ········--watch="src"·\␊
#  85 + │ ········--watch="dev-ideas"␊
#  86   │ ␊
#  87 + │ cc:·check_continuous␊

check_loop:
	@echo "hello world"

	@time cargo watch \
		--clear \
		--delay 1.2 \
		--why \
		--quiet \
		--postpone \
		--watch-when-idle \
		--watch="src" \
		--watch="benches" \
		--watch="dev-ideas" \
		--watch="examples" \
		--shell="cargo check --quiet"

cc: check_loop


check_workspace:
	@time cargo watch \
		--clear \
		--delay 1.2 \
		--why \
		--quiet \
		--postpone \
		--watch-when-idle \
		--watch="src" \
		--watch="benches" \
		--watch="dev-ideas" \
		--watch="examples" \
		--shell="cargo check --quiet --workspace"

cw: check_workspace


cargo_docs:
	@time cargo doc --no-deps --all-features --document-private-items --workspace --open

md: cargo_docs
