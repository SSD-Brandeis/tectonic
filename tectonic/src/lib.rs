#![feature(extend_one)]
#![feature(btree_cursors)]
#![feature(trusted_random_access)]
#![feature(trait_alias)]
#![allow(clippy::needless_return)]
#![allow(dead_code)]

use anyhow::{Context, Result, anyhow, bail};
use rand::distr::Alphanumeric;
use rand::prelude::SliceRandom;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::iter::repeat_n;
use std::path::PathBuf;
use tracing::debug;

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
use crate::spec::{CharacterSet, StringExpr, WorkloadSpec};

struct AsciiOperationFormatter;
impl AsciiOperationFormatter {
    fn write_insert(
        w: &mut impl Write,
        rng: &mut impl Rng,
        key: &Key,
        val: &StringExpr,
        character_set: Option<CharacterSet>,
    ) -> Result<()> {
        w.write_all("I ".as_bytes())?;
        w.write_all(key)?;
        w.write_all(" ".as_bytes())?;
        val.write_all(w, rng, character_set)?;
        w.write_all("\n".as_bytes())?;

        return Ok(());
    }
    fn write_update(
        w: &mut impl Write,
        rng: &mut impl Rng,
        key: &Key,
        val: &StringExpr,
        character_set: Option<CharacterSet>,
    ) -> Result<()> {
        w.write_all("U ".as_bytes())?;
        w.write_all(key)?;
        w.write_all(" ".as_bytes())?;
        val.write_all(w, rng, character_set)?;
        w.write_all("\n".as_bytes())?;

        return Ok(());
    }
    fn write_merge(
        w: &mut impl Write,
        rng: &mut impl Rng,
        key: &Key,
        val: &StringExpr,
        character_set: Option<CharacterSet>,
    ) -> Result<()> {
        w.write_all("M ".as_bytes())?;
        w.write_all(key)?;
        w.write_all(" ".as_bytes())?;
        val.write_all(w, rng, character_set)?;
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
    Merge,
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
            let character_set = group
                .character_set
                .or(section.character_set)
                .or(workload.character_set);

            let insert_count = group
                .inserts
                .as_ref()
                .map_or(0, |is| is.amount.evaluate(rng_ref) as usize);
            let update_count = group
                .updates
                .as_ref()
                .map_or(0, |us| us.amount.evaluate(rng_ref) as usize);
            let merge_count = group
                .merges
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

            debug!(
                ?insert_count,
                ?update_count,
                ?merge_count,
                ?delete_point_count,
                ?delete_point_empty_count,
                ?delete_range_count,
                ?query_point_count,
                ?query_point_empty_count,
                ?query_range_count
            );

            let more_delete_point_than_keys = delete_point_count > keys_valid.len();
            if more_delete_point_than_keys {
                bail!("Cannot have more point deletes than existing valid keys.");
            }

            let mut key_pool = if let Some(sorted) = &group.sorted {
                let is = group
                    .inserts
                    .as_ref()
                    .ok_or_else(|| anyhow!("Insert spec must exist if sorted config exists"))?;
                let mut pool = Vec::with_capacity(insert_count);
                for _ in 0..insert_count {
                    let key = is.key.generate(rng_ref, is.character_set.or(character_set));
                    pool.push(key);
                }

                // reverse sort so that we can pop from the end
                pool.sort_by(|a, b| b.cmp(a));

                let k = sorted.k.evaluate(rng_ref) as usize;
                for _ in 0..(k / 2) {
                    // clamp bounds are [idx-l = 0, idx+l = pool.len() - 1]
                    let idx = rng_ref.random_range(0..pool.len()) as isize;
                    let l = (sorted.l.evaluate(rng_ref) as isize)
                        .clamp(-idx, pool.len() as isize - 1 - idx);
                    pool.swap(idx as usize, (idx + l) as usize);
                }
                Some(pool)
            } else {
                None
            };

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

                // let key_len = is.key_len.evaluate(rng_ref) as usize;
                // let key = gen_string(rng_ref, key_len);
                // let val_len = is.val_len.evaluate(rng_ref) as usize;
                // let val = gen_string(rng_ref, val_len);
                let key = key_pool
                    .as_mut()
                    .and_then(|pool| pool.pop())
                    .unwrap_or_else(|| {
                        is.key.generate(rng_ref, is.character_set.or(character_set))
                    });
                // let key = is.key.generate(rng_ref, is.character_set);
                AsciiOperationFormatter::write_insert(
                    writer,
                    rng_ref,
                    &key,
                    &is.val,
                    is.character_set.or(character_set),
                )?;
                keys_valid.push(key);
            } else {
                markers.extend(repeat_n(Op::Insert, insert_count));
            }
            markers.extend(repeat_n(Op::Update, update_count));
            markers.extend(repeat_n(Op::Merge, merge_count));
            markers.extend(repeat_n(Op::PointDelete, delete_point_count));
            markers.extend(repeat_n(Op::PointDeleteEmpty, delete_point_empty_count));
            markers.extend(repeat_n(Op::RangeDelete, delete_range_count));
            markers.extend(repeat_n(Op::PointQuery, query_point_count));
            markers.extend(repeat_n(Op::EmptyPointQuery, query_point_empty_count));
            markers.extend(repeat_n(Op::RangeQuery, query_range_count));
            markers.shuffle(rng_ref);

            for marker in markers {
                match marker {
                    Op::Insert => {
                        let is = group.inserts.as_ref().ok_or_else(|| {
                            anyhow!("Insert marker can only appear when inserts is not None")
                        })?;
                        let key = key_pool
                            .as_mut()
                            .and_then(|pool| pool.pop())
                            .unwrap_or_else(|| {
                                is.key.generate(rng_ref, is.character_set.or(character_set))
                            });
                        // let key = is.key.generate(rng_ref, is.character_set);
                        AsciiOperationFormatter::write_insert(
                            writer,
                            rng_ref,
                            &key,
                            &is.val,
                            is.character_set.or(character_set),
                        )?;
                        keys_valid.push(key);
                    }
                    Op::Update => {
                        let us = group.updates.as_ref().ok_or_else(|| {
                            anyhow!("Update marker can only appear when updates is not None")
                        })?;
                        if keys_valid.is_empty() {
                            bail!("Cannot have updates when there are no valid keys.");
                        }
                        keys_valid.sort(us.sort_by);
                        let key = keys_valid.get_random(rng_ref, &us.selection);
                        AsciiOperationFormatter::write_update(
                            writer,
                            rng_ref,
                            key,
                            &us.val,
                            us.character_set.or(character_set),
                        )?;
                    }
                    Op::Merge => {
                        let ms = group.merges.as_ref().ok_or_else(|| {
                            anyhow!("Merge marker can only appear when updates is not None")
                        })?;
                        if keys_valid.is_empty() {
                            bail!("Cannot have merges when there are no valid keys.");
                        }
                        keys_valid.sort(ms.sort_by);
                        let key = keys_valid.get_random(rng_ref, &ms.selection);
                        AsciiOperationFormatter::write_merge(
                            writer,
                            rng_ref,
                            key,
                            &ms.val,
                            ms.character_set.or(character_set),
                        )?;
                    }
                    Op::PointDelete => {
                        let pds = group.point_deletes.as_ref().ok_or_else(|| {
                            anyhow!("Point delete marker can only appear when updates is not None")
                        })?;
                        keys_valid.sort(pds.sort_by);
                        let key = keys_valid.remove_random(rng_ref, &pds.selection);

                        AsciiOperationFormatter::write_point_delete(writer, &key)?;
                    }
                    Op::PointQuery => {
                        if keys_valid.is_empty() {
                            bail!("Cannot have point queries when there are no valid keys.");
                        }
                        let pqs = group.point_queries.as_ref().ok_or_else(|| {
                            anyhow!("Point query marker can only appear when updates is not None")
                        })?;
                        keys_valid.sort(pqs.sort_by);
                        let key = keys_valid.get_random(rng_ref, &pqs.selection);
                        AsciiOperationFormatter::write_point_query(writer, key)?
                    }
                    Op::PointDeleteEmpty => {
                        let epd = group.empty_point_deletes.as_ref().ok_or_else(|| {
                            anyhow!("Empty point delete marker can only appear when empty_point_deletes is not None")
                        })?;
                        let key = loop {
                            let k = epd
                                .key
                                .generate(rng_ref, epd.character_set.or(character_set));
                            if !keys_valid.contains(&k) {
                                break k;
                            }
                        };

                        AsciiOperationFormatter::write_point_delete(writer, &key)?
                    }
                    Op::EmptyPointQuery => {
                        let epq = group.empty_point_queries.as_ref().ok_or_else(|| {
                            anyhow!("Empty point query marker can only appear when empty_point_queries is not None")
                        })?;
                        let key = loop {
                            let k = epq
                                .key
                                .generate(rng_ref, epq.character_set.or(character_set));
                            if !keys_valid.contains(&k) {
                                break k;
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

                        keys_valid.sort(rqs.sort_by);
                        let (key1, key2) = keys_valid.get_range_random(
                            rqs.selectivity.evaluate(rng_ref),
                            rng_ref,
                            &rqs.selection,
                        );

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

                        keys_valid.sort(rds.sort_by);
                        let (key1, key2) = keys_valid.remove_range_random(
                            rds.selectivity.evaluate(rng_ref),
                            rng_ref,
                            &rds.selection,
                        );
                        AsciiOperationFormatter::write_range_delete(writer, &key1, &key2)?;
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
    let schema = schemars::schema_for!(WorkloadSpec);
    return serde_json::to_string_pretty(&schema);
}
