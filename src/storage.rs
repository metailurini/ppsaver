use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Mutex;

pub trait Storage {
    fn init() -> Self;
    fn get(&self, key: String) -> Option<String>;
    fn set(&mut self, key: String, value: String) -> Result<(), Box<dyn Error>>;
    fn close(&mut self);
}

pub struct RMS {
    db: HashMap<String, String>,
}

impl Storage for RMS {
    fn init() -> Self {
        RMS { db: HashMap::new() }
    }

    fn get(&self, key: String) -> Option<String> {
        match self.db.get(key.as_str()) {
            Some(value) => Some(value.to_string()),
            None => None,
        }
    }

    fn set(&mut self, key: String, value: String) -> Result<(), Box<dyn Error>> {
        self.db.insert(key, value);
        Ok(())
    }

    fn close(&mut self) {
        self.db.clear()
    }
}

pub fn get<T>(db: &'static Lazy<Mutex<T>>, key: String) -> Option<String>
where
    T: Storage + 'static,
{
    let d = match db.lock() {
        Ok(d) => d,
        Err(_) => {
            return None;
        }
    };
    d.get(key)
}

pub fn set<T>(x: &'static Lazy<Mutex<T>>, key: String, value: String) -> Result<(), Box<dyn Error>>
where
    T: Storage + 'static,
{
    x.lock()?.set(key, value)
}
