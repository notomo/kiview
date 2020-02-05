
KIVIEW_LOG_LEVEL ?= debug
test:
	$(MAKE) build
	THEMIS_VIM=nvim THEMIS_ARGS="-e -s --headless" themis

build:
	$(MAKE) -C src build

start:
	$(MAKE) -C src start

.PHONY: test
.PHONY: build
.PHONY: start
