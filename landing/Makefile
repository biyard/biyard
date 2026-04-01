PORT ?= 4000
API_URL ?= http://localhost:3000
CONSOLE_URL ?= http://localhost:4000

.PHONY: run node_modules
node_modules:
	pnpm i -p

run: node_modules
	PORT=$(PORT) VITE_API_BASE_URL=$(API_URL) VITE_CONSOLE_BASE_URL=$(CONSOLE_URL) pnpm dev

test: node_modules
	$(PLAYWRIGHT_ENV) npx playwright test $(PLAYWRIGHT_ARGS)

build: node_modules
	VITE_API_BASE_URL=$(API_URL) pnpm run build
