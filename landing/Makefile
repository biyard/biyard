PORT ?= 4000

.PHONY: run node_module
node_module:
	pnpm i -p

run: node_module
	PORT=$(PORT) pnpm dev

test: node_modules
	$(PLAYWRIGHT_ENV) npx playwright test $(PLAYWRIGHT_ARGS)
