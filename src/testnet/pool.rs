use once_cell::sync::Lazy;
use std::sync::{
    RwLock,
    Mutex,
    mpsc::Sender,
};
use std::thread::JoinHandle;
use std::collections::HashMap;

// Does this need Arc?
// Should this be statically defined?
#[allow(dead_code)]
pub static TESTNETS: Lazy<RwLock<HashMap<String, (Mutex<Sender<()>>, JoinHandle<String>)>>> = Lazy::new(||{
    RwLock::new(HashMap::new())
});
