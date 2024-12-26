features := asynchronous

.PHONY: all
all: check test

.PHONY: check
check: $(addprefix check-,$(features))
	cargo clippy -- -D warnings
	cargo clippy --all-features -- -D warnings
	cargo fmt --all -- --check

.PHONY: $(addprefix check-,$(features))
$(addprefix check-,$(features)): check-%:
	cargo clippy --features $* -- -D warnings

.PHONY: test
test: $(addprefix test-,$(features))
	cargo test
	cargo test --all-features

.PHONY: $(addprefix test-,$(features))
$(addprefix test-,$(features)): test-%:
	cargo build --features $*
