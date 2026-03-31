---
globs: ["api/**/*.rs"]
---

# Backend API Conventions

## API Versioning

- `/v1/*` — DynamoDB-backed public endpoints
- `/m1/*` — System admin endpoints

## Axum + Askama Architecture

- **REST APIs**: Axum handlers for JSON API endpoints
- **SSR Pages**: Askama templates for SEO-critical pages (landing, public profiles)
- Routing is split: API endpoints vs SSR pages use separate routers

## Controller Path Convention

Controller file paths must mirror the URL path (excluding path parameters).

Example: handlers for `/v1/projects/:project-id/tokens` go in:
`api/src/controllers/v1/projects/tokens/*.rs`

```
api/src/controllers/
├── mod.rs
├── v1/
│   ├── mod.rs
│   ├── accounts/
│   ├── credentials/
│   └── projects/
│       ├── create_project.rs
│       ├── get_project.rs
│       ├── points/
│       └── tokens/
│           ├── create_token.rs
│           └── mod.rs
└── console/          # SSR templates
```

## Feature Organization

Each business domain lives under `api/src/features/<feature-name>/`:

```
api/src/features/<feature-name>/
├── models/
│   ├── mod.rs
│   └── <model_name>.rs    # DynamoDB model (DynamoEntity derive)
├── dto/
│   ├── mod.rs
│   └── <dto_name>.rs      # Request/response types
├── types/
│   ├── mod.rs
│   └── <type_name>.rs     # Domain types
└── utils/
    ├── mod.rs
    └── <util>.rs           # Feature-specific utilities
```

## Error Handling

- Return proper HTTP status codes: 400 (validation), 404 (not found), 500 (internal)
- Handle DynamoDB errors: `ConditionalCheckFailed`, `ResourceNotFound`
- Log errors with `btracing` wrapper
- Provide clear error messages for API consumers

## Multi-Tenancy

- Include `project_id` in partition keys for data isolation
- Validate project ownership before operations
- Extract/verify user identity from request context
- Prevent cross-project data leakage in queries
