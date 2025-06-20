#![allow(clippy::needless_return)]

use anyhow::{Context, Result};
use rand::{Rng, SeedableRng};

use crate::keyset::Key;
use rand::distr::weighted::WeightedIndex;
use rand::distr::Alphanumeric;
use rand_distr::Distribution as _;
use rand_xoshiro::Xoshiro256Plus;
use schemars::JsonSchema;
use schemars::Schema;
use schemars::SchemaGenerator;
use std::borrow::Cow;
use std::io::Write;
// pub trait Evaluate {
//     /// Evaluates the expression to a value.
//     fn evaluate(&self, rng: &mut impl Rng) -> f32;
//
//     /// Returns the expected value of the expression.
//     fn expected_value(&self) -> f32;
// }

#[derive(serde::Deserialize, JsonSchema, Clone, Debug)]
#[serde(rename_all = "snake_case")]
enum DistributionConfig {
    Uniform { min: f32, max: f32 },
    Normal { mean: f32, std_dev: f32 },
    Exponential { lambda: f32 },
    Beta { alpha: f32, beta: f32 },
    Zipf { n: usize, s: f32 },
}

#[derive(serde::Deserialize, Clone, Debug)]
#[serde(try_from = "DistributionConfig")]
/// Different types of distributions that can be used to sample values.
pub enum Distribution {
    /// Uniform distribution over the range [min, max).
    Uniform {
        min: f32,
        max: f32,
        distr: rand_distr::Uniform<f32>,
    },
    /// Normal distribution with the given mean and standard deviation.
    Normal {
        mean: f32,
        std_dev: f32,
        distr: rand_distr::Normal<f32>,
    },
    /// Exponential distribution with the given lambda parameter.
    Exponential {
        lambda: f32,
        distr: rand_distr::Exp<f32>,
    },
    /// Beta distribution with the given alpha and beta parameters.
    Beta {
        alpha: f32,
        beta: f32,
        distr: rand_distr::Beta<f32>,
    },
    /// Zipf distribution with the given n and s parameters.
    Zipf {
        n: usize,
        s: f32,
        distr: rand_distr::Zipf<f32>,
    },
}

impl TryFrom<DistributionConfig> for Distribution {
    type Error = anyhow::Error;

    fn try_from(value: DistributionConfig) -> Result<Self, Self::Error> {
        use DistributionConfig as DC;
        let distr = match value {
            DC::Uniform { min, max } => Self::Uniform {
                min,
                max,
                distr: rand_distr::Uniform::new(min, max)?,
            },
            DC::Normal { mean, std_dev } => Self::Normal {
                mean,
                std_dev,
                distr: rand_distr::Normal::new(mean, std_dev)?,
            },
            DC::Exponential { lambda } => Self::Exponential {
                lambda,
                distr: rand_distr::Exp::new(lambda)?,
            },
            DC::Beta { alpha, beta } => Self::Beta {
                alpha,
                beta,
                distr: rand_distr::Beta::new(alpha, beta)?,
            },
            DC::Zipf { n, s } => Self::Zipf {
                n,
                s,
                distr: rand_distr::Zipf::new(n as f32, s)?,
            },
        };
        return Ok(distr);
    }
}

impl JsonSchema for Distribution {
    fn schema_name() -> Cow<'static, str> {
        "Distribution".into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        return DistributionConfig::json_schema(generator);
    }
}

fn generalized_harmonic(n: usize, s: f32) -> f32 {
    (1..=n).map(|k| 1.0 / (k as f32).powf(s)).sum()
}

impl Distribution {
    fn evaluate(&self, rng: &mut impl Rng) -> f32 {
        return match self {
            Self::Uniform { distr, .. } => distr.sample(rng),
            Self::Normal { distr, .. } => distr.sample(rng),
            Self::Exponential { distr, .. } => distr.sample(rng),
            Self::Beta { distr, .. } => distr.sample(rng),
            Self::Zipf { distr, .. } => distr.sample(rng),
        };
    }

    fn expected_value(&self) -> f32 {
        return match self {
            Distribution::Uniform { min, max, .. } => min + max / 2.0,
            Distribution::Normal { mean, .. } => *mean,
            Distribution::Exponential { lambda, .. } => 1.0 / lambda,
            Distribution::Beta { alpha, beta, .. } => alpha / (alpha + beta),
            Distribution::Zipf { s, n, .. } => {
                let hs = generalized_harmonic(*n, *s);
                let hs_minus1 = generalized_harmonic(*n, *s - 1.0);
                return hs_minus1 / hs;
            }
        };
    }
}

// No docstring
#[derive(serde::Deserialize, JsonSchema, Clone, Debug)]
#[serde(untagged)]
pub enum NumberExpr {
    Constant(f32),
    Sampled(Distribution),
}

impl NumberExpr {
    /// Evaluates the expression to a value.
    pub fn evaluate(&self, rng: &mut impl Rng) -> f32 {
        match self {
            Self::Constant(val) => *val,
            Self::Sampled(dist) => dist.evaluate(rng),
        }
    }

    /// Expected value of the expression.
    pub fn expected_value(&self) -> f32 {
        match self {
            Self::Constant(val) => *val,
            Self::Sampled(dist) => dist.expected_value(),
        }
    }
}

#[derive(serde::Deserialize, JsonSchema, Default, Clone, Debug)]
#[serde(rename_all = "snake_case")]
/// Different selection strategies for keys in a workload.
pub enum KeyDistribution {
    #[default]
    Uniform,
}

#[derive(serde::Deserialize, JsonSchema, Clone, Debug)]
pub struct Weight {
    /// The weight of the item.
    pub weight: f32,
    /// The value of the item.
    pub value: StringExpr,
}

#[derive(serde::Deserialize, JsonSchema, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum StringExprConfig {
    Constant(String),
    Sampled {
        /// The distribution to use for sampling the string.
        distribution: Distribution,
        /// The length of the string to sample.
        length: NumberExpr,
        /// The character set to use for sampling the string.
        character_set: CharacterSet,
    },
    Weighted(Vec<Weight>),
    Segmented {
        separator: String,
        /// The segments to use for the string.
        segments: Vec<StringExpr>,
    },
    HotRange {
        length: usize,
        count: usize,
        probability: f32,
    },
}

#[derive(serde::Deserialize, Clone, Debug)]
#[serde(try_from = "StringExprConfig")]
pub enum StringExpr {
    Constant(String),
    Sampled {
        /// The distribution to use for sampling the string.
        distribution: Distribution,
        /// The length of the string to sample.
        length: NumberExpr,
        /// The character set to use for sampling the string.
        character_set: CharacterSet,
    },
    Weighted {
        items: Vec<Weight>,
        distr: WeightedIndex<f32>,
    },
    Segmented {
        separator: String,
        /// The segments to use for the string.
        segments: Vec<StringExpr>,
    },
    HotRange {
        length: usize,
        count: usize,
        probability: f32,
        hot_ranges: Vec<Key>,
    },
}

impl JsonSchema for StringExpr {
    fn schema_name() -> Cow<'static, str> {
        "StringExpr".into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        return StringExprConfig::json_schema(generator);
    }
}

impl TryFrom<StringExprConfig> for StringExpr {
    type Error = anyhow::Error;

    fn try_from(value: StringExprConfig) -> Result<Self, Self::Error> {
        return match value {
            StringExprConfig::Constant(val) => Ok(Self::Constant(val)),
            StringExprConfig::Sampled {
                distribution,
                length,
                character_set,
            } => Ok(Self::Sampled {
                distribution,
                length,
                character_set,
            }),
            StringExprConfig::Weighted(items) => {
                let weights = items.iter().map(|w| w.weight).collect::<Vec<_>>();
                let distr = WeightedIndex::new(&weights).context("Building weighted index")?;
                Ok(Self::Weighted { items, distr })
            }
            StringExprConfig::Segmented {
                separator,
                segments,
            } => Ok(Self::Segmented {
                separator,
                segments,
            }),
            StringExprConfig::HotRange {
                length,
                count,
                probability,
            } => {
                let mut rng = Xoshiro256Plus::from_os_rng();
                let rng_ref = &mut rng;
                let mut hot_ranges = Vec::with_capacity(count);
                for _ in 0..count {
                    let key: Box<[u8]> = rng_ref.sample_iter(Alphanumeric).take(length).collect();
                    hot_ranges.push(key);
                }
                Ok(Self::HotRange {
                    length,
                    count,
                    probability,
                    hot_ranges,
                })
            }
        };
    }
}

impl StringExpr {
    pub fn generate(&self, rng: &mut impl Rng) -> Key {
        return match self {
            Self::Constant(val) => Key::from(val.as_bytes()),
            Self::Sampled {
                distribution: _,
                length,
                character_set,
            } => {
                let len = length.evaluate(rng) as usize;
                let distr = match character_set {
                    CharacterSet::Alphanumeric => Alphanumeric,
                };
                Key::from_iter(rng.sample_iter(distr).take(len))
            }
            Self::Weighted { items, distr } => {
                let random_value = rng.sample(distr);
                let item = &items[random_value];
                item.value.generate(rng)
            }
            Self::Segmented {
                separator,
                segments,
            } => {
                let mut buf = Vec::new();
                for segment in segments {
                    segment
                        .write_all(&mut buf, rng)
                        .context("Writing weighted string")
                        .expect("to be able to write to a vec");
                    buf.extend(separator.as_bytes());
                }
                Key::from(buf)
            }
            Self::HotRange { hot_ranges, probability, length, .. } => {
                let is_hot = rng.random_bool(*probability as f64);
                return if is_hot {
                    let index = rng.random_range(0..hot_ranges.len());
                    hot_ranges[index].clone()
                } else {
                    let key: Box<[u8]> = rng.sample_iter(Alphanumeric).take(*length).collect();
                    Key::from(key)
                };
            }
        };
    }
    /// Evaluates the expression to a value.
    pub fn write_all(&self, writer: &mut impl Write, rng: &mut impl Rng) -> Result<()> {
        match self {
            Self::Constant(val) => writer
                .write_all(val.as_bytes())
                .context("Writing constant string"),
            Self::Sampled {
                distribution: _,
                length,
                character_set,
            } => {
                let len = length.evaluate(rng) as usize;
                let distr = match character_set {
                    CharacterSet::Alphanumeric => Alphanumeric,
                };
                for ch in rng.sample_iter(distr).take(len) {
                    writer.write_all(&[ch]).context("Writing sampled string")?;
                }
                return Ok(());
            }
            Self::Weighted{items, distr} => {
                let random_value = rng.sample(distr);
                let item = &items[random_value];
                return item
                    .value
                    .write_all(writer, rng)
                    .context("Writing weighted string");
            }
            Self::Segmented {
                separator,
                segments,
            } => {
                for segment in segments {
                    segment
                        .write_all(writer, rng)
                        .context("Writing weighted string")?;
                    writer
                        .write_all(separator.as_bytes())
                        .context("Writing separator")?;
                }
                return Ok(());
            },
            Self::HotRange { hot_ranges, probability, length, .. } => {
                let is_hot = rng.random_bool(*probability as f64);
                let key = if is_hot {
                    let index = rng.random_range(0..hot_ranges.len());
                    hot_ranges[index].clone()
                } else {
                    let key: Box<[u8]> = rng.sample_iter(Alphanumeric).take(*length).collect();
                    Key::from(key)
                };
                writer.write_all(&key).context("Writing weighted string")
            }
        }
    }
}

#[derive(serde::Deserialize, JsonSchema, Clone, Debug)]
/// Inserts specification.
pub struct Inserts {
    /// Number of inserts
    pub amount: NumberExpr,
    /// Key
    pub key: StringExpr,
    /// Value
    pub val: StringExpr,
}

#[derive(serde::Deserialize, JsonSchema, Clone, Debug)]
/// Updates specification.
pub struct Updates {
    /// Number of updates
    pub amount: NumberExpr,
    /// Value
    pub val: StringExpr,
}

#[derive(serde::Deserialize, JsonSchema, Clone, Debug)]
/// Non-empty point deletes specification.
pub struct PointDeletes {
    /// Number of non-empty point deletes
    pub amount: NumberExpr,
}

#[derive(serde::Deserialize, JsonSchema, Clone, Debug)]
/// Empty point deletes specification.
pub struct EmptyPointDeletes {
    /// Number of empty point deletes
    pub amount: NumberExpr,
    /// Key
    pub key: StringExpr,
}
#[derive(serde::Deserialize, JsonSchema, Clone, Debug)]
/// Range deletes specification.
pub struct RangeDeletes {
    /// Number of range deletes
    pub amount: NumberExpr,
    /// Selectivity of range deletes. Based off of the range of valid keys, not the full key space.
    pub selectivity: NumberExpr,
}

#[derive(serde::Deserialize, JsonSchema, Clone, Debug)]
/// Non-empty point queries specification.
pub struct PointQueries {
    /// Number of point queries
    pub amount: NumberExpr,
}

#[derive(serde::Deserialize, JsonSchema, Clone, Debug)]
/// Empty point queries specification.
pub struct EmptyPointQueries {
    /// Number of point queries
    pub amount: NumberExpr,
    /// Key
    pub key: StringExpr,
}

#[derive(serde::Deserialize, JsonSchema, Clone, Debug)]
/// Range queries specification.
pub struct RangeQueries {
    /// Number of range queries
    pub amount: NumberExpr,
    /// Selectivity of range queries. Based off of the range of valid keys, not the full key-space.
    pub selectivity: NumberExpr,
}

#[derive(serde::Deserialize, JsonSchema, Clone, Debug)]
pub struct WorkloadSpecGroup {
    pub inserts: Option<Inserts>,
    pub updates: Option<Updates>,
    pub point_deletes: Option<PointDeletes>,
    pub empty_point_deletes: Option<EmptyPointDeletes>,
    pub range_deletes: Option<RangeDeletes>,
    pub point_queries: Option<PointQueries>,
    pub empty_point_queries: Option<EmptyPointQueries>,
    pub range_queries: Option<RangeQueries>,
}

// impl WorkloadSpecGroup {
//     pub fn operation_count(&self) -> usize {
//         let operation_count = self.inserts.map_or(0, |s| s.amount)
//             + self.updates.map_or(0, |us| us.amount)
//             + self.point_queries.map_or(0, |is| is.amount)
//             + self.empty_point_queries.map_or(0, |is| is.amount)
//             + self.range_queries.map_or(0, |is| is.amount)
//             + self.point_deletes.map_or(0, |is| is.amount)
//             + self.range_deletes.map_or(0, |is| is.amount);
//         return operation_count;
//     }
//
//     pub fn bytes_count(&self, insert_key_len: usize) -> usize {
//         let bytes_insert = self.inserts.map_or(0, |is| {
//             (b"I ".len() + is.key_len + b" ".len() + is.val_len + b"\n".len()) * is.amount
//         });
//         let bytes_update = self.updates.map_or(0, |us| {
//             (b"U ".len() + insert_key_len + b" ".len() + us.val_len + b"\n".len()) * us.amount
//         });
//         let bytes_delete = self.point_deletes.map_or(0, |ds| {
//             (b"D ".len() + insert_key_len + b"\n".len()) * ds.amount
//         });
//         let bytes_point_queries = self.point_queries.map_or(0, |pq| {
//             (b"P ".len() + insert_key_len + b"\n".len()) * pq.amount
//         });
//         let bytes_empty_point_queries = self.empty_point_queries.map_or(0, |epq| {
//             (b"P ".len() + epq.key_len + b"\n".len()) * epq.amount
//         });
//         let bytes_range_queries = self.range_queries.map_or(0, |rq| {
//             (b"S ".len() + insert_key_len + b" ".len() + insert_key_len + b"\n".len())
//                 * rq.amount
//         });
//         let bytes_range_deletes = self.range_deletes.map_or(0, |rd| {
//             (b"S ".len() + insert_key_len + b" ".len() + insert_key_len + b"\n".len())
//                 * rd.amount
//         });
//         return bytes_insert
//             + bytes_update
//             + bytes_delete
//             + bytes_point_queries
//             + bytes_empty_point_queries
//             + bytes_range_queries
//             + bytes_range_deletes;
//     }
// }

#[derive(serde::Deserialize, JsonSchema, Default, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum CharacterSet {
    #[default]
    Alphanumeric,
    // AlphanumericLower,
    // AlphanumericUpper,
    // Alphabetic,
    // AlphabeticLower,
    // AlphabeticUpper,
    // Numeric,
    // Hexadecimal,
    // Utf8,
}

#[derive(serde::Deserialize, JsonSchema, Clone, Debug)]
pub struct WorkloadSpecSection {
    /// A list of groups. Groups share valid keys between operations.
    ///
    /// E.g., non-empty point queries will use a key from an insert in this group.
    pub groups: Vec<WorkloadSpecGroup>,
    /// The domain from which the keys will be created from.
    #[serde(default = "CharacterSet::default")]
    pub character_set: CharacterSet,
    /// The domain from which the keys will be created from.
    #[serde(default = "KeyDistribution::default")]
    pub key_distribution: KeyDistribution,
    /// Whether to skip the check that a generated key is in the valid key set for inserts and empty point queries/deletes.
    ///
    /// This is useful when the keyspace is much larger than the number of keys being generated, as it can greatly decrease generation time.
    #[serde(default)]
    pub skip_key_contains_check: bool,
}

// impl WorkloadSpecSection {
//     pub fn operation_count(&self) -> usize {
//         return self.groups.iter().map(|g| g.operation_count()).sum();
//     }
//
//     pub fn bytes_count(&self) -> usize {
//         let insert_key_len = self
//             .groups
//             .iter()
//             .map(|g| g.inserts.map_or(0, |is| is.key_len))
//             .max()
//             .expect("No groups in workload spec");
//         return self
//             .groups
//             .iter()
//             .map(|g| g.bytes_count(insert_key_len))
//             .sum();
//     }
//
//     pub fn insert_count(&self) -> usize {
//         return self
//             .groups
//             .iter()
//             .map(|g| g.inserts.map_or(0, |is| is.amount))
//             .sum();
//     }
// }

#[derive(serde::Deserialize, JsonSchema, Debug, Clone)]
pub struct WorkloadSpec {
    /// Sections of a workload where a key from one will (probably) not appear in another.
    pub sections: Vec<WorkloadSpecSection>,
}

// impl WorkloadSpec {
//     pub fn operation_count(&self) -> usize {
//         return self.sections.iter().map(|s| s.operation_count()).sum();
//     }
//
//     pub fn bytes_count(&self) -> usize {
//         return self.sections.iter().map(|s| s.bytes_count()).sum();
//     }
// }
