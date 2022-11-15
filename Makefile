
# include configuration for the Makefile
include config.mk


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
	tokei -e book --verbose -e .idea -e target -e static -C -s code
#   tokei -C -s code

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
	@time cargo watch \
		--clear \
		--delay 1.2 \
		--why \
		--quiet \
		--watch-when-idle \
		--watch="src" \
		--watch="benches" \
		--watch="dev-ideas" \
		--watch="examples" \
		--shell="cargo check --quiet"

cc: check_loop


check_workspace:
	@# --postpone
	@time cargo watch \
		--clear \
		--delay 1.2 \
		--why \
		--quiet \
		--watch-when-idle \
		--watch="src" \
		--watch="benches" \
		--watch="dev-ideas" \
		--watch="examples" \
		--shell="cargo check --quiet --workspace --all-features"

cw: check_workspace


test_all:
	@cargo test --quiet --workspace --all-features -- --show-output

test_config:
	@cargo test -q --lib config -- --show-output

docs:
	@time cargo doc --no-deps --all-features --document-private-items --workspace --open

md: docs

cfmt:
	@cargo fmt --all -- --check


clip:
	@cargo clippy

#cti: test_all docs clip cfmt
cti: test_all clip cfmt




init-git-config:
	# set git hooks to be in the folder .githooks
	git config --local include.path ../git/config


#publish:
#	cargo doc --no-deps --all-features --document-private-items --workspace
#	cargo test --workspace -- --show-output
#	cargo clippy -- -D warnings
#	cargo fmt --all -- --check
#	cargo publish --key $crates_io_key_as_env_var


# optimizations for the future
#pgo:
#	exit 1
#	# STEP 0: Make sure there is no left-over profiling data from previous runs
#	rm -rf /tmp/pgo-data
#
#	# STEP 1: Build the instrumented binaries
#	RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" \
#	    cargo build --release --target=x86_64-unknown-linux-gnu
#
#	# STEP 2: Run the instrumented binaries with some typical data
#	./target/x86_64-unknown-linux-gnu/release/myprogram mydata1.csv
#	./target/x86_64-unknown-linux-gnu/release/myprogram mydata2.csv
#	./target/x86_64-unknown-linux-gnu/release/myprogram mydata3.csv
#
#	# STEP 3: Merge the `.profraw` files into a `.profdata` file
#	llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data
#
#	# STEP 4: Use the `.profdata` file for guiding optimizations
#	RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata" \
#	    cargo build --release --target=x86_64-unknown-linux-gnu
#
