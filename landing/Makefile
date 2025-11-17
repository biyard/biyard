PORT ?= 4000

.PHONY: run node_modules
node_modules:
	pnpm i -p

run: node_modules
	PORT=$(PORT) pnpm dev

test: node_modules
	$(PLAYWRIGHT_ENV) npx playwright test $(PLAYWRIGHT_ARGS)

build: node_modules
	pnpm run build
