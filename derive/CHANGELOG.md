# Changelog

## \[0.3.0]

- Update inline Docs and README files to reflect the current state of the project.
  - [fc95c271](https://www.github.com/iotaledger/stronghold.rs/commit/fc95c27128dedf8aa2d366776c22cb9c8e3f158a) add changes. on 2021-07-01
  - [eafca12a](https://www.github.com/iotaledger/stronghold.rs/commit/eafca12ad915166d8039df6ad050bb1c65cbe248) fix changes format. on 2021-07-01
- \[[PR 258](https://github.com/iotaledger/stronghold.rs/pull/258)]
  Implement API for the Stronghold Procedures, see PR 258 for details.
  - [47b6364b](https://www.github.com/iotaledger/stronghold.rs/commit/47b6364bbd256f71cc7eb7cf4a731db19d39dab6) chore(\*): patch iota-crypto v0.7.0 -> v0.8.0 on 2021-11-12

## \[0.2.0]

- Merged Store, Vault and Snapshot into a single crate called Stronghold-Engine.
  Merged utils-derive and communication-macros into a new crate called stronghold-derive
  Export Stronghold-derive through Stronghold-utils.
  - [36c8983](https://www.github.com/iotaledger/stronghold.rs/commit/36c8983eefd594c702a9e8b32bad25354ad127c0) merge derive/macro crates. on 2021-04-21
  - [b7d44f5](https://www.github.com/iotaledger/stronghold.rs/commit/b7d44f530e08be27128f25f46b4bb05cf3da99bd) update config. on 2021-04-21

## \[0.1.1]

- move stronghold-utils and add utils-derive for proc macros.
  rebuild vault and remove versioning.
  update client to use new vault.
  - [5490f0a](https://www.github.com/iotaledger/stronghold.rs/commit/5490f0aaaf58e5322a5569c02669514ec067b02f) refactor vault ([#181](https://www.github.com/iotaledger/stronghold.rs/pull/181)) on 2021-04-15
- Updated cargo.toml files with the updated crypto.rs revisions and authors.
  Fixed logic in snapshot and providers to use the `try_*` encryption and decryption functions.
  Fixed commandline and stopped it from overwriting snapshots.
  - [64e08fe](https://www.github.com/iotaledger/stronghold.rs/commit/64e08fe39454d2191561783d009b155c91db37c1) add .changes. on 2021-03-19
  - [0758b67](https://www.github.com/iotaledger/stronghold.rs/commit/0758b6734a1e22d491345a6b894acea12ab5b1b7) add .changes. on 2021-03-19
- Patch libp2p v0.35 -> v0.36, handle Mdns and dns transport changes, and make P2PNetworkBehaviour init_swarm method async.
  Move communication macro test to stronghold-communication.
  - [7e3c024](https://www.github.com/iotaledger/stronghold.rs/commit/7e3c02412b4d8657e62bc0b14862443d2f1f1f63) patch(comms): libp2p v0.36 ([#180](https://www.github.com/iotaledger/stronghold.rs/pull/180)) on 2021-03-24
- Patch libp2p v0.36 -> v0.37, handle changes in Identify protocol and the removed Dereferencing from Swarm to NetworkBehaviour.
  Remove unnecessary fields in the pattern matching of the `RequestPermissions` macro.
  - [7e9d267](https://www.github.com/iotaledger/stronghold.rs/commit/7e9d267b873563656d8004416678ef0891f239ad) Update libp2p requirement from 0.36 to 0.37 in /communication ([#185](https://www.github.com/iotaledger/stronghold.rs/pull/185)) on 2021-04-15
