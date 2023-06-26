use serde::Serialize;

use super::Key;

#[derive(Default)]
pub struct Query {
    // namespaces the query to results whose keys have Prefix
    pub prefix: String,
    // filter results. apply sequentially
    pub filters: Vec<Box<dyn Filter>>,
    // order results. apply hierarchically
    pub orders: Vec<Box<dyn Order>>,
    // maximum number of results
    pub limit: usize,
    // skip given number of results
    pub offset: usize,
    // return only keys.
    pub keys_only: bool,
    // return expirations (see TTLDatastore)
    pub return_expirations: bool,
    // always return sizes. If not set, datastore impl can return
    // it anyway if it doesn't involve a performance cost. If KeysOnly
    // is not set, Size should always be set.
    pub returns_sizes: bool,
}

pub trait Filter {
    fn filter(&self, e: &Entry) -> bool;
}

pub trait Order {
    fn compare(&self, a: &Entry, b: &Entry) -> i32;
}

#[derive(Default)]
pub struct Entry {
    pub key: Key,
    // Will be nil if KeysOnly has been passed.
    pub value: Vec<u8>,
    // Entry expiration timestamp if requested and supported (see TTLDatastore).
    pub expiration: u64,
    // Might be -1 if the datastore doesn't support listing the size with KeysOnly
    // or if ReturnsSizes is not set
    pub size: usize,
}

pub trait Results {
    // type Item;
    // type IntoIter: Iterator<Item = Self::Item>;
    // fn into_iter(self) -> Self::IntoIter;
    fn query(&self) -> &Query;
    // fn rest(&self) -> Vec<anyhow::Result<Entry>>;
    fn to_vec(&self) -> &Vec<anyhow::Result<Entry>>;
}
