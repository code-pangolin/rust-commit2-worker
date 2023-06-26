pub mod results;

use std::path::Path;

use super::datastore::{
    error::DataStoreError,
    query::{Entry, Results},
    Datastore, Read, Write,
};

struct RocksDS {
    db: rocksdb::DB,
}

impl RocksDS {
    pub fn new_datastore<P: AsRef<Path>>(
        opts: &rocksdb::Options,
        path: P,
    ) -> Result<Self, DataStoreError> {
        let db = rocksdb::DB::open(opts, path).map_err(|e| DataStoreError::Unknown(e.into()))?;
        Ok(Self { db })
    }
}

impl Datastore for RocksDS {
    fn sync(&self, _prefix: super::datastore::Key) -> Result<(), DataStoreError> {
        self.db
            .flush()
            .map_err(|e| DataStoreError::Unknown(e.into()))
    }

    fn close(&self) -> Result<(), DataStoreError> {
        self.db
            .flush()
            .map_err(|e| DataStoreError::Unknown(e.into()))
    }
}

impl Read for RocksDS {
    fn get(&self, key: &super::datastore::Key) -> Result<Vec<u8>, DataStoreError> {
        match self.db.get(&key.0) {
            Ok(v) => match v {
                Some(v) => Ok(v),
                None => Err(DataStoreError::Redaction(key.0.clone())),
            },
            Err(e) => Err(DataStoreError::ReadError(e.into())),
        }
    }

    fn has(&self, key: &super::datastore::Key) -> Result<bool, DataStoreError> {
        match self.db.get_pinned(&key.0) {
            Ok(v) => match v {
                Some(_) => Ok(true),
                None => Ok(false),
            },
            Err(e) => Err(DataStoreError::ReadError(e.into())),
        }
    }

    fn get_size(&self, key: &super::datastore::Key) -> Result<usize, DataStoreError> {
        match self.db.get_pinned(&key.0) {
            Ok(v) => match v {
                Some(v) => Ok(v.len()),
                None => Ok(0),
            },
            Err(e) => Err(DataStoreError::ReadError(e.into())),
        }
    }
    // only list is implmented
    fn query(
        &self,
        query: super::datastore::query::Query,
    ) -> std::result::Result<Box<dyn Results>, DataStoreError> {
        let res = self.db.prefix_iterator(&query.prefix);

        let mut results = Vec::<anyhow::Result<Entry>>::new();

        for v in res {
            match v {
                Err(e) => {
                    let result = Err(e.into());
                    results.push(result);
                }
                Ok(v) => {
                    let entry = Entry {
                        key: super::datastore::Key(
                            String::from_utf8(v.0.to_vec()).unwrap_or_default(),
                        ),
                        value: v.1.to_vec(),
                        expiration: 0,
                        size: v.1.len(),
                    };
                    let result = Ok(entry);
                    results.push(result);
                }
            }
        }

        return Ok(Box::new(results::RocksdbResults {
            results,
            query,
            cur: 0,
        }));
    }
}

impl Write for RocksDS {
    fn put(&self, key: &super::datastore::Key, value: &Vec<u8>) -> Result<(), DataStoreError> {
        self.db
            .put(key.to_string(), value)
            .map_err(|e| DataStoreError::WriteError(e.into()))
    }

    fn delete(&self, key: &super::datastore::Key) -> Result<(), DataStoreError> {
        self.db
            .delete(key.to_string())
            .map_err(|e| DataStoreError::ReadError(e.into()))
    }
}
