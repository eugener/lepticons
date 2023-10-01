use std::str::FromStr;

use leptos::window;
use web_sys::Storage;

pub struct LocalStorage;

impl LocalStorage {
    fn instance() -> Option<Storage> {
        window().local_storage().ok()?
    }

    pub fn get<T: FromStr>(key: &str) -> Option<T> {
        LocalStorage::instance()
            .map(|s| s.get_item(key).ok()?)
            .flatten()
            .map(|v| v.parse::<T>().ok())
            .flatten()
    }

    pub fn set<T: ToString>(key: &str, value: &T) -> Option<()> {
        LocalStorage::instance().and_then(|s| {
            let value = value.to_string();
            s.set_item(key, &value).ok()
        });
        Some(())
    }
}
