//! Small conversion helpers for the runtime `Value` type.
//! This keeps `types.rs` focused on the `Value` definition and impls.
use super::types::Value;

/// Convenience function used across the runtime to convert into a `Value`.
pub fn into_value<T>(value: T) -> Value
where
    T: Into<Value>,
{
    value.into()
}
