#![allow(non_snake_case)]
#![allow(clippy::needless_return)]

use std::collections::HashMap;
use anyhow::Result;
use criterion::{criterion_group, criterion_main, Criterion};
use std::io::sink;
use workload_gen::{spec::WorkloadSpec, write_operations_with_keyset, keyset::KeySet};
use std::sync::LazyLock;

fn generate_operations<TKeySet: KeySet>(spec: &WorkloadSpec, keyset_constructor: impl Fn(usize) -> TKeySet) -> Result<()> {
    return write_operations_with_keyset(&mut sink(), spec, keyset_constructor);
}

static SPECS: LazyLock<HashMap<&'static str, WorkloadSpec>> = LazyLock::new(|| {
    let mut specs = HashMap::new();
    specs.insert("empty_point_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__point_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__point_deletes.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__point_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__point_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__point_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__point_deletes__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__point_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__point_queries.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__point_queries__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__point_queries__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__point_queries__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__point_queries__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__point_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__point_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__range_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__range_deletes.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__range_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__range_deletes__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__inserts__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__inserts__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__point_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__point_deletes.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__point_deletes__point_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__point_deletes__point_queries.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__point_deletes__point_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__point_deletes__point_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__point_deletes__range_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__point_deletes__range_deletes.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__point_deletes__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__point_deletes__range_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__point_deletes__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__point_deletes__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__point_deletes__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__point_deletes__range_deletes__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__point_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__point_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__point_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__point_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__point_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__point_deletes__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__point_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__point_queries.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__point_queries__range_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__point_queries__range_deletes.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__point_queries__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__point_queries__range_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__point_queries__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__point_queries__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__point_queries__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__point_queries__range_deletes__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__point_queries__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__point_queries__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__point_queries__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__point_queries__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__point_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__point_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__range_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__range_deletes.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__range_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__range_deletes__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__empty_point_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__empty_point_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__inserts", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__point_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__point_deletes.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__point_deletes__point_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__point_deletes__point_queries.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__point_deletes__point_queries__range_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__point_deletes__point_queries__range_deletes.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__updates.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__point_deletes__point_queries__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__point_deletes__point_queries__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__point_deletes__point_queries__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__point_deletes__point_queries__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__point_deletes__point_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__point_deletes__point_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__point_deletes__range_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__point_deletes__range_deletes.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__point_deletes__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__point_deletes__range_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__point_deletes__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__point_deletes__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__point_deletes__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__point_deletes__range_deletes__updates.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__point_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__point_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__point_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__point_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__point_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__point_deletes__updates.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__point_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__point_queries.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__point_queries__range_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__point_queries__range_deletes.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__point_queries__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__point_queries__range_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__point_queries__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__point_queries__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__point_queries__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__point_queries__range_deletes__updates.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__point_queries__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__point_queries__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__point_queries__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__point_queries__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__point_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__point_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__range_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__range_deletes.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__range_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__range_deletes__updates.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__inserts__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__inserts__updates.json")).unwrap());
    specs.insert("empty_point_deletes__point_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__point_deletes.json")).unwrap());
    specs.insert("empty_point_deletes__point_deletes__point_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__point_deletes__point_queries.json")).unwrap());
    specs.insert("empty_point_deletes__point_deletes__point_queries__range_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__point_deletes__point_queries__range_deletes.json")).unwrap());
    specs.insert("empty_point_deletes__point_deletes__point_queries__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__point_deletes__point_queries__range_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__point_deletes__point_queries__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__point_deletes__point_queries__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__point_deletes__point_queries__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__point_deletes__point_queries__range_deletes__updates.json")).unwrap());
    specs.insert("empty_point_deletes__point_deletes__point_queries__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__point_deletes__point_queries__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__point_deletes__point_queries__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__point_deletes__point_queries__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__point_deletes__point_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__point_deletes__point_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__point_deletes__range_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__point_deletes__range_deletes.json")).unwrap());
    specs.insert("empty_point_deletes__point_deletes__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__point_deletes__range_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__point_deletes__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__point_deletes__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__point_deletes__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__point_deletes__range_deletes__updates.json")).unwrap());
    specs.insert("empty_point_deletes__point_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__point_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__point_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__point_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__point_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__point_deletes__updates.json")).unwrap());
    specs.insert("empty_point_deletes__point_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__point_queries.json")).unwrap());
    specs.insert("empty_point_deletes__point_queries__range_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__point_queries__range_deletes.json")).unwrap());
    specs.insert("empty_point_deletes__point_queries__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__point_queries__range_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__point_queries__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__point_queries__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__point_queries__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__point_queries__range_deletes__updates.json")).unwrap());
    specs.insert("empty_point_deletes__point_queries__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__point_queries__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__point_queries__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__point_queries__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__point_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__point_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__range_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__range_deletes.json")).unwrap());
    specs.insert("empty_point_deletes__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__range_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__range_deletes__updates.json")).unwrap());
    specs.insert("empty_point_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_deletes__updates.json")).unwrap());
    specs.insert("empty_point_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_queries.json")).unwrap());
    specs.insert("empty_point_queries__inserts", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts.json")).unwrap());
    specs.insert("empty_point_queries__inserts__point_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__point_deletes.json")).unwrap());
    specs.insert("empty_point_queries__inserts__point_deletes__point_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__point_deletes__point_queries.json")).unwrap());
    specs.insert("empty_point_queries__inserts__point_deletes__point_queries__range_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__point_deletes__point_queries__range_deletes.json")).unwrap());
    specs.insert("empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_queries__inserts__point_deletes__point_queries__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__point_deletes__point_queries__range_deletes__updates.json")).unwrap());
    specs.insert("empty_point_queries__inserts__point_deletes__point_queries__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__point_deletes__point_queries__range_queries.json")).unwrap());
    specs.insert("empty_point_queries__inserts__point_deletes__point_queries__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__point_deletes__point_queries__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_queries__inserts__point_deletes__point_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__point_deletes__point_queries__updates.json")).unwrap());
    specs.insert("empty_point_queries__inserts__point_deletes__range_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__point_deletes__range_deletes.json")).unwrap());
    specs.insert("empty_point_queries__inserts__point_deletes__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__point_deletes__range_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_queries__inserts__point_deletes__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__point_deletes__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_queries__inserts__point_deletes__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__point_deletes__range_deletes__updates.json")).unwrap());
    specs.insert("empty_point_queries__inserts__point_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__point_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_queries__inserts__point_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__point_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_queries__inserts__point_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__point_deletes__updates.json")).unwrap());
    specs.insert("empty_point_queries__inserts__point_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__point_queries.json")).unwrap());
    specs.insert("empty_point_queries__inserts__point_queries__range_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__point_queries__range_deletes.json")).unwrap());
    specs.insert("empty_point_queries__inserts__point_queries__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__point_queries__range_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_queries__inserts__point_queries__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__point_queries__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_queries__inserts__point_queries__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__point_queries__range_deletes__updates.json")).unwrap());
    specs.insert("empty_point_queries__inserts__point_queries__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__point_queries__range_queries.json")).unwrap());
    specs.insert("empty_point_queries__inserts__point_queries__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__point_queries__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_queries__inserts__point_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__point_queries__updates.json")).unwrap());
    specs.insert("empty_point_queries__inserts__range_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__range_deletes.json")).unwrap());
    specs.insert("empty_point_queries__inserts__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__range_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_queries__inserts__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_queries__inserts__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__range_deletes__updates.json")).unwrap());
    specs.insert("empty_point_queries__inserts__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__range_queries.json")).unwrap());
    specs.insert("empty_point_queries__inserts__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_queries__inserts__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__inserts__updates.json")).unwrap());
    specs.insert("empty_point_queries__point_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__point_deletes.json")).unwrap());
    specs.insert("empty_point_queries__point_deletes__point_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__point_deletes__point_queries.json")).unwrap());
    specs.insert("empty_point_queries__point_deletes__point_queries__range_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__point_deletes__point_queries__range_deletes.json")).unwrap());
    specs.insert("empty_point_queries__point_deletes__point_queries__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__point_deletes__point_queries__range_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_queries__point_deletes__point_queries__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__point_deletes__point_queries__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_queries__point_deletes__point_queries__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__point_deletes__point_queries__range_deletes__updates.json")).unwrap());
    specs.insert("empty_point_queries__point_deletes__point_queries__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__point_deletes__point_queries__range_queries.json")).unwrap());
    specs.insert("empty_point_queries__point_deletes__point_queries__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__point_deletes__point_queries__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_queries__point_deletes__point_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__point_deletes__point_queries__updates.json")).unwrap());
    specs.insert("empty_point_queries__point_deletes__range_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__point_deletes__range_deletes.json")).unwrap());
    specs.insert("empty_point_queries__point_deletes__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__point_deletes__range_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_queries__point_deletes__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__point_deletes__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_queries__point_deletes__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__point_deletes__range_deletes__updates.json")).unwrap());
    specs.insert("empty_point_queries__point_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__point_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_queries__point_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__point_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_queries__point_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__point_deletes__updates.json")).unwrap());
    specs.insert("empty_point_queries__point_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__point_queries.json")).unwrap());
    specs.insert("empty_point_queries__point_queries__range_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__point_queries__range_deletes.json")).unwrap());
    specs.insert("empty_point_queries__point_queries__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__point_queries__range_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_queries__point_queries__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__point_queries__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_queries__point_queries__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__point_queries__range_deletes__updates.json")).unwrap());
    specs.insert("empty_point_queries__point_queries__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__point_queries__range_queries.json")).unwrap());
    specs.insert("empty_point_queries__point_queries__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__point_queries__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_queries__point_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__point_queries__updates.json")).unwrap());
    specs.insert("empty_point_queries__range_deletes", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__range_deletes.json")).unwrap());
    specs.insert("empty_point_queries__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__range_deletes__range_queries.json")).unwrap());
    specs.insert("empty_point_queries__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_queries__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__range_deletes__updates.json")).unwrap());
    specs.insert("empty_point_queries__range_queries", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__range_queries.json")).unwrap());
    specs.insert("empty_point_queries__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__range_queries__updates.json")).unwrap());
    specs.insert("empty_point_queries__updates", serde_json::from_str(include_str!("./bench-specs/empty_point_queries__updates.json")).unwrap());
    specs.insert("inserts", serde_json::from_str(include_str!("./bench-specs/inserts.json")).unwrap());
    specs.insert("inserts__point_deletes", serde_json::from_str(include_str!("./bench-specs/inserts__point_deletes.json")).unwrap());
    specs.insert("inserts__point_deletes__point_queries", serde_json::from_str(include_str!("./bench-specs/inserts__point_deletes__point_queries.json")).unwrap());
    specs.insert("inserts__point_deletes__point_queries__range_deletes", serde_json::from_str(include_str!("./bench-specs/inserts__point_deletes__point_queries__range_deletes.json")).unwrap());
    specs.insert("inserts__point_deletes__point_queries__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/inserts__point_deletes__point_queries__range_deletes__range_queries.json")).unwrap());
    specs.insert("inserts__point_deletes__point_queries__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/inserts__point_deletes__point_queries__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("inserts__point_deletes__point_queries__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/inserts__point_deletes__point_queries__range_deletes__updates.json")).unwrap());
    specs.insert("inserts__point_deletes__point_queries__range_queries", serde_json::from_str(include_str!("./bench-specs/inserts__point_deletes__point_queries__range_queries.json")).unwrap());
    specs.insert("inserts__point_deletes__point_queries__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/inserts__point_deletes__point_queries__range_queries__updates.json")).unwrap());
    specs.insert("inserts__point_deletes__point_queries__updates", serde_json::from_str(include_str!("./bench-specs/inserts__point_deletes__point_queries__updates.json")).unwrap());
    specs.insert("inserts__point_deletes__range_deletes", serde_json::from_str(include_str!("./bench-specs/inserts__point_deletes__range_deletes.json")).unwrap());
    specs.insert("inserts__point_deletes__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/inserts__point_deletes__range_deletes__range_queries.json")).unwrap());
    specs.insert("inserts__point_deletes__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/inserts__point_deletes__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("inserts__point_deletes__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/inserts__point_deletes__range_deletes__updates.json")).unwrap());
    specs.insert("inserts__point_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/inserts__point_deletes__range_queries.json")).unwrap());
    specs.insert("inserts__point_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/inserts__point_deletes__range_queries__updates.json")).unwrap());
    specs.insert("inserts__point_deletes__updates", serde_json::from_str(include_str!("./bench-specs/inserts__point_deletes__updates.json")).unwrap());
    specs.insert("inserts__point_queries", serde_json::from_str(include_str!("./bench-specs/inserts__point_queries.json")).unwrap());
    specs.insert("inserts__point_queries__range_deletes", serde_json::from_str(include_str!("./bench-specs/inserts__point_queries__range_deletes.json")).unwrap());
    specs.insert("inserts__point_queries__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/inserts__point_queries__range_deletes__range_queries.json")).unwrap());
    specs.insert("inserts__point_queries__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/inserts__point_queries__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("inserts__point_queries__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/inserts__point_queries__range_deletes__updates.json")).unwrap());
    specs.insert("inserts__point_queries__range_queries", serde_json::from_str(include_str!("./bench-specs/inserts__point_queries__range_queries.json")).unwrap());
    specs.insert("inserts__point_queries__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/inserts__point_queries__range_queries__updates.json")).unwrap());
    specs.insert("inserts__point_queries__updates", serde_json::from_str(include_str!("./bench-specs/inserts__point_queries__updates.json")).unwrap());
    specs.insert("inserts__range_deletes", serde_json::from_str(include_str!("./bench-specs/inserts__range_deletes.json")).unwrap());
    specs.insert("inserts__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/inserts__range_deletes__range_queries.json")).unwrap());
    specs.insert("inserts__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/inserts__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("inserts__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/inserts__range_deletes__updates.json")).unwrap());
    specs.insert("inserts__range_queries", serde_json::from_str(include_str!("./bench-specs/inserts__range_queries.json")).unwrap());
    specs.insert("inserts__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/inserts__range_queries__updates.json")).unwrap());
    specs.insert("inserts__updates", serde_json::from_str(include_str!("./bench-specs/inserts__updates.json")).unwrap());
    specs.insert("point_deletes", serde_json::from_str(include_str!("./bench-specs/point_deletes.json")).unwrap());
    specs.insert("point_deletes__point_queries", serde_json::from_str(include_str!("./bench-specs/point_deletes__point_queries.json")).unwrap());
    specs.insert("point_deletes__point_queries__range_deletes", serde_json::from_str(include_str!("./bench-specs/point_deletes__point_queries__range_deletes.json")).unwrap());
    specs.insert("point_deletes__point_queries__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/point_deletes__point_queries__range_deletes__range_queries.json")).unwrap());
    specs.insert("point_deletes__point_queries__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/point_deletes__point_queries__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("point_deletes__point_queries__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/point_deletes__point_queries__range_deletes__updates.json")).unwrap());
    specs.insert("point_deletes__point_queries__range_queries", serde_json::from_str(include_str!("./bench-specs/point_deletes__point_queries__range_queries.json")).unwrap());
    specs.insert("point_deletes__point_queries__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/point_deletes__point_queries__range_queries__updates.json")).unwrap());
    specs.insert("point_deletes__point_queries__updates", serde_json::from_str(include_str!("./bench-specs/point_deletes__point_queries__updates.json")).unwrap());
    specs.insert("point_deletes__range_deletes", serde_json::from_str(include_str!("./bench-specs/point_deletes__range_deletes.json")).unwrap());
    specs.insert("point_deletes__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/point_deletes__range_deletes__range_queries.json")).unwrap());
    specs.insert("point_deletes__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/point_deletes__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("point_deletes__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/point_deletes__range_deletes__updates.json")).unwrap());
    specs.insert("point_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/point_deletes__range_queries.json")).unwrap());
    specs.insert("point_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/point_deletes__range_queries__updates.json")).unwrap());
    specs.insert("point_deletes__updates", serde_json::from_str(include_str!("./bench-specs/point_deletes__updates.json")).unwrap());
    specs.insert("point_queries", serde_json::from_str(include_str!("./bench-specs/point_queries.json")).unwrap());
    specs.insert("point_queries__range_deletes", serde_json::from_str(include_str!("./bench-specs/point_queries__range_deletes.json")).unwrap());
    specs.insert("point_queries__range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/point_queries__range_deletes__range_queries.json")).unwrap());
    specs.insert("point_queries__range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/point_queries__range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("point_queries__range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/point_queries__range_deletes__updates.json")).unwrap());
    specs.insert("point_queries__range_queries", serde_json::from_str(include_str!("./bench-specs/point_queries__range_queries.json")).unwrap());
    specs.insert("point_queries__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/point_queries__range_queries__updates.json")).unwrap());
    specs.insert("point_queries__updates", serde_json::from_str(include_str!("./bench-specs/point_queries__updates.json")).unwrap());
    specs.insert("range_deletes", serde_json::from_str(include_str!("./bench-specs/range_deletes.json")).unwrap());
    specs.insert("range_deletes__range_queries", serde_json::from_str(include_str!("./bench-specs/range_deletes__range_queries.json")).unwrap());
    specs.insert("range_deletes__range_queries__updates", serde_json::from_str(include_str!("./bench-specs/range_deletes__range_queries__updates.json")).unwrap());
    specs.insert("range_deletes__updates", serde_json::from_str(include_str!("./bench-specs/range_deletes__updates.json")).unwrap());
    specs.insert("range_queries", serde_json::from_str(include_str!("./bench-specs/range_queries.json")).unwrap());
    specs.insert("range_queries__updates", serde_json::from_str(include_str!("./bench-specs/range_queries__updates.json")).unwrap());
    specs.insert("updates", serde_json::from_str(include_str!("./bench-specs/updates.json")).unwrap());

    specs
});

fn vec_key_set(c: &mut Criterion) {
    let keyset_constructor = |cap| workload_gen::keyset::VecKeySet::new(cap);

    c.bench_function("empty_point_deletes", |b| {
        let spec = &SPECS["empty_point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts", |b| {
        let spec = &SPECS["empty_point_deletes__inserts"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries", |b| {
        let spec = &SPECS["empty_point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts", |b| {
        let spec = &SPECS["empty_point_queries__inserts"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__inserts__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts", |b| {
        let spec = &SPECS["inserts"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes", |b| {
        let spec = &SPECS["inserts__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__range_deletes", |b| {
        let spec = &SPECS["inserts__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["inserts__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__range_queries", |b| {
        let spec = &SPECS["inserts__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries", |b| {
        let spec = &SPECS["inserts__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__range_deletes", |b| {
        let spec = &SPECS["inserts__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["inserts__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["inserts__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["inserts__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__range_queries", |b| {
        let spec = &SPECS["inserts__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["inserts__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__updates", |b| {
        let spec = &SPECS["inserts__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__range_deletes", |b| {
        let spec = &SPECS["inserts__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__range_deletes__range_queries", |b| {
        let spec = &SPECS["inserts__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["inserts__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__range_deletes__updates", |b| {
        let spec = &SPECS["inserts__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__range_queries", |b| {
        let spec = &SPECS["inserts__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__range_queries__updates", |b| {
        let spec = &SPECS["inserts__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__updates", |b| {
        let spec = &SPECS["inserts__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes", |b| {
        let spec = &SPECS["point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries", |b| {
        let spec = &SPECS["point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__range_deletes", |b| {
        let spec = &SPECS["point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__range_queries", |b| {
        let spec = &SPECS["point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__updates", |b| {
        let spec = &SPECS["point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries", |b| {
        let spec = &SPECS["point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__range_deletes", |b| {
        let spec = &SPECS["point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__range_queries", |b| {
        let spec = &SPECS["point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__range_queries__updates", |b| {
        let spec = &SPECS["point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__updates", |b| {
        let spec = &SPECS["point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("range_deletes", |b| {
        let spec = &SPECS["range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("range_deletes__range_queries", |b| {
        let spec = &SPECS["range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("range_deletes__updates", |b| {
        let spec = &SPECS["range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("range_queries", |b| {
        let spec = &SPECS["range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("range_queries__updates", |b| {
        let spec = &SPECS["range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("updates", |b| {
        let spec = &SPECS["updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });

}
fn vec_hash_set_key_set(c: &mut Criterion) {
    let keyset_constructor = |cap| workload_gen::keyset::VecHashSetKeySet::new(cap);

    c.bench_function("empty_point_deletes", |b| {
        let spec = &SPECS["empty_point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts", |b| {
        let spec = &SPECS["empty_point_deletes__inserts"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries", |b| {
        let spec = &SPECS["empty_point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts", |b| {
        let spec = &SPECS["empty_point_queries__inserts"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__inserts__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts", |b| {
        let spec = &SPECS["inserts"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes", |b| {
        let spec = &SPECS["inserts__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__range_deletes", |b| {
        let spec = &SPECS["inserts__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["inserts__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__range_queries", |b| {
        let spec = &SPECS["inserts__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries", |b| {
        let spec = &SPECS["inserts__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__range_deletes", |b| {
        let spec = &SPECS["inserts__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["inserts__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["inserts__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["inserts__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__range_queries", |b| {
        let spec = &SPECS["inserts__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["inserts__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__updates", |b| {
        let spec = &SPECS["inserts__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__range_deletes", |b| {
        let spec = &SPECS["inserts__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__range_deletes__range_queries", |b| {
        let spec = &SPECS["inserts__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["inserts__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__range_deletes__updates", |b| {
        let spec = &SPECS["inserts__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__range_queries", |b| {
        let spec = &SPECS["inserts__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__range_queries__updates", |b| {
        let spec = &SPECS["inserts__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__updates", |b| {
        let spec = &SPECS["inserts__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes", |b| {
        let spec = &SPECS["point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries", |b| {
        let spec = &SPECS["point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__range_deletes", |b| {
        let spec = &SPECS["point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__range_queries", |b| {
        let spec = &SPECS["point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__updates", |b| {
        let spec = &SPECS["point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries", |b| {
        let spec = &SPECS["point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__range_deletes", |b| {
        let spec = &SPECS["point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__range_queries", |b| {
        let spec = &SPECS["point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__range_queries__updates", |b| {
        let spec = &SPECS["point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__updates", |b| {
        let spec = &SPECS["point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("range_deletes", |b| {
        let spec = &SPECS["range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("range_deletes__range_queries", |b| {
        let spec = &SPECS["range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("range_deletes__updates", |b| {
        let spec = &SPECS["range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("range_queries", |b| {
        let spec = &SPECS["range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("range_queries__updates", |b| {
        let spec = &SPECS["range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("updates", |b| {
        let spec = &SPECS["updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });

}
fn vec_bloom_filter_key_set(c: &mut Criterion) {
    let keyset_constructor = |cap| workload_gen::keyset::VecBloomFilterKeySet::new(cap);

    c.bench_function("empty_point_deletes", |b| {
        let spec = &SPECS["empty_point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts", |b| {
        let spec = &SPECS["empty_point_deletes__inserts"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries", |b| {
        let spec = &SPECS["empty_point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts", |b| {
        let spec = &SPECS["empty_point_queries__inserts"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__inserts__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts", |b| {
        let spec = &SPECS["inserts"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes", |b| {
        let spec = &SPECS["inserts__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__range_deletes", |b| {
        let spec = &SPECS["inserts__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["inserts__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__range_queries", |b| {
        let spec = &SPECS["inserts__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries", |b| {
        let spec = &SPECS["inserts__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__range_deletes", |b| {
        let spec = &SPECS["inserts__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["inserts__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["inserts__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["inserts__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__range_queries", |b| {
        let spec = &SPECS["inserts__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["inserts__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__updates", |b| {
        let spec = &SPECS["inserts__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__range_deletes", |b| {
        let spec = &SPECS["inserts__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__range_deletes__range_queries", |b| {
        let spec = &SPECS["inserts__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["inserts__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__range_deletes__updates", |b| {
        let spec = &SPECS["inserts__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__range_queries", |b| {
        let spec = &SPECS["inserts__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__range_queries__updates", |b| {
        let spec = &SPECS["inserts__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__updates", |b| {
        let spec = &SPECS["inserts__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes", |b| {
        let spec = &SPECS["point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries", |b| {
        let spec = &SPECS["point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__range_deletes", |b| {
        let spec = &SPECS["point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__range_queries", |b| {
        let spec = &SPECS["point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__updates", |b| {
        let spec = &SPECS["point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries", |b| {
        let spec = &SPECS["point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__range_deletes", |b| {
        let spec = &SPECS["point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__range_queries", |b| {
        let spec = &SPECS["point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__range_queries__updates", |b| {
        let spec = &SPECS["point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__updates", |b| {
        let spec = &SPECS["point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("range_deletes", |b| {
        let spec = &SPECS["range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("range_deletes__range_queries", |b| {
        let spec = &SPECS["range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("range_deletes__updates", |b| {
        let spec = &SPECS["range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("range_queries", |b| {
        let spec = &SPECS["range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("range_queries__updates", |b| {
        let spec = &SPECS["range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("updates", |b| {
        let spec = &SPECS["updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });

}
fn vec_hash_map_index_key_set(c: &mut Criterion) {
    let keyset_constructor = |cap| workload_gen::keyset::VecHashMapIndexKeySet::new(cap);

    c.bench_function("empty_point_deletes", |b| {
        let spec = &SPECS["empty_point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts", |b| {
        let spec = &SPECS["empty_point_deletes__inserts"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries", |b| {
        let spec = &SPECS["empty_point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts", |b| {
        let spec = &SPECS["empty_point_queries__inserts"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__inserts__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts", |b| {
        let spec = &SPECS["inserts"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes", |b| {
        let spec = &SPECS["inserts__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__range_deletes", |b| {
        let spec = &SPECS["inserts__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["inserts__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__range_queries", |b| {
        let spec = &SPECS["inserts__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries", |b| {
        let spec = &SPECS["inserts__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__range_deletes", |b| {
        let spec = &SPECS["inserts__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["inserts__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["inserts__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["inserts__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__range_queries", |b| {
        let spec = &SPECS["inserts__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["inserts__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__updates", |b| {
        let spec = &SPECS["inserts__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__range_deletes", |b| {
        let spec = &SPECS["inserts__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__range_deletes__range_queries", |b| {
        let spec = &SPECS["inserts__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["inserts__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__range_deletes__updates", |b| {
        let spec = &SPECS["inserts__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__range_queries", |b| {
        let spec = &SPECS["inserts__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__range_queries__updates", |b| {
        let spec = &SPECS["inserts__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__updates", |b| {
        let spec = &SPECS["inserts__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes", |b| {
        let spec = &SPECS["point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries", |b| {
        let spec = &SPECS["point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__range_deletes", |b| {
        let spec = &SPECS["point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__range_queries", |b| {
        let spec = &SPECS["point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__updates", |b| {
        let spec = &SPECS["point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries", |b| {
        let spec = &SPECS["point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__range_deletes", |b| {
        let spec = &SPECS["point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__range_queries", |b| {
        let spec = &SPECS["point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__range_queries__updates", |b| {
        let spec = &SPECS["point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__updates", |b| {
        let spec = &SPECS["point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("range_deletes", |b| {
        let spec = &SPECS["range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("range_deletes__range_queries", |b| {
        let spec = &SPECS["range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("range_deletes__updates", |b| {
        let spec = &SPECS["range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("range_queries", |b| {
        let spec = &SPECS["range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("range_queries__updates", |b| {
        let spec = &SPECS["range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("updates", |b| {
        let spec = &SPECS["updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });

}
fn b_tree_set_key_set(c: &mut Criterion) {
    let keyset_constructor = |cap| workload_gen::keyset::BTreeSetKeySet::new(cap);

    c.bench_function("empty_point_deletes", |b| {
        let spec = &SPECS["empty_point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__inserts__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__inserts__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__empty_point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__empty_point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts", |b| {
        let spec = &SPECS["empty_point_deletes__inserts"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__inserts__updates", |b| {
        let spec = &SPECS["empty_point_deletes__inserts__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries", |b| {
        let spec = &SPECS["empty_point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts", |b| {
        let spec = &SPECS["empty_point_queries__inserts"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__inserts__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__inserts__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__inserts__updates", |b| {
        let spec = &SPECS["empty_point_queries__inserts__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__point_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__range_deletes", |b| {
        let spec = &SPECS["empty_point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["empty_point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__range_queries", |b| {
        let spec = &SPECS["empty_point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__range_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("empty_point_queries__updates", |b| {
        let spec = &SPECS["empty_point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts", |b| {
        let spec = &SPECS["inserts"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes", |b| {
        let spec = &SPECS["inserts__point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__range_deletes", |b| {
        let spec = &SPECS["inserts__point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["inserts__point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__range_queries", |b| {
        let spec = &SPECS["inserts__point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_deletes__updates", |b| {
        let spec = &SPECS["inserts__point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries", |b| {
        let spec = &SPECS["inserts__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__range_deletes", |b| {
        let spec = &SPECS["inserts__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["inserts__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["inserts__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["inserts__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__range_queries", |b| {
        let spec = &SPECS["inserts__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["inserts__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__point_queries__updates", |b| {
        let spec = &SPECS["inserts__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__range_deletes", |b| {
        let spec = &SPECS["inserts__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__range_deletes__range_queries", |b| {
        let spec = &SPECS["inserts__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["inserts__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__range_deletes__updates", |b| {
        let spec = &SPECS["inserts__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__range_queries", |b| {
        let spec = &SPECS["inserts__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__range_queries__updates", |b| {
        let spec = &SPECS["inserts__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("inserts__updates", |b| {
        let spec = &SPECS["inserts__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes", |b| {
        let spec = &SPECS["point_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries", |b| {
        let spec = &SPECS["point_deletes__point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__range_deletes", |b| {
        let spec = &SPECS["point_deletes__point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["point_deletes__point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["point_deletes__point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["point_deletes__point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__range_queries", |b| {
        let spec = &SPECS["point_deletes__point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__range_queries__updates", |b| {
        let spec = &SPECS["point_deletes__point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__point_queries__updates", |b| {
        let spec = &SPECS["point_deletes__point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__range_deletes", |b| {
        let spec = &SPECS["point_deletes__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__range_deletes__range_queries", |b| {
        let spec = &SPECS["point_deletes__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["point_deletes__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__range_deletes__updates", |b| {
        let spec = &SPECS["point_deletes__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__range_queries", |b| {
        let spec = &SPECS["point_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__range_queries__updates", |b| {
        let spec = &SPECS["point_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_deletes__updates", |b| {
        let spec = &SPECS["point_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries", |b| {
        let spec = &SPECS["point_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__range_deletes", |b| {
        let spec = &SPECS["point_queries__range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__range_deletes__range_queries", |b| {
        let spec = &SPECS["point_queries__range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["point_queries__range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__range_deletes__updates", |b| {
        let spec = &SPECS["point_queries__range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__range_queries", |b| {
        let spec = &SPECS["point_queries__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__range_queries__updates", |b| {
        let spec = &SPECS["point_queries__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("point_queries__updates", |b| {
        let spec = &SPECS["point_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("range_deletes", |b| {
        let spec = &SPECS["range_deletes"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("range_deletes__range_queries", |b| {
        let spec = &SPECS["range_deletes__range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("range_deletes__range_queries__updates", |b| {
        let spec = &SPECS["range_deletes__range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("range_deletes__updates", |b| {
        let spec = &SPECS["range_deletes__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("range_queries", |b| {
        let spec = &SPECS["range_queries"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("range_queries__updates", |b| {
        let spec = &SPECS["range_queries__updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });
    c.bench_function("updates", |b| {
        let spec = &SPECS["updates"];
        b.iter(|| generate_operations(&spec, &keyset_constructor).expect("Failed to generate operations"));
    });

}


fn criterion_custom() -> Criterion {
    Criterion::default()
        .warm_up_time(std::time::Duration::from_secs(1))
        .measurement_time(std::time::Duration::from_secs(1))
}

criterion_group!(
    name = benches;
    config = criterion_custom();
    targets =
    vec_key_set,
    vec_hash_set_key_set,
    vec_bloom_filter_key_set,
    vec_hash_map_index_key_set,
    b_tree_set_key_set,

);
criterion_main!(benches);
