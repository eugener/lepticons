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
            .map(|s| s.get_item(key).ok()?)
            .flatten()
            .map(|v| v.parse::<T>().ok())
            .flatten()
    }

    /// Sets the value for the key.
    pub fn set<T: ToString>(key: &str, value: &T) -> Option<()> {
        LocalStorage::instance().and_then(|s| {
            let value = value.to_string();
            s.set_item(key, &value).ok()
        });
        Some(())
    }
}
