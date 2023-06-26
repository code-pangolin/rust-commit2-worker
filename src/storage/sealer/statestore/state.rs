use anyhow::anyhow;
use hyper::body::Buf;
use serde::{de::DeserializeOwned, Serialize};

use crate::storage::ipfs::datastore::{Datastore, Key};

pub struct StoredState<'a, T: Datastore> {
    pub(super) ds: &'a T,
    pub(super) name: Key,
}

impl<'a, T: Datastore> StoredState<'a, T> {
    pub fn mutate<F, S>(&self, mutator: F) -> anyhow::Result<()>
    where
        S: Serialize + DeserializeOwned,
        F: FnOnce(&mut S),
    {
        let mut bytes = self.ds.get(&self.name)?;
        let mut res: S = ciborium::from_reader(bytes.reader())?;

        mutator(&mut res);

        ciborium::into_writer(&res, &mut bytes)?;

        self.ds.put(&self.name, &bytes)?;
        Ok(())

        /*
        File: /Users/coolrc/go/pkg/mod/github.com/filecoin-project/go-statestore@v0.2.0/state.go
        49: func (st *StoredState) Mutate(mutator interface{}) error {
         */
        //update saved cbor bytes
    }

    pub fn get<S>(&self) -> anyhow::Result<S>
    where
        S: DeserializeOwned,
    {
        let bytes = self.ds.get(&self.name)?;
        let res: S = ciborium::from_reader(bytes.reader())?;
        Ok(res)
    }

    pub fn end(self) -> anyhow::Result<()> {
        let has = self.ds.has(&self.name)?;
        if !has {
            return Err(anyhow!("No state for {}", &self.name));
        };
        self.ds.delete(&self.name)?;
        Ok(())
    }
}
