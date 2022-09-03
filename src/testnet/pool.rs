use once_cell::sync::Lazy;
use std::sync::{
    RwLock,
};
// use std::thread::JoinHandle;
use std::process::Child;
use std::collections::HashMap;

// Does this need Arc?
// Should this be statically defined?
#[allow(dead_code)]
pub static TESTNETS: Lazy<RwLock<HashMap<String, (Child, u32)>>> = Lazy::new(||{
    RwLock::new(HashMap::new())
});
