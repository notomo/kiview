
build:
	cargo build

release_build:
	cargo build --release

start:
	$(MAKE) build
	./target/debug/kiview do --arg ${ARG}

# FIXME
test:
	$(MAKE) -C ..

.PHONY: build
.PHONY: release_build
.PHONY: start
.PHONY: test
