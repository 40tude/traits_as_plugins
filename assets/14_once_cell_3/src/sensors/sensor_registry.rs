// sensor_registry.rs RwLock

use std::collections::HashMap;
use std::sync::RwLock;

// Generic registry for any trait object `dyn T`.
// NOTE: `T` is unsized because trait objects are unsized.
// We store function pointers that construct `Box<dyn T>`.
pub struct Registry<T: ?Sized + 'static> {
    // map: RwLock<HashMap<&'static str, fn() -> Box<T>>>,
    map: RwLock<HashMap<String, fn() -> Box<T>>>, //String not &str
}

// TODO : Duplicated names are not yet handled
impl<T: ?Sized + 'static> Registry<T> {
    // Create an empty registry.
    pub fn new() -> Self {
        Self { map: RwLock::new(HashMap::new()) }
    }

    // Register a named constructor.
    // It now accept &str and String (they may come from dB, file...)
    pub fn register_sensor<S: Into<String>>(&self, name: S, ctor: fn() -> Box<T>) -> bool {
        let mut map = self.map.write().expect("REGISTRY RwLock poisoned");
        // map.insert(name, ctor).is_none() // returns false if it replaced an existing one
        map.insert(name.into(), ctor).is_none() // returns false if it replaced an existing one
    }

    // Build an instance by name.
    pub fn make_sensor(&self, name: &str) -> Option<Box<T>> {
        let map = self.map.read().expect("REGISTRY RwLock poisoned");
        map.get(name).map(|ctor| ctor())
    }
}

// Useful for debugging / introspection / diag
// pub fn keys(&self) -> Vec<&'static str> {
//     let map = self.map.read().expect("poisoned");
//     map.keys().copied().collect()
// }
