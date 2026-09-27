use crate::core::client::Supabase;
use crate::core::error::SupabasicError;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use uuid::Uuid;

/// Trait every table struct should implement
pub trait DbModel: Sized + DeserializeOwned + Serialize {
    fn table() -> &'static str;
}

// === Generic CRUD === //

/// Fetch a single row by UUID
pub async fn fetch<T: DbModel>(id: Uuid) -> Result<T, SupabasicError> {
    let supa = Supabase::new_from_env()?;
    let val = supa
        .from(T::table())
        .select("*")
        .eq("id", &id.to_string())
        .single()
        .await?;
    Ok(serde_json::from_value(val)?)
}

/// List all rows in the table

/// Existing list<T>
pub async fn list<T>() -> Result<Vec<T>, SupabasicError>
where
    T: DeserializeOwned + DbModel,
{
    let supa = Supabase::new_from_env()?;
    let raw = supa.from(T::table()).select("*").execute().await?;
    let parsed: Vec<T> = serde_json::from_value(raw)?;
    Ok(parsed)
}

/// NEW: raw_list
pub async fn raw_list(table: &str) -> Result<Value, SupabasicError> {
    let supa = Supabase::new_from_env()?;
    let raw = supa.from(table).select("*").execute().await?;
    Ok(raw)
}

/// Existing insert<T,R>
pub async fn insert<T, R>(payload: &T) -> Result<R, SupabasicError>
where
    T: Serialize + DbModel,
    R: DeserializeOwned,
{
    let supa = Supabase::new_from_env()?;
    let raw = supa
        .from(T::table())
        .insert(serde_json::json!([payload]))
        .select("*")
        .execute()
        .await?;
    let parsed: Vec<R> = serde_json::from_value(raw)?;
    parsed
        .into_iter()
        .next()
        .ok_or_else(|| SupabasicError::Other("Insert returned empty response".to_string()))
}

/// Update a row by UUID (returns the updated row)
pub async fn update<I, O>(id: Uuid, item: &I) -> Result<O, SupabasicError>
where
    I: Serialize,
    O: DbModel + DeserializeOwned,
{
    let supa = Supabase::new_from_env()?;
    let val = supa
        .from(O::table())
        .update(serde_json::to_value(item)?)
        .eq("id", &id.to_string())
        .single()
        .await?;
    Ok(serde_json::from_value(val)?)
}

/// Delete a row by UUID (returns the deleted row, if found)
pub async fn delete<T: DbModel>(id: Uuid) -> Result<T, SupabasicError> {
    let supa = Supabase::new_from_env()?;
    let val = supa
        .from(T::table())
        .delete()
        .eq("id", &id.to_string())
        .single()
        .await?;
    Ok(serde_json::from_value(val)?)
}
