# Changelog

## \[0.3.1]

- - [PR 293](https://github.com/iotaledger/stronghold.rs/pull/293): Refactor Sink and Stream implementation for EventChannel
  - [b28f42bf](https://www.github.com/iotaledger/stronghold.rs/commit/b28f42bfead8a93c3da987a73c62d6dd957e75e1) fix(p2p): fix EventChannel, add tests ([#293](https://www.github.com/iotaledger/stronghold.rs/pull/293)) on 2021-11-24

- \[[PR 276](https://github.com/iotaledger/stronghold.rs/pull/276)]

- Remove `relay` and `mdns` features.

- In the `StrongholdP2p` Interface enable / disable mdns and relay functionality on init via config flags in the `StrongholdP2pBuilder`.
  Per default, both are enabled.

- In the `Stronghold` client interface enable / disable mdns and relay in the `NetworkConfig` when spawning a new p2p-network actor.
  Per default, both are disabled.

- [8cbb8944](https://www.github.com/iotaledger/stronghold.rs/commit/8cbb8944bd4ef94ec331b97a8a9cbc7122172f8e) Add changelog on 2021-10-29

- [679cf029](https://www.github.com/iotaledger/stronghold.rs/commit/679cf02943460edf4560445f0b563f9cd0f0c9e8) feat(client):  mdns/relay config in Stronghold client on 2021-11-01

- - [PR 295](https://github.com/iotaledger/stronghold.rs/pull/295): use `libp2p::swarm::toggle` to enable/ disable relay and mdns
  - [987bce97](https://www.github.com/iotaledger/stronghold.rs/commit/987bce97b6cfbd7d027ec4b7e5a8c7f82586a33f) feat(p2p): use libp2p::toggle for optional mdns/relay on 2021-11-28

- \[[PR 271](https://github.com/iotaledger/stronghold.rs/pull/271)]
  Patch libp2p version from v0.39 to v0.40.0-rc.2.
  - [edf29bfc](https://www.github.com/iotaledger/stronghold.rs/commit/edf29bfc0cc19a4ac11a87cf28400d260e20d8c0) Add changelog on 2021-10-22

- \[[PR 290](https://github.com/iotaledger/stronghold.rs/pull/290)]

- Persist the state of stronghold-p2p in the `SecureClient` by serializing the `NetworkConfig` and writing it to the store.

- Allow loading stored states into the `NetworkActor` on init.

- Allow reuse of same `Keypair` that is stored in the vault.

- [83903c7e](https://www.github.com/iotaledger/stronghold.rs/commit/83903c7e69803a7dea54f2140d58a271796e6cc9) Add changelog for p2p/persist-config on 2021-11-16
