# AGENT.md

Read `CLAUDE.md` first. This file is a short, persistent handoff summary.

## Canonical Rules
- User-facing term is **Brand** (브랜드).
- User-facing term for multi-tenant container is **Enterprise** (기업).
- Internal domain term stays **Project** in code, routes, and data model.
- Internal domain term is **Enterprise** in code, types, and URL slugs.
- Do not rename routes/modules/types from `project` unless user asks for full migration.
- Do not reintroduce "Organization" as a parallel concept.
- Interpret **Brand == Project** (presentation alias) in current scope.

## Tenancy/RBAC Direction
- Product is B2B.
- Membership belongs to **Enterprise** (not directly to Project).
- Target relations: `User -< Membership >- Enterprise -< Project`.
- RBAC at Enterprise scope: `Owner`, `Admin`, `Viewer`.

## Product Constraints
- Token name/symbol are immutable after issuance.
- Each Project has **at most one Token** (1:1).
- Prioritize safe UI/feature iteration over immediate deep schema migration.

## If Unsure
- Preserve existing backend/data compatibility.
- Apply terminology changes in UI/i18n first.
- Document any architecture-changing proposal before implementation.
