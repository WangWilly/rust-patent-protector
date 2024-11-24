use envy::{prefixed, Result};
use serde::de::DeserializeOwned;

////////////////////////////////////////////////////////////////////////////////

pub fn get_env<T>(prefix: &'static str) -> Result<T>
where
    T: DeserializeOwned,
{
    prefixed(prefix).from_env::<T>()
}
