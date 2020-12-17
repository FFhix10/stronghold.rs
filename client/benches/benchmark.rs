// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use criterion::{criterion_group, criterion_main, Criterion};

use iota_stronghold::{Location, RecordHint, Stronghold};

use riker::actors::*;

use futures::executor::block_on;

fn init_stronghold_system_write() -> Stronghold {
    let system = ActorSystem::new().unwrap();
    Stronghold::init_stronghold_system(system, b"path".to_vec(), vec![])
}

fn init_stronghold_system_read() -> Stronghold {
    let system = ActorSystem::new().unwrap();
    let stronghold = Stronghold::init_stronghold_system(system, b"path".to_vec(), vec![]);

    for i in 0..10 {
        block_on(stronghold.write_data(
            Location::generic("test", format!("some_record {}", i)),
            format!("test data {}", i).as_bytes().to_vec(),
            RecordHint::new(b"test").unwrap(),
            vec![],
        ));
    }

    stronghold
}

fn bench_stronghold_write(c: &mut Criterion) {
    let stronghold = init_stronghold_system_write();

    c.bench_function("write to stronghold", |b| {
        b.iter(|| {
            block_on(stronghold.write_data(
                Location::generic("test", "some_record"),
                b"test data".to_vec(),
                RecordHint::new(b"test").unwrap(),
                vec![],
            ));
        });
    });
}

fn bench_stronghold_read(c: &mut Criterion) {
    let stronghold = init_stronghold_system_read();

    c.bench_function("read from stronghold", |b| {
        b.iter(|| {
            block_on(stronghold.read_data(Location::generic("test", "some_record 5")));
        });
    });
}

criterion_group!(benches, bench_stronghold_write, bench_stronghold_read);
criterion_main!(benches);
