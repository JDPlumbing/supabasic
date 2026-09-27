# Supabasic Module

The `supabasic` module is a lightweight, async Supabase REST client and data access layer. It provides simple CRUD operations, query building, and ORM-like traits for the core domain tables used by Omnivox (worlds, simulations, events, entities, users, addresses, etc.).

## Features

- Environment-driven configuration (`SUPABASE_URL`, `SUPABASE_KEY`).
- Fluent query builder for `select`, `insert`, `update`, `delete`, filters, and ordering.
- `DbModel` trait for mapping structs to tables with generic list/create/update helpers.
- Convenience modules for common entities.
- Error type covering HTTP, JSON, and Supabase-specific errors.

## Structure

- **mod.rs** — Re-exports `admin` and `core`.
- **core/** — Foundational client, ORM, and error types.
  - `client.rs` — `Supabase` client and `QueryBuilder`.
  - `orm.rs` — `DbModel` trait and generic CRUD helpers.
  - `error.rs` — `SupabasicError`.
- **admin/** — Higher-level admin-oriented data modules (companies, contacts, content, jobs, locations, media, pages, scheduled_events, etc.).
- **_dep/** — Legacy entity mappings (see its README).

## Key Types

- `Supabase` — The main client. Use `new_from_env()` or `from(url, key)`.
- `QueryBuilder` — Returned by `supabase.from("table")`.
- Row structs (e.g., `WorldRow`, `Entity`) that implement `DbModel`.

## Example

```rust
let supa = Supabase::new_from_env()?;

// List worlds
let worlds = supa.from("worlds").select("*").execute().await?;

// Using DbModel helpers (if implemented for the row type)
let new_world = NewWorld { ... };
let created = WorldRow::create(&supa, &new_world).await?;
```

## Subpackages

- `supabasic::core` — Client + generic ORM.
- `supabasic::admin` — Admin resource modules (companies, contacts, content/blocks, jobs, etc.).

## See Also

- [src/api/](../api/) — API handlers and routers that call into Supabase via this client.
- [src/shared/](../shared/) — Higher-level source traits that may wrap or complement supabasic access.

---
**Maintainer:** drippy  
**License:** MIT
