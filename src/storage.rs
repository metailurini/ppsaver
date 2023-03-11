use std::collections::HashMap;
use std::error::Error;

pub(crate) trait Storage {
    fn init() -> Self;
    fn get(&self, key: String) -> Option<String>;
    fn set(&mut self, key: String, value: String) -> Result<(), Box<dyn Error>>;
    fn close(&mut self);
}

pub(crate) struct RMS {
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
