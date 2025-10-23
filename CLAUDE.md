# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Biyard is a Launchpad-like SaaS and PaaS platform that enables users to create projects and manage points and tokens over blockchain through our PaaS APIs. Built as a full-stack monorepo with a Rust backend and React frontend, currently undergoing a DynamoDB.

**Repository:** https://github.com/biyard/biyard

### Platform Purpose
- **SaaS/PaaS Platform:** Launchpad-style service for blockchain projects
- **User Projects:** Users create and manage their own projects on the platform
- **PaaS APIs:** Services consume Biyard APIs to manage points and tokens on blockchain
- **Target Users:** Project creators who need blockchain token/point management infrastructure

## Monorepo Structure

This is a hybrid Rust + TypeScript monorepo:
- **Rust workspace:** Managed by Cargo with workspace members in `Cargo.toml`
- **Node workspace:** Managed by pnpm with workspace configuration in `pnpm-workspace.yaml`

```
biyard/
├── api/              # Rust backend (Axum for REST APIs, Askama for SSR/SEO)
├── web/              # React frontend (Vite + TypeScript CSR)
└── packages/         # Shared Rust libraries (planned, not yet created)
    ├── btracing/     # Custom tracing wrapper (v0.1.*)
    ├── by-axum/      # Custom Axum framework wrapper (v0.2.*)
    └── by-macros/    # Procedural macros (v0.6.*)
```

## Development Methodology

**Feature-Driven Development:** The codebase is organized around features rather than technical layers. When implementing new functionality:
- Group related components, hooks, API calls, and types by feature
- Each feature should be self-contained and independently testable
- Organize code by business domain (e.g., `features/projects/`, `features/tokens/`, `features/points/`)

## Development Commands

### Backend (Rust)

```bash
# Build the API server
cd api
cargo build

# Run the API server
cargo run

# Run tests
cargo test

# Check code without building
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy
```

### Frontend (React + Vite)

```bash
cd web

# Install dependencies
pnpm install

# Development server (with HMR)
pnpm dev
# Or using Makefile:
make run

# Build for production
pnpm build

# Lint code
pnpm lint

# Preview production build
pnpm preview

# Run tests (Playwright)
make test
```

### Root Level

```bash
# Install all workspace dependencies
pnpm install

# Build everything
cargo build && cd web && pnpm build
```

## Technology Stack

### Backend
- **Language:** Rust (Edition 2024)
- **Async Runtime:** Tokio 1.40.0 (full features)
- **Web Framework:** Axum (via custom `by-axum` wrapper)
- **Template Engine:** Askama (for SSR pages requiring SEO optimization)
- **Database:** DynamoDB (AWS SDK for Rust)
- **Serialization:** Serde + Serde JSON
- **Schema:** Schemars (for OpenAPI/JSON schema generation)
- **HTTP Client:** Reqwest (blocking, JSON, multipart)
- **Tracing:** `tracing` + `tracing-subscriber` (via custom `btracing` wrapper)

### Frontend
- **Language:** TypeScript 5.9.3 (strict mode)
- **Framework:** React 19.1.1 (Client-Side Rendering)
- **Build Tool:** Vite 7.1.7
- **Styling:** TailwindCSS
- **UI Components:** Shadcn UI + Radix UI primitives
- **Component Documentation:** Storybook
- **Data Fetching:** React Query (TanStack Query)
- **Package Manager:** pnpm 10.18.2+
- **Linting:** ESLint 9.36.0 with TypeScript support
- **Testing:** Playwright (end-to-end web testing)

### Rendering Strategy
- **CSR (Client-Side Rendering):** React app served by Vite for interactive UI
- **SSR (Server-Side Rendering):** Askama templates from Rust backend for SEO-critical pages (landing pages, public profiles, etc.)

## Custom Internal Packages

The workspace references three custom Rust packages (not yet created):

1. **`btracing`** (`packages/btracing/`)
   - Wrapper around the `tracing` crate
   - Provides opinionated logging/observability setup

2. **`by-axum`** (`packages/by-axum/`)
   - Custom Axum web framework wrapper
   - Likely includes middleware, routing, and handler utilities
   - Used by the `api/` service

3. **`by-macros`** (`packages/by-macros/`)
   - Procedural macros for code generation
   - May include routing macros, schema generation, etc.

## API Versioning Pattern

The backend will serve two API versions simultaneously:
- URL pattern: `/v1/*` for DynamoDB-backed endpoints
- Use separate route modules/handlers for each version
- Ensure proper database client initialization for each version

## Frontend Development Notes

### React + Vite Setup
- Uses Vite with React Fast Refresh (HMR)
- TypeScript strict mode enabled (`tsconfig.app.json`)
- ESLint configured with React Hooks rules
- Environment variables: Set `VERSION` and `PORT` when running dev server
- Build output goes to `web/dist/`

### UI Development Workflow
- **TailwindCSS:** Use utility classes for styling; avoid custom CSS when possible
- **Shadcn/Radix:** Leverage pre-built accessible components from Shadcn UI library
- **Storybook:** Document and develop components in isolation
  - Run Storybook: `pnpm storybook` (if configured)
  - Build Storybook: `pnpm build-storybook` (if configured)
- **React Query:** Use for all server state management
  - Queries: `useQuery` for fetching data
  - Mutations: `useMutation` for POST/PUT/DELETE operations
  - Invalidation: Properly invalidate caches after mutations

### Feature Organization
When creating new features:
```
web/src/features/<feature-name>/
├── components/       # Feature-specific React components
├── hooks/           # Custom hooks for this feature
├── api/             # React Query hooks and API calls
├── types/           # TypeScript types/interfaces
└── utils/           # Feature-specific utilities
```

## Code Quality Standards

### TypeScript
- Strict type checking enabled
- No unused variables or parameters
- Consistent use of `import.meta.env` for environment variables

### Rust
- Use workspace-level dependency versions
- Edition 2024 features available
- Prefer async/await with Tokio runtime

## Backend Development Notes

### Axum + Askama Architecture
- **REST APIs:** Use Axum handlers for JSON API endpoints (`/v1/*`, `/m1/*`)
- **SSR Pages:** Use Askama templates for SEO-critical pages (landing, public profiles)
- **Routing:** Separate routers for API endpoints vs SSR pages
- **DynamoDB Integration:** Use AWS SDK for Rust to interact with DynamoDB tables

```
api/src/templates
└── index.html
```

### Controller definition
Controllers path should be same with API path without path parameters
- For example, handlers for `/v1/projects/:project-id/tokens` should be placed in `api/src/controllers/v1/projects/tokens/*.rs`

```
api/src/controllers/
├── mod.rs
├── v1
|   ├── mod.rs
|   └── {model_name}.rs  # DynamoDB model
├── dto
|     ├── mod.rs
|     └── {dto_name}.rs  # Request and response types for handlers
├── types
|     ├── mod.rs
|     └── {type_name}.rs  # Types used by DynamoDB or handlers
└── utils
      ├── mod.rs
      └── {util_namespace}.rs  # Utilities for the feature
```

### Feature Organization (Backend)
```
api/src/features/<feature-name>/
├── models
|     ├── mod.rs
|     └── {model_name}.rs  # DynamoDB model
├── dto
|     ├── mod.rs
|     └── {dto_name}.rs  # Request and response types for handlers
├── types
|     ├── mod.rs
|     └── {type_name}.rs  # Types used by DynamoDB or handlers
└── utils
      ├── mod.rs
      └── {util_namespace}.rs  # Utilities for the feature
```

## Build Verification

When implementing v1 APIs:
1. Add DynamoDB client dependencies to `api/Cargo.toml` (e.g., `aws-sdk-dynamodb`)
2. Implement handlers/routes in separate feature modules
3. Update `web/src/features/` to call v1 endpoints using React Query
4. Run `cargo build` to verify backend builds
5. Run `cd web && pnpm build` to verify frontend builds
6. Run tests: `cargo test` (backend) and `make test` (frontend with Playwright)

## Platform-Specific Considerations

### Blockchain Integration
- PaaS APIs will handle blockchain operations for points and tokens
- Ensure proper error handling for blockchain transaction failures
- Consider async processing for blockchain operations (webhooks, callbacks)

### Multi-Tenancy
- Users create projects on the platform
- Each project should be properly isolated in DynamoDB (partition keys)
- API endpoints should validate project ownership/permissions
