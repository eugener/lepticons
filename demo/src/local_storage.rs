use std::str::FromStr;

use web_sys::window;
use web_sys::Storage;

/// Wrapper around the browser's local storage.
pub struct LocalStorage;

impl LocalStorage {
    fn instance() -> Option<Storage> {
        window()?.local_storage().ok()?
    }

    /// Returns the value for the key.
    pub fn get<T: FromStr>(key: &str) -> Option<T> {
        LocalStorage::instance()
            .and_then(|s| s.get_item(key).ok()?)
            .and_then(|v| v.parse::<T>().ok())
    }

    /// Sets the value for the key.
    pub fn set<T: ToString>(key: &str, value: &T) -> Option<()> {
        LocalStorage::instance().and_then(|s| {
            s.set_item(key, &value.to_string()).ok()
        })
    }
}
