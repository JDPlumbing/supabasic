# Supabasic Core

Core Supabase client, generic ORM, and error types.

## Contents

- **client.rs** — `Supabase` client and `QueryBuilder`.
- **orm.rs** — `DbModel` trait and generic list/create/update helpers.
- **error.rs** — `SupabasicError`.

This is the foundation that `supabasic::admin::*` and any direct callers build upon.

## See Also

- [src/supabasic/](../README.md)
- [src/supabasic/_dep/](../_dep/) — Older entity-specific mappings.

---
**Maintainer:** drippy  
**License:** MIT
