---
globs: ["console/**/*.rs"]
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

## ID Generation — Use UUID v7, not v4

**Always use `uuid::Uuid::now_v7()` when generating identifiers for DynamoDB
entities.** Do **not** use `Uuid::new_v4()`.

```rust
// Correct
let id = uuid::Uuid::now_v7().to_string();

// Wrong
let id = uuid::Uuid::new_v4().to_string();
```

**Why:**
- UUID v7 embeds a millisecond Unix timestamp in the high bits, so generated
  IDs are **lexicographically sortable by creation time**. That means
  time-ordered DynamoDB sort keys (e.g., `SK = TXN#<uuid>`) naturally produce
  chronological reads without a separate `created_at` range key.
- Insert patterns stay adjacent in the B-tree, which is friendlier to
  DynamoDB's internal partitioning than v4's fully random layout.
- Debuggability: you can eyeball two IDs and tell which came first.

**Cargo feature:** `console/Cargo.toml` enables the `v7` feature on the `uuid`
crate, not `v4`. Do not add `v4` back to the feature list.

**Scope:** this applies to every new entity ID in `console/` (projects,
credentials, accounts, point transactions, S3 object keys, etc.). Existing v4
IDs already written to DynamoDB stay valid — only new generation sites must
use v7.

## Transactional Writes — Multi-Item Atomicity

**Any handler that performs more than one DynamoDB write must wrap those
writes in a single DynamoDB transaction.** Sequential `.create()` /
`.update()` / `.delete()` / `.execute()` calls are forbidden when two or
more writes belong to the same logical operation, because a partial
failure (first write succeeds, second fails) leaves the database in an
inconsistent state with no automatic recovery.

Use the macros from [console/src/common/macros/dynamo.rs](../../console/src/common/macros/dynamo.rs):

| Macro                                  | When to use                                                                |
|:---------------------------------------|:---------------------------------------------------------------------------|
| `transact_write!(cli, a, b, c)`        | Fixed, known set of items (≤100). Most handlers.                           |
| `transact_write_items!(cli, vec)`      | Dynamically built `Vec<TransactWriteItem>` whose length is bounded ≤100.   |
| `transact_write_all_items!(cli, vec)`  | Vec may exceed 100 items. Chunks into 100-item batches (not atomic across chunks). |
| `transact_write_all_items_with_failover!(cli, vec)` | Same as above plus per-chunk retry. Use only when transient failures are expected. |

All four macros are `#[cfg(feature = "server")]` and exported at the
crate root, so call sites use `crate::transact_write!(...)`.

### Building TransactWriteItem values

`DynamoEntity` (from `by-macros`) generates these helpers on every
entity. Prefer them over the standalone `.create()` / `.update()` /
`.delete()` async methods when building a transaction:

| Method                                                  | Semantics                                                  |
|:--------------------------------------------------------|:-----------------------------------------------------------|
| `entity.create_transact_write_item()`                   | Conditional put — fails if pk/sk already exists.           |
| `entity.upsert_transact_write_item()`                   | Unconditional put.                                         |
| `Entity::delete_transact_write_item(pk, sk)`            | Conditional delete — fails if key does not exist.          |
| `Entity::updater(pk, sk).with_x(...).transact_write_item()`   | Conditional update — fails if key does not exist. Mirrors `.execute()`. |
| `Entity::updater(pk, sk).with_x(...).transact_upsert_item()`  | Unconditional update — creates the item if missing.        |

### Pattern

```rust
// WRONG — partial failure leaves enterprise without an account
enterprise.create(cli).await?;
account.create(cli).await?;

// CORRECT — atomic
crate::transact_write!(
    cli,
    enterprise.create_transact_write_item(),
    account.create_transact_write_item(),
)?;
```

```rust
// Mixed create + update in one transaction
let invitation_update = Invitation::updater(invitation.pk.clone(), invitation.sk.clone())
    .with_status(InvitationStatus::Accepted)
    .with_updated_at(now);

crate::transact_write!(
    cli,
    account.create_transact_write_item(),
    invitation_update.transact_write_item(),
)?;
```

### Allowed exceptions

- **Single-write handlers.** Plain `entity.create(cli).await?` is fine
  and preferred for readability when a handler writes exactly one item.
  Do not wrap a single write in `transact_write!` for "consistency".
- **Reads between writes are not an excuse to skip the transaction.**
  Do all reads first, build every `TransactWriteItem` from the read
  results, then issue one `transact_write!` at the end of the handler.
- **Writes across different logical operations** (e.g., a write plus an
  audit log written by separate middleware) do not need to share a
  transaction.

### DynamoDB transaction limits to keep in mind

- ≤100 items per `TransactWriteItems` call. Use `transact_write_all_items!`
  to chunk if you may exceed this — but note that atomicity is per-chunk,
  not across the whole vector.
- ≤4 MB total payload.
- A single transaction cannot include two operations on the same item
  (same pk + sk). Coalesce updates before building the items.

## Best Practices

- Add GSIs only when query patterns require them
- Use sparse indexes (attribute existence) for filtering
- Store timestamps as Unix epoch (`i64`) for range queries
- Design partition keys for even distribution (avoid hot partitions)
