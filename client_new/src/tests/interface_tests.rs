// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{error::Error, path::Path};

use crate::{
    procedures::{GenerateKey, KeyType, StrongholdProcedure},
    Client, ClientVault, KeyProvider, Location, Store, Stronghold,
};
use engine::vault::RecordHint;
use zeroize::Zeroize;

/// This is a testing stub and MUST be removed, if the actual implementation
/// is present
struct Snapshot {}

impl Snapshot {
    pub fn named(filename: String) -> Self {
        todo!()
    }

    pub fn try_from<P>(path: P) -> Result<Self, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        todo!()
    }

    pub async fn write(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

#[tokio::test]
async fn test_full_stronghold_access() -> Result<(), Box<dyn Error>> {
    let vault_path = b"vault_path".to_vec();
    let client_path = b"client_path".to_vec();

    // load the base type
    let stronghold = Stronghold::default();

    let keyprovider = KeyProvider::try_from(b"secret".to_vec()).map_err(|e| format!("Error {:?}", e))?;
    let snapshot_path = "/path/to/snapshot";

    let snapshot = Snapshot::try_from("/path/to/snapshot")?;

    // no mutability allowed!
    let mut client = Client::default();

    let generate_key_procedure = GenerateKey {
        ty: KeyType::Ed25519,
        output: crate::Location::generic(b"".to_vec(), b"".to_vec()),
        hint: RecordHint::new(b"").unwrap(),
    };
    client
        .execute_procedure(StrongholdProcedure::GenerateKey(generate_key_procedure))
        .await?;

    let store = client.store().await;

    let vault = client.vault(Location::const_generic(vault_path.to_vec(), b"".to_vec()));

    // create a new secret inside the vault
    vault.write_secret(
        Location::const_generic(vault_path.clone(), b"record-path".to_vec()),
        vec![],
    )?;

    // Write the state of the client back into the snapshot
    client.update(&snapshot).await?;

    // Write the current state into the snapshot
    snapshot.write().await?;

    Ok(())
}

#[test]
fn test_lock() {
    use std::sync::{Arc, RwLock};

    let data = Arc::new(RwLock::new(Some(12usize)));

    let mut data = data.write().expect("");

    let moved = data.take();
}
