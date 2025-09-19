// sensor_registry.rs RwLock
use std::collections::HashMap;
use std::sync::RwLock;

// pub struct Registry<T: ?Sized + 'static> {
//     map: RwLock<HashMap<&'static str, fn() -> Box<T>>>,
//     // Does'nt make sense to support String since Sensor kind are, by design, known at compile time
//     // map: RwLock<HashMap<String, fn() -> Box<T>>>, //String not &str
// }

// Clippy complained about the line `map: RwLock<HashMap<&'static str, fn() -> Box<T>>>,` above
// Let's use an alias
type Ctor<T> = fn() -> Box<T>;

pub struct Registry<T: ?Sized + 'static> {
    map: RwLock<HashMap<&'static str, Ctor<T>>>,
}

// // Extreme version using type aliases
// type Key = &'static str;
// type Ctor<T> = fn() -> Box<T>;
// type RegMap<T> = HashMap<Key, Ctor<T>>;
// type RegLock<T> = RwLock<RegMap<T>>;

// pub struct Registry<T: ?Sized + 'static> {
//     // Clipy complained about the line map: RwLock<HashMap<&'static str, fn() -> Box<T>>>,
//     map: RegLock<T>,
// }

// Clippy asked for a Default implementation
// Manual Default because #[derive(Default)] did not work
// Manual Default => no extra bound on T
impl<T: ?Sized + 'static> Default for Registry<T> {
    fn default() -> Self {
        Self { map: RwLock::new(HashMap::new()) }
    }
}

// TODO : Duplicated names are not yet handled
impl<T: ?Sized + 'static> Registry<T> {
    // Create an empty registry.
    pub fn new() -> Self {
        // Self { map: RwLock::new(HashMap::new()) }
        Self::default()
    }

    // Register a named constructor.
    // It now accept &str and String (they may come from dB, file...)
    // pub fn register_sensor<S: Into<String>>(&self, name: S, ctor: fn() -> Box<T>) -> bool {
    pub fn register_sensor(&self, name: &'static str, ctor: fn() -> Box<T>) {
        let mut map = self.map.write().expect("REGISTRY RwLock poisoned");
        map.insert(name, ctor); // returns false if it replaced an existing one
        // map.insert(name.into(), ctor).is_none() // returns false if it replaced an existing one
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
