PORT ?= 5000
VITE_API_URL ?= http://localhost:3000

.PHONY: run node_modules
node_modules:
	pnpm i -p

run: node_modules
	PORT=$(PORT) VITE_API_URL=$(VITE_API_URL) pnpm dev

test: node_modules
	$(PLAYWRIGHT_ENV) npx playwright test $(PLAYWRIGHT_ARGS)

build: node_modules
	pnpm run build
