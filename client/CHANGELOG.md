# Changelog

## \[0.5.0]

- - replace actor system riker with actix

- introduced registry actor for clients as service

- introduced snapshot actor as service

- merge `Internal` and `Client`-Actors into `SecureClient`

- api change in interface for test reading secrets out of a vault. minimal impact.

- [e8b10eac](https://www.github.com/iotaledger/stronghold.rs/commit/e8b10eac4a914e5d78aae40ab4f1da15ac136ac7) feat: Migrating stronghold from riker actor system implementation to actix. client + internal actor have been merged. Message types are transformed into structs. on 2021-08-23

- \[[PR 270](https://github.com/iotaledger/stronghold.rs/pull/270)]

- Move management of network-Actor and client-target into Registry

- Make client-target optional, in case that it is killed before switching to another target

- Make registry a normal actor instead of a system-service

- [a4cb0152](https://www.github.com/iotaledger/stronghold.rs/commit/a4cb0152f79c643afbd4eea72318c3ce300a0c27) doc(client): Add changelog on 2021-10-26

- - \[[PR 297](https://github.com/iotaledger/stronghold.rs/pull/297)] Add `CopyRecord` procedure.
  - [ae6f566c](https://www.github.com/iotaledger/stronghold.rs/commit/ae6f566c97ca1bc2de4b710ac8aa86768159502b) feat(client): add procedure for copying a record on 2021-11-29

- - make stronghold interface clonable
  - [681a024e](https://www.github.com/iotaledger/stronghold.rs/commit/681a024e7fd5d6095bbf571d5a3d22fb449b54da) Clonable Stronghold Instance ([#257](https://www.github.com/iotaledger/stronghold.rs/pull/257)) on 2021-09-13

- Update inline Docs and README files to reflect the current state of the project.
  - [fc95c271](https://www.github.com/iotaledger/stronghold.rs/commit/fc95c27128dedf8aa2d366776c22cb9c8e3f158a) add changes. on 2021-07-01
  - [eafca12a](https://www.github.com/iotaledger/stronghold.rs/commit/eafca12ad915166d8039df6ad050bb1c65cbe248) fix changes format. on 2021-07-01

- - Add `actors::secure::StoreError::NotExisting` as proper error type for correct error handling in client.
  - [ad57181e](https://www.github.com/iotaledger/stronghold.rs/commit/ad57181e7c5baa4b6ccb66fb464667c97967db08) fix: inconsistent error message. ([#251](https://www.github.com/iotaledger/stronghold.rs/pull/251)) on 2021-08-26

- \[[PR 254](https://github.com/iotaledger/stronghold.rs/pull/254)]\
  Change key handling in the `SecureClient` to avoid unnecessary cloning of keys.
  Remove obsolete VaultId-HashSet from the `SecureClient`.
  - [9b8d0da1](https://www.github.com/iotaledger/stronghold.rs/commit/9b8d0da150afd7446198672c8f7675547031c060) Fix(client): Avoid Key cloning, remove redundant code ([#254](https://www.github.com/iotaledger/stronghold.rs/pull/254)) on 2021-09-09

- \[[PR 276](https://github.com/iotaledger/stronghold.rs/pull/276)]

- Remove `relay` and `mdns` features.

- In the `StrongholdP2p` Interface enable / disable mdns and relay functionality on init via config flags in the `StrongholdP2pBuilder`.
  Per default, both are enabled.

- In the `Stronghold` client interface enable / disable mdns and relay in the `NetworkConfig` when spawning a new p2p-network actor.
  Per default, both are disabled.

- [8cbb8944](https://www.github.com/iotaledger/stronghold.rs/commit/8cbb8944bd4ef94ec331b97a8a9cbc7122172f8e) Add changelog on 2021-10-29

- [679cf029](https://www.github.com/iotaledger/stronghold.rs/commit/679cf02943460edf4560445f0b563f9cd0f0c9e8) feat(client):  mdns/relay config in Stronghold client on 2021-11-01

- Patch crypto.rs version v0.7 -> v0.8.
  - [47b6364b](https://www.github.com/iotaledger/stronghold.rs/commit/47b6364bbd256f71cc7eb7cf4a731db19d39dab6) chore(\*): patch iota-crypto v0.7.0 -> v0.8.0 on 2021-11-12

- \[[PR 290](https://github.com/iotaledger/stronghold.rs/pull/290)]

- Persist the state of stronghold-p2p in the `SecureClient` by serializing the `NetworkConfig` and writing it to the store.

- Allow loading stored states into the `NetworkActor` on init.

- Allow reuse of same `Keypair` that is stored in the vault.

- [83903c7e](https://www.github.com/iotaledger/stronghold.rs/commit/83903c7e69803a7dea54f2140d58a271796e6cc9) Add changelog for p2p/persist-config on 2021-11-16

- \[[PR 285](https://github.com/iotaledger/stronghold.rs/pull/285)]
  Implement messages to write the keypair used for `StrongholdP2p` in the vault and derive the
  `PeerId` and a new noise `AuthenticKeypair` from it.
  - [70b29c10](https://www.github.com/iotaledger/stronghold.rs/commit/70b29c1086db0491f4c8b14d8db49eadb6d6cfa8) doc(client): document p2p_messages, add changelog on 2021-11-12

- \[[PR 258](https://github.com/iotaledger/stronghold.rs/pull/258)]
  Implement API for the Stronghold Procedures, see PR 258 for details.
  - [47b6364b](https://www.github.com/iotaledger/stronghold.rs/commit/47b6364bbd256f71cc7eb7cf4a731db19d39dab6) chore(\*): patch iota-crypto v0.7.0 -> v0.8.0 on 2021-11-12

- \[[PR 269](https://github.com/iotaledger/stronghold.rs/pull/269)]
  Refactor Error types in engine and client:

- Add differentiated error types for the different methods

- Avoid unwraps in the engine

- Remove the single, crate-wide used error types of engine and client

- Remove `anyhow` Error types in client and bubble up the actual error instead

- Add nested Result type alias `StrongholdResult<T> = Result<T, ActorError>` for interface errors

- [d6b814dd](https://www.github.com/iotaledger/stronghold.rs/commit/d6b814dd7729dbbf39b73e050767992aadc19377) Add Changelog for PR 269 on 2021-11-03

- - corrects wrong control flow. `write_to_vault` always returned an error even if the operation was successful.
  - [aea8a9dc](https://www.github.com/iotaledger/stronghold.rs/commit/aea8a9dc8c3fa12e5444c5b4bb3303876e4c1a2f) Fix/wrong cf on write to vault ([#253](https://www.github.com/iotaledger/stronghold.rs/pull/253)) on 2021-08-30
