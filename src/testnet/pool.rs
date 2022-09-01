use once_cell::sync::Lazy;
use std::sync::RwLock;
use std::collections::HashMap;
use std::thread::JoinHandle;

// #[allow(dead_code)]
// pub static TESTNETS: Lazy<Mutex<HashMap<ThreadId, Thread>>> = Lazy::new(|| {Mutex::new(HashMap::new())});

// Does this need Arc?
// Should this be statically defined?
#[allow(dead_code)]
pub static TESTNETS: Lazy<RwLock<HashMap<String, JoinHandle<String>>>> = Lazy::new(||{
    RwLock::new(HashMap::new())
});
