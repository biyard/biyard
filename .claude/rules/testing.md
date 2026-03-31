---
globs: ["api/**/tests.rs", "api/src/tests/**"]
---

# Testing Conventions

## Test File Location

Place test files at `api/src/controllers/v1/<feature>/tests.rs` within the corresponding controller module.

## TestContext Setup

```rust
#[tokio::test]
async fn test_example() {
    let TestContext { app, app_state, now, ddb, account1, account2, admin, .. } = TestContext::setup().await;
    // test code
}
```

Fields:
- `app` — Application instance
- `app_state` — Shared app state
- `now` — Current timestamp for unique test data
- `ddb` — DynamoDB client for direct DB operations
- `account1`, `account2` — Test user tuples `(Account, HeaderMap)`
- `admin` — Admin user tuple

## HTTP Request Macros

Macros from `rest-api` package: `get!`, `post!`, `put!`, `patch!`, `delete!`

**Parameters** (in order):
1. `app:` — App instance from TestContext
2. `path:` — API endpoint path (e.g., `"/v1/projects/{id}"`)
3. `headers:` (optional) — HTTP headers for authentication
4. `body:` (optional) — Request body as JSON literal
5. `response_type:` (optional) — Expected response type (default: `serde_json::Value`)

**Returns:** `(StatusCode, HeaderMap, ResponseBody)`

### Examples

```rust
// GET with auth
let (status, _headers, body) = get! {
    app: app,
    path: format!("/v1/projects/{}", project_id),
    headers: account1.1.clone(),
    response_type: ProjectResponse
};
assert_eq!(status, 200);

// POST with body
let (status, _headers, body) = post! {
    app: app,
    path: "/v1/projects",
    headers: account1.1.clone(),
    body: {
        "name": "Test Project",
        "description": "A test project"
    },
    response_type: CreateProjectResponse
};
assert_eq!(status, 200);

// GET without auth
let (status, _headers, body) = get! {
    app: app,
    path: format!("/v1/projects/{}", project_id),
    response_type: ProjectResponse
};
```

## Best Practices

- Test each handler: success cases, error cases, edge cases
- Test with and without authentication
- Verify response status codes and body structure
- Use descriptive names: `test_create_project_when_authenticated`
- Run `cargo test` before committing
