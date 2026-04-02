# AGENT.md

Read `CLAUDE.md` first. This file is a short, persistent handoff summary.

## Canonical Rules
- User-facing term is **Brand**.
- Internal domain term stays **Project** for now.
- Do not rename routes/modules/types from `project` unless user asks for full migration.
- Interpret **Brand == Project** in current scope.

## Tenancy/RBAC Direction
- Product is B2B.
- Membership belongs to **Organization** (not directly to Project).
- Target relations: `User -< Membership >- Organization -< Project`.
- RBAC at organization scope: `Owner`, `Admin`, `Viewer`.

## Product Constraints
- Token name/symbol are immutable after issuance.
- Prioritize safe UI/feature iteration over immediate deep schema migration.

## If Unsure
- Preserve existing backend/data compatibility.
- Apply terminology changes in UI/i18n first.
- Document any architecture-changing proposal before implementation.
