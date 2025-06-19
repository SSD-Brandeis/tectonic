#![allow(clippy::needless_return)]

use anyhow::Result;
use rand::Rng;

use rand_distr::Distribution as _;
use schemars::JsonSchema;
use schemars::Schema;
use schemars::SchemaGenerator;
use std::borrow::Cow;

pub(crate) trait Evaluate {
    /// Evaluates the expression to a value.
    fn evaluate(&self, rng: &mut impl Rng) -> f32;

    /// Returns the expected value of the expression.
    fn expected_value(&self) -> f32;
}

#[derive(serde::Deserialize, JsonSchema, Clone, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
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

impl Evaluate for Distribution {
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
pub enum Expr {
    Constant(f32),
    Sampled(Distribution),
}

impl Expr {
    /// Evaluates the expression to a value.
    pub fn evaluate(&self, rng: &mut impl Rng) -> f32 {
        use Expr as E;
        match self {
            E::Constant(val) => *val,
            E::Sampled(dist) => dist.evaluate(rng),
        }
    }

    /// Expected value of the expression.
    pub fn expected_value(&self) -> f32 {
        use Expr as E;
        match self {
            E::Constant(val) => *val,
            E::Sampled(dist) => dist.expected_value(),
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
/// Inserts specification.
pub struct Inserts {
    /// Number of inserts
    pub(crate) amount: Expr,
    /// Key length
    pub(crate) key_len: Expr,
    /// Value length
    pub(crate) val_len: Expr,
}

#[derive(serde::Deserialize, JsonSchema, Clone, Debug)]
/// Updates specification.
pub struct Updates {
    /// Number of updates
    pub(crate) amount: Expr,
    /// Value length
    pub(crate) val_len: Expr,
}

#[derive(serde::Deserialize, JsonSchema, Clone, Debug)]
/// Non-empty point deletes specification.
pub struct PointDeletes {
    /// Number of non-empty point deletes
    pub(crate) amount: Expr,
}

#[derive(serde::Deserialize, JsonSchema, Clone, Debug)]
/// Empty point deletes specification.
pub struct EmptyPointDeletes {
    /// Number of empty point deletes
    pub(crate) amount: Expr,
    /// Key length
    pub(crate) key_len: Expr,
}
#[derive(serde::Deserialize, JsonSchema, Clone, Debug)]
/// Range deletes specification.
pub struct RangeDeletes {
    /// Number of range deletes
    pub(crate) amount: Expr,
    /// Selectivity of range deletes. Based off of the range of valid keys, not the full key space.
    pub(crate) selectivity: Expr,
}

#[derive(serde::Deserialize, JsonSchema, Clone, Debug)]
/// Non-empty point queries specification.
pub struct PointQueries {
    /// Number of point queries
    pub(crate) amount: Expr,
}

#[derive(serde::Deserialize, JsonSchema, Clone, Debug)]
/// Empty point queries specification.
pub struct EmptyPointQueries {
    /// Number of point queries
    pub(crate) amount: Expr,
    /// Key length
    pub(crate) key_len: Expr,
}

#[derive(serde::Deserialize, JsonSchema, Clone, Debug)]
/// Range queries specification.
pub struct RangeQueries {
    /// Number of range queries
    pub(crate) amount: Expr,
    /// Selectivity of range queries. Based off of the range of valid keys, not the full key-space.
    pub(crate) selectivity: Expr,
}

#[derive(serde::Deserialize, JsonSchema, Clone, Debug)]
pub(crate) struct WorkloadSpecGroup {
    pub(crate) inserts: Option<Inserts>,
    pub(crate) updates: Option<Updates>,
    pub(crate) point_deletes: Option<PointDeletes>,
    pub(crate) empty_point_deletes: Option<EmptyPointDeletes>,
    pub(crate) range_deletes: Option<RangeDeletes>,
    pub(crate) point_queries: Option<PointQueries>,
    pub(crate) empty_point_queries: Option<EmptyPointQueries>,
    pub(crate) range_queries: Option<RangeQueries>,
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
pub(crate) enum CharacterSet {
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
pub(crate) struct WorkloadSpecSection {
    /// A list of groups. Groups share valid keys between operations.
    ///
    /// E.g., non-empty point queries will use a key from an insert in this group.
    pub(crate) groups: Vec<WorkloadSpecGroup>,
    /// The domain from which the keys will be created from.
    #[serde(default = "CharacterSet::default")]
    pub(crate) character_set: CharacterSet,
    /// The domain from which the keys will be created from.
    #[serde(default = "KeyDistribution::default")]
    pub(crate) key_distribution: KeyDistribution,
    /// Whether to skip the check that a generated key is in the valid key set for inserts and empty point queries/deletes.
    ///
    /// This is useful when the keyspace is much larger than the number of keys being generated, as it can greatly decrease generation time.
    #[serde(default)]
    pub(crate) skip_key_contains_check: bool,
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
    pub(crate) sections: Vec<WorkloadSpecSection>,
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
