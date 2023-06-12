UNAME := $(shell uname)

TEST := tests
SRC  := src
RT   := runtime
P1   := course
P2   := main

ifeq ($(UNAME), Linux)
ARCH := elf64
endif
ifeq ($(UNAME), Darwin)
ARCH := macho64
endif

$(TEST)/%.s: $(TEST)/%.snek $(SRC)/main.rs $(SRC)/compiler.rs $(SRC)/parser.rs $(SRC)/utils.rs $(SRC)/types.rs
	cargo run -- $< $(TEST)/$*.s

$(TEST)/$(P1)/%.run: $(TEST)/$(P1)/%.s $(RT)/start.rs
	nasm -f $(ARCH) $(TEST)/$(P1)/$*.s -o $(TEST)/$(P1)/$*.o
	ar rcs $(TEST)/$(P1)/lib$*.a $(TEST)/$(P1)/$*.o
	rustc -gL $(TEST)/$(P1)/ -lour_code:$* $(RT)/start.rs -o $(TEST)/$(P1)/$*.run

$(TEST)/$(P2)/%.run: $(TEST)/$(P2)/%.s $(RT)/start.rs
	nasm -f $(ARCH) $(TEST)/$(P2)/$*.s -o $(TEST)/$(P2)/$*.o
	ar rcs $(TEST)/$(P2)/lib$*.a $(TEST)/$(P2)/$*.o
	rustc -gL $(TEST)/$(P2)/ -lour_code:$* $(RT)/start.rs -o $(TEST)/$(P2)/$*.run

$(TEST)/%.run: $(TEST)/%.s $(RT)/start.rs
	nasm -f $(ARCH) $(TEST)/$*.s -o $(TEST)/$*.o
	ar rcs $(TEST)/lib$*.a $(TEST)/$*.o
	rustc -gL $(TEST)/ -lour_code:$* $(RT)/start.rs -o $(TEST)/$*.run

.PHONY: test
test:
	cargo build
	cargo test --test all_tests

.PHONY: clean
clean:
	rm -f $(TEST)/*.a $(TEST)/*.s $(TEST)/*.run $(TEST)/*.o
	rm -f $(TEST)/*/*.a $(TEST)/*/*.s $(TEST)/*/*.run $(TEST)/*/*.o
