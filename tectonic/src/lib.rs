#![feature(extend_one)]
#![feature(btree_cursors)]
#![feature(trusted_random_access)]
#![feature(trait_alias)]
#![allow(clippy::needless_return)]
#![allow(dead_code)]

use anyhow::{Context, Result, anyhow, bail};
use rand::distr::Alphanumeric;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::iter::repeat_n;
use std::path::PathBuf;

mod keyset;
pub mod spec;

// Operation order to be kept for each enum/match statement
// - insert
// - update
// - merge
// - delete point
// - delete point empty
// - delete range
// - query point
// - query point empty
// - query range

use crate::keyset::{Key, KeySet, VecKeySet};
use crate::spec::WorkloadSpec;

struct AsciiOperationFormatter;
impl AsciiOperationFormatter {
    fn write_insert(w: &mut impl Write, key: &Key, val: &Key) -> Result<()> {
        w.write_all("I ".as_bytes())?;
        w.write_all(key)?;
        w.write_all(" ".as_bytes())?;
        w.write_all(val)?;
        w.write_all("\n".as_bytes())?;

        return Ok(());
    }
    fn write_update(w: &mut impl Write, key: &Key, val: &Key) -> Result<()> {
        w.write_all("U ".as_bytes())?;
        w.write_all(key)?;
        w.write_all(" ".as_bytes())?;
        w.write_all(val)?;
        w.write_all("\n".as_bytes())?;

        return Ok(());
    }
    fn write_point_delete(w: &mut impl Write, key: &Key) -> Result<()> {
        w.write_all("D ".as_bytes())?;
        w.write_all(key)?;
        w.write_all("\n".as_bytes())?;

        return Ok(());
    }
    fn write_point_query(w: &mut impl Write, key: &Key) -> Result<()> {
        w.write_all("P ".as_bytes())?;
        w.write_all(key)?;
        w.write_all("\n".as_bytes())?;

        return Ok(());
    }
    fn write_range_query(w: &mut impl Write, key1: &Key, key2: &Key) -> Result<()> {
        w.write_all("S ".as_bytes())?;
        w.write_all(key1)?;
        w.write_all(" ".as_bytes())?;
        w.write_all(key2)?;
        w.write_all("\n".as_bytes())?;

        return Ok(());
    }
    fn write_range_delete(w: &mut impl Write, key1: &Key, key2: &Key) -> Result<()> {
        w.write_all("R ".as_bytes())?;
        w.write_all(key1)?;
        w.write_all(" ".as_bytes())?;
        w.write_all(key2)?;
        w.write_all("\n".as_bytes())?;

        return Ok(());
    }
}

#[derive(Debug, Copy, Clone, Eq, Ord, PartialOrd, PartialEq)]
enum Op {
    Insert,
    Update,
    PointDelete,
    PointDeleteEmpty,
    RangeDelete,
    PointQuery,
    EmptyPointQuery,
    RangeQuery,
}

#[inline]
fn gen_string(rng: &mut Xoshiro256Plus, len: usize) -> Key {
    return rng.sample_iter(Alphanumeric).take(len).collect();
}

/// Generates a workload given the spec and writes it to the given writer.
pub fn write_operations(writer: &mut impl Write, workload: &WorkloadSpec) -> Result<()> {
    write_operations_with_keyset(writer, workload, VecKeySet::new)
}

pub fn write_operations_with_keyset<KeySetT: KeySet>(
    writer: &mut impl Write,
    workload: &WorkloadSpec,
    keyset_constructor: impl Fn(usize) -> KeySetT,
) -> Result<()> {
    let mut rng = Xoshiro256Plus::from_os_rng();
    // let mut keys_prev_sections = BloomFilter::with_rate(0.01, todo!());

    for section in &workload.sections {
        let mut keys_valid = keyset_constructor(0 /*section.insert_count()*/);

        for group in &section.groups {
            let rng_ref = &mut rng;
            let mut markers: Vec<Op> = Vec::with_capacity(0 /*group.operation_count()*/);

            let insert_count = group
                .inserts
                .as_ref()
                .map_or(0, |is| is.amount.evaluate(rng_ref) as usize);
            let update_count = group
                .updates
                .as_ref()
                .map_or(0, |us| us.amount.evaluate(rng_ref) as usize);
            let delete_point_count = group
                .point_deletes
                .as_ref()
                .map_or(0, |dps| dps.amount.evaluate(rng_ref) as usize);
            let delete_point_empty_count = group
                .empty_point_deletes
                .as_ref()
                .map_or(0, |dpes| dpes.amount.evaluate(rng_ref) as usize);
            let delete_range_count = group
                .range_deletes
                .as_ref()
                .map_or(0, |drs| drs.amount.evaluate(rng_ref) as usize);
            let query_point_count = group
                .point_queries
                .as_ref()
                .map_or(0, |drs| drs.amount.evaluate(rng_ref) as usize);
            let query_point_empty_count = group
                .empty_point_queries
                .as_ref()
                .map_or(0, |qpes| qpes.amount.evaluate(rng_ref) as usize);
            let query_range_count = group
                .range_queries
                .as_ref()
                .map_or(0, |drs| drs.amount.evaluate(rng_ref) as usize);

            let delete_range_sel_expected = group
                .range_deletes
                .as_ref()
                .map_or(0.0, |drs| drs.selectivity.expected_value());

            let more_delete_point_than_keys = delete_point_count > keys_valid.len();
            fn remove_items(initial: usize, selectivity: f32, removals: usize) -> usize {
                let mut current = initial;

                for _ in 0..removals {
                    let to_remove = (current as f32 * selectivity) as usize;
                    current = current.saturating_sub(to_remove);

                    if current == 0 {
                        break;
                    }
                }
                return current;
            }
            let more_delete_range_than_keys = remove_items(
                keys_valid.len(),
                delete_range_sel_expected,
                delete_range_count,
            ) == 0;
            if more_delete_point_than_keys {
                bail!("Cannot have more point deletes than existing valid keys.");
            }
            if more_delete_range_than_keys {
                bail!("Cannot have more range deletes than existing valid keys.");
            }

            // A group must have at least 1 valid key before any other operation can occur.
            if keys_valid.is_empty() {
                if insert_count == 0 {
                    bail!(
                        "Invalid workload spec. Group must have existing valid keys or have insert operations."
                    );
                }
                let is = group
                    .inserts
                    .as_ref()
                    .expect("inserts to exist if insert count > 0");
                markers.extend(repeat_n(Op::Insert, insert_count - 1));

                let key_len = is.key_len.evaluate(rng_ref) as usize;
                let key = gen_string(rng_ref, key_len);
                let val_len = is.val_len.evaluate(rng_ref) as usize;
                let val = gen_string(rng_ref, val_len);
                AsciiOperationFormatter::write_insert(writer, &key, &val)?;
                keys_valid.push(key);
            } else {
                markers.extend(repeat_n(Op::Insert, insert_count));
            }
            markers.extend(repeat_n(Op::Update, update_count));
            markers.extend(repeat_n(Op::PointDelete, delete_point_count));
            markers.extend(repeat_n(Op::PointDeleteEmpty, delete_point_empty_count));
            markers.extend(repeat_n(Op::RangeDelete, delete_range_count));
            markers.extend(repeat_n(Op::PointQuery, query_point_count));
            markers.extend(repeat_n(Op::EmptyPointQuery, query_point_empty_count));
            markers.extend(repeat_n(Op::RangeQuery, query_range_count));

            for marker in markers {
                match marker {
                    Op::Insert => {
                        let is = group.inserts.as_ref().ok_or_else(|| {
                            anyhow!("Insert marker can only appear when inserts is not None")
                        })?;
                        let key_len = is.key_len.evaluate(rng_ref) as usize;
                        let key = gen_string(rng_ref, key_len);
                        let len1 = is.val_len.evaluate(rng_ref) as usize;
                        let val = gen_string(rng_ref, len1);
                        AsciiOperationFormatter::write_insert(writer, &key, &val)?;
                        keys_valid.push(key);
                    }
                    Op::Update => {
                        let us = group.updates.as_ref().ok_or_else(|| {
                            anyhow!("Update marker can only appear when updates is not None")
                        })?;
                        if keys_valid.is_empty() {
                            bail!("Cannot have updates when there are no valid keys.");
                        }
                        let key = keys_valid.get_random(rng_ref);
                        let len = us.val_len.evaluate(rng_ref) as usize;
                        let val = gen_string(rng_ref, len);

                        AsciiOperationFormatter::write_update(writer, key, &val)?;
                    }
                    Op::PointDelete => {
                        // let idx = rng_ref.random_range(0..keys_valid.len());
                        // let key = keys_valid.remove(idx);
                        let key = keys_valid.remove_random(rng_ref);

                        AsciiOperationFormatter::write_point_delete(writer, &key)?;
                    }
                    Op::PointQuery => {
                        if keys_valid.is_empty() {
                            bail!("Cannot have point queries when there are no valid keys.");
                        }
                        let key = keys_valid.get_random(rng_ref);
                        AsciiOperationFormatter::write_point_query(writer, key)?
                    }
                    Op::PointDeleteEmpty => {
                        let epd = group.empty_point_deletes.as_ref().ok_or_else(|| {
                            anyhow!("Empty point delete marker can only appear when empty_point_deletes is not None")
                        })?;
                        let key = loop {
                            let len = epd.key_len.evaluate(rng_ref) as usize;
                            let key = gen_string(rng_ref, len);
                            if !keys_valid.contains(&key) {
                                break key;
                            }
                        };

                        AsciiOperationFormatter::write_point_delete(writer, &key)?
                    }
                    Op::EmptyPointQuery => {
                        let epq = group.empty_point_queries.as_ref().ok_or_else(|| {
                            anyhow!("Empty point query marker can only appear when empty_point_queries is not None")
                        })?;
                        let key = loop {
                            let len = epq.key_len.evaluate(rng_ref) as usize;
                            let key = gen_string(rng_ref, len);
                            if !keys_valid.contains(&key) {
                                break key;
                            }
                        };

                        AsciiOperationFormatter::write_point_query(writer, &key)?
                    }
                    Op::RangeQuery => {
                        let rqs = group.range_queries.as_ref().ok_or_else(|| {
                            anyhow!(
                                "Range query marker can only appear when range_queries is not None"
                            )
                        })?;
                        if keys_valid.is_empty() {
                            bail!("Cannot have range queries when there are no valid keys.");
                        }

                        keys_valid.sort();
                        // It would be better to use `from` and `try_from` instead of `as` here.
                        // Maybe the `num_traits` crate could help.
                        // https://doc.rust-lang.org/reference/expressions/operator-expr.html#r-expr.as.numeric.float-as-int
                        // let selectivity = match rqs.selectivity {
                        //     spec::Expr::Constant(s) => s,
                        //     spec::Expr::Sampled(distribution) => distribution.evaluate(rng_ref),
                        // }:
                        let num_items = (rqs.selectivity.evaluate(rng_ref)
                            * (keys_valid.len() as f32).floor())
                            as usize;
                        let start_range = 0..keys_valid.len() - num_items;

                        let start_idx = rng_ref.random_range(start_range);
                        let key1 = keys_valid.get(start_idx).expect("index to be in range");
                        let key2 = keys_valid
                            .get(start_idx + num_items)
                            .expect("index to be in range");

                        AsciiOperationFormatter::write_range_query(writer, key1, key2)?
                    }
                    Op::RangeDelete => {
                        let rds = group.range_deletes.as_ref().ok_or_else(|| {
                            anyhow!(
                                "RangeDelete marker can only appear when range_deletes is not None",
                            )
                        })?;
                        if keys_valid.is_empty() {
                            bail!("Cannot have range deletes when there are no valid keys.");
                        }

                        keys_valid.sort();
                        let num_items = (rds.selectivity.evaluate(rng_ref)
                            * (keys_valid.len() as f32).floor())
                            as usize;
                        let start_range = 0..keys_valid.len() - num_items;

                        // let start_idx = rng_ref.random_range(start_range);
                        // let key1 = keys_valid.get_random(start_range);
                        let (key1, start_idx) = loop {
                            let start_idx = rng_ref.random_range(start_range.clone());
                            match keys_valid.get(start_idx) {
                                Some(key) => break (key, start_idx),
                                None => continue,
                            }
                        };
                        let key2 = keys_valid
                            .get(start_idx + num_items)
                            .expect("index to be in range");

                        AsciiOperationFormatter::write_range_delete(writer, key1, key2)?;

                        keys_valid.remove_range(start_idx..start_idx + num_items);
                    }
                }
            }
        }
    }

    return Ok(());
}

/// Takes in a JSON representation of a workload specification and writes the workload to a file.
pub fn generate_workload(workload_spec_string: &str, output_file: PathBuf) -> Result<()> {
    let workload_spec: WorkloadSpec =
        serde_json::from_str(workload_spec_string).context("Parsing spec file")?;
    let mut buf_writer = BufWriter::with_capacity(1024 * 1024, File::create(output_file)?);
    write_operations(&mut buf_writer, &workload_spec)?;
    buf_writer.flush()?;

    Ok(())
}

pub fn generate_workload_spec_schema() -> serde_json::Result<String> {
    let schema = schemars::schema_for!(crate::spec::WorkloadSpec);
    return serde_json::to_string_pretty(&schema);
}
