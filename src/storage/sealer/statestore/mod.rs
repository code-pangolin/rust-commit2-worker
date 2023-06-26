pub mod state;

use anyhow::anyhow;
use hyper::body::Buf;
use serde::{de::DeserializeOwned, Serialize};

use self::state::StoredState;
use crate::storage::ipfs::datastore::{
    query::{Query, Results},
    Datastore, Key,
};

// github.com/filecoin-project/go-statestore@v0.2.0/store.goL16
pub struct StateStore<'a, T: Datastore> {
    pub(super) ds: &'a T,
}

impl<'a, T: Datastore> StateStore<'a, T> {
    pub fn new(ds: &'a T) -> Self {
        Self { ds }
    }

    pub fn to_key(ds: impl ToString) -> Key {
        Key(ds.to_string())
    }

    pub fn begin(&self, i: &impl ToString, state: impl Serialize) -> anyhow::Result<()> {
        let key = Key(i.to_string());
        let has = self.ds.has(&key)?;
        if has {
            return Err(anyhow!("already tracking state for {}", &key));
        };

        let mut value = vec![];
        ciborium::into_writer(&state, &mut value)?;
        self.ds.put(&key, &value)?;
        Ok(())
    }

    pub fn get(&self, i: impl ToString) -> Box<StoredState<T>> {
        Box::new(StoredState {
            ds: self.ds,
            name: StateStore::<T>::to_key(i),
        })
    }

    pub fn has(&self, i: impl ToString) -> anyhow::Result<bool> {
        Ok(self.ds.has(&StateStore::<T>::to_key(i))?)
    }

    // File: github.com/filecoin-project/go-statestore@v0.2.0/store.go
    // 65: func (st *StateStore) List(out interface{}) error {

    pub fn list<S>(&self) -> anyhow::Result<Vec<S>>
    where
        S: DeserializeOwned,
    {
        let results = self.ds.query(Query::default())?;

        let mut resultlist = Vec::<S>::new();

        for res in results.vec() {
            match res {
                Ok(entry) => {
                    let item: S = ciborium::from_reader(entry.value.reader())?;
                    resultlist.push(item);
                }
                Err(e) => return Err(anyhow!("{}", e)),
            };
        }

        // loop {
        //     let (res, ok) = results.next();
        //     if !ok {
        //         break;
        //     }

        //     if res.error.is_some() {
        //         let err = res.error.unwrap();
        //         return Err(anyhow!(err));
        //     }

        //     let v: S = ciborium::from_reader(res.entry.value.reader())?;
        //     resultlist.push(v);
        // }

        Ok(resultlist)
    }
}
