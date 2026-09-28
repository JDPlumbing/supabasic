# supabasic

A lightweight Rust client and infrastructure library for working with [Supabase](https://supabase.com/).

`supabasic` provides reusable Supabase HTTP client infrastructure for Rust applications without tying the client to a specific application domain.

Created by [JDPlumbing](https://github.com/JDPlumbing).

## Installation

Add `supabasic` to your `Cargo.toml`:

```toml
[dependencies]
supabasic = { git = "https://github.com/JDPlumbing/supabasic.git" }
```

## Configuration

`supabasic` can be initialized from environment variables:

```env
SUPABASE_URL=your_supabase_url
SUPABASE_ANON_KEY=your_anon_key
SUPABASE_SERVICE_ROLE_KEY=your_service_role_key
```

Then create a client:

```rust
use supabasic::Supabase;

fn main() -> anyhow::Result<()> {
    let supabase = Supabase::new_from_env()?;

    // Use the client with your application's
    // domain-specific persistence layer.

    Ok(())
}
```

A client can also be constructed explicitly:

```rust
use supabasic::Supabase;

let supabase = Supabase::new(
    "https://your-project.supabase.co",
    "your-anon-key",
    "your-service-role-key",
);
```

## Design

`supabasic` is intentionally domain-agnostic.

It provides generic Supabase infrastructure while applications and domain crates define their own:

- database rows
- queries
- persistence adapters
- domain models
- business logic

This keeps Supabase connectivity reusable without coupling the library to any particular application.

## Status

`supabasic` is currently under active development and its API may change.

## License

License information has not yet been specified.