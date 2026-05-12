
pub mod persistence;
pub mod store;
pub mod cli;
pub mod errors;
pub mod app;
pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/persistent_store.rs"));
}


#[cfg(test)]
mod persistence_test;