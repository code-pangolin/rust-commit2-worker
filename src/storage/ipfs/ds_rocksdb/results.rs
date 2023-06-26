use std::vec::IntoIter;

use anyhow::Result;

use crate::storage::ipfs::datastore::query::{Entry, Results};
pub struct RocksdbResults {
    pub results: Vec<Result<Entry>>,
    pub cur: usize,
    pub query: crate::storage::ipfs::datastore::query::Query,
}

impl Results for RocksdbResults {
    fn query(&self) -> &crate::storage::ipfs::datastore::query::Query {
        return &self.query;
    }
    fn to_vec(&self) -> &Vec<Result<Entry>> {
        return &self.results;
    }
}
