pub mod error;
pub mod query;

use std::fmt::Display;

use self::{
    error::DataStoreError,
    query::{Query, Results},
};

pub trait Datastore: Read + Write {
    // Sync guarantees that any Put or Delete calls under prefix that returned
    // before Sync(prefix) was called will be observed after Sync(prefix)
    // returns, even if the program crashes. If Put/Delete operations already
    // satisfy these requirements then Sync may be a no-op.
    //
    // If the prefix fails to Sync this method returns an error.
    fn sync(&self, prefix: Key) -> Result<(), DataStoreError>;
    fn close(&self) -> Result<(), DataStoreError>;
}

pub trait Read {
    fn get(&self, key: &Key) -> Result<Vec<u8>, DataStoreError>;
    fn has(&self, key: &Key) -> Result<bool, DataStoreError>;
    fn get_size(&self, key: &Key) -> Result<usize, DataStoreError>;
    //only list is implmented
    fn query(&self, query: Query) -> Result<Box<dyn Results>, DataStoreError>;
}

pub trait Write {
    fn put(&self, key: &Key, value: &Vec<u8>) -> Result<(), DataStoreError>;
    fn delete(&self, key: &Key) -> Result<(), DataStoreError>;
}

#[derive(Debug, Clone, Default)]
pub struct Key(pub String);

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// impl<T: ToString + !fmt::Display> From<T> for Key {
//     fn to_string(&self) -> String {
//         self.0
//     }
// }
