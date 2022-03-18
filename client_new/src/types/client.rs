// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use std::{
    error::Error,
    sync::{Arc, RwLock},
};

use engine::{
    new_runtime::memories::buffer::Buffer,
    vault::{
        BoxProvider, ClientId, DbView, RecordError as EngineRecordError, RecordHint, VaultError as EngineVaultError,
        VaultId,
    },
};

use crate::{
    derive_vault_id,
    procedures::{
        FatalProcedureError, Procedure, ProcedureError, ProcedureOutput, Products, Runner, StrongholdProcedure,
    },
    ClientError, KeyStore, Location, Provider, Store, Vault,
};

pub type VaultError<E> = EngineVaultError<<Provider as BoxProvider>::Error, E>;
pub type RecordError = EngineRecordError<<Provider as BoxProvider>::Error>;

pub struct Client {
    // store: Option<Arc<Store>>,
    vault: Option<Arc<Vault>>,

    // A keystore
    pub(crate) keystore: KeyStore<Provider>,

    // A view on the vault entries
    pub(crate) db: DbView<Provider>, // Arc<RwLock<DbView<Provider>>>,

    // The id of this client
    pub id: ClientId,

    // Contains the Record Ids for the most recent Record in each vault.
    pub store: Arc<Store>,
}

impl Default for Client {
    fn default() -> Self {
        todo!()
    }
}

impl Drop for Client {
    fn drop(&mut self) {}
}

impl Client {
    /// Returns an atomic reference to the [`Store`]
    ///
    /// # Example
    /// ```
    /// ```
    pub async fn store(&self) -> Arc<Store> {
        self.store.clone()
    }

    /// Returns a [`Vault`] according to path
    ///
    /// # Example
    /// ```
    /// ```
    pub async fn vault<P>(&self, path: P) -> Vault
    where
        P: AsRef<Vec<u8>>,
    {
        todo!()
    }

    /// Returns `true`, if a vault exists
    ///
    /// # Example
    /// ```
    /// ```
    pub async fn vault_exists<P>(&self, vault_path: P) -> bool
    where
        P: AsRef<Vec<u8>>,
    {
        let vault_id = derive_vault_id(vault_path);
        self.keystore.vault_exists(vault_id)
    }

    /// Returns Ok, if the record exists
    ///
    /// # Example
    /// ```
    /// ```
    pub async fn record_exists(&mut self, location: Location) -> Result<bool, ClientError> {
        let (vault_id, record_id) = location.resolve();
        let result = match self.keystore.take_key(vault_id) {
            Some(key) => {
                let res = self.db.contains_record(&key, vault_id, record_id);
                self.keystore.insert_key(vault_id, key);
                res
            }
            None => false,
        };
        Ok(result)
    }

    /// Returns the [`ClientId`] of the client
    ///
    /// # Example
    /// ```
    /// ```
    pub async fn id(&self) -> &ClientId {
        &self.id
    }

    /// Writes all the changes into the snapshot
    ///
    /// # Example
    /// ```
    /// ```
    pub async fn update<S>(&self, snapshot: S) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    /// Executes a cryptographic [`Procedure`] and returns its output.
    /// A cryptographic [`Procedure`] is the main operation on secrets.
    ///
    /// # Example
    /// ```no_run
    /// ```
    pub async fn execute_procedure<P>(&mut self, procedure: P) -> Result<P::Output, ProcedureError>
    where
        P: Procedure + Into<StrongholdProcedure>,
    {
        let res = self.execure_procedure_chained(vec![procedure.into()]).await;
        let mapped = res.map(|mut vec| vec.pop().unwrap().try_into().ok().unwrap())?;
        Ok(mapped)
    }

    /// Executes a list of cryptographic [`Procedures`] sequentially and returns a collected output
    ///
    /// # Example
    /// ```no_run
    /// ```
    pub async fn execure_procedure_chained(
        &mut self,
        procedures: Vec<StrongholdProcedure>,
    ) -> core::result::Result<Vec<ProcedureOutput>, ProcedureError> {
        let mut out = Vec::new();
        let mut log = Vec::new();
        // Execute the procedures sequentially.
        for proc in procedures {
            if let Some(output) = proc.output() {
                log.push(output);
            }
            let output = match proc.execute(self) {
                Ok(o) => o,
                Err(e) => {
                    for location in log {
                        let _ = self.revoke_data(&location);
                    }
                    return Err(e);
                }
            };
            out.push(output);
        }
        Ok(out)
    }
}

// Compatibility to old structure
