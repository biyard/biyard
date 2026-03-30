---
globs: ["api/**/*.rs", "packages/by-macros/**/*.rs"]
---

# DynamoDB Patterns

## Single-Table Design

Uses composite keys with prefixes for entity identification:
- **Partition Key (pk)**: `PROJECT#<id>`, `USER#<id>`, `TOKEN#<id>`, etc.
- **Sort Key (sk)**: Entity type or composite (e.g., `METADATA#<id>`, `TOKEN#<token_id>`)
- **GSIs**: `gsi1` through `gsi6` for alternative access patterns
- **Table prefix**: Set via `DYNAMO_TABLE_PREFIX` env var at **compile time**

## DynamoEntity Derive Macro

`DynamoEntity` (from `by-macros`) generates CRUD functions for DynamoDB interaction.

### Structure Attributes

| Attribute    | Description                          | Default               |
|:-------------|:-------------------------------------|:----------------------|
| `table`      | Table name suffix (after prefix)     | `main`                |
| `result_ty`  | Result type                          | `std::result::Result` |
| `error_ctor` | Error constructor                    | `create::Error2`      |
| `pk_name`    | Partition key field name             | `pk`                  |
| `sk_name`    | Sort key field name (omit to remove) | `sk`                  |

### Field Attributes

| Attribute | Description                          |
|:----------|:-------------------------------------|
| `prefix`  | Prefix for indexed value             |
| `index`   | GSI name (e.g., `gsi1`, `gsi2`)     |
| `pk`      | Mark as partition key of the index   |
| `sk`      | Mark as sort key of the index        |
| `name`    | Generated query function name        |

### Example

```rust
use bdk::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity)]
pub struct EmailVerification {
    pub pk: String,
    pub sk: String,

    #[dynamo(prefix = "TS", index = "gsi2", sk)]
    pub created_at: i64,

    #[dynamo(prefix = "EMAIL", name = "find_by_email_and_code", index = "gsi1", pk)]
    pub email: String,

    #[dynamo(index = "gsi1", sk)]
    #[dynamo(index = "gsi2", name = "find_by_code", pk)]
    pub value: String,

    pub expired_at: i64,
    pub attemp_count: i32,
}
```

This generates:
- `EmailVerification::find_by_email_and_code(email, code)` — queries `gsi1` where `gsi1_pk = EMAIL#<email>`, `gsi1_sk = <value>`
- `EmailVerification::find_by_code(code)` — queries `gsi2` where `gsi2_pk = <value>`, `gsi2_sk = TS#<created_at>`

### Key Construction

- If `prefix = "EMAIL"` and field value is `a@example.com`, the indexed value becomes `EMAIL#a@example.com`
- If no prefix is set, the raw field value is used
- PK/SK fields (`pk`, `sk`) use `format!("PREFIX#{}", value)` in constructors

## DynamoEnum Derive

Use `DynamoEnum` for enum types stored in DynamoDB fields. Provides automatic serialization/deserialization.

## Best Practices

- Add GSIs only when query patterns require them
- Use sparse indexes (attribute existence) for filtering
- Store timestamps as Unix epoch (`i64`) for range queries
- Use `transact_write!` for multi-item atomic operations
- Design partition keys for even distribution (avoid hot partitions)
