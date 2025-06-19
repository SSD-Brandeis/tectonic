#![feature(prelude_import)]
#![feature(extend_one)]
#![feature(btree_cursors)]
#![feature(trusted_random_access)]
#![feature(trait_alias)]
#![allow(clippy::needless_return)]
#![allow(dead_code)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use anyhow::{Context, Result, anyhow, bail};
use rand::distr::Alphanumeric;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::iter::repeat_n;
use std::path::PathBuf;
mod keyset {
    #![allow(clippy::needless_return)]
    #![allow(dead_code)]
    use rand::Rng;
    use bloom::{ASMS, BloomFilter};
    use std::cmp::max;
    use std::collections::{HashMap, HashSet};
    use std::ops::{Bound, Range};
    use tracing::debug;
    pub type Key = Box<[u8]>;
    pub trait KeySet {
        fn new(capacity: usize) -> Self;
        fn len(&self) -> usize;
        fn is_empty(&self) -> bool;
        fn push(&mut self, key: Key);
        fn remove(&mut self, idx: usize) -> Option<Key>;
        fn remove_random(&mut self, rng: &mut impl Rng) -> Key {
            let mut i = 0;
            loop {
                let idx = rng.random_range(0..self.len());
                i += 1;
                match self.remove(idx) {
                    Some(key) => {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event tectonic/src/keyset.rs:36",
                                        "tectonic::keyset",
                                        ::tracing::Level::DEBUG,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "tectonic/src/keyset.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(36u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "tectonic::keyset",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::DEBUG
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::DEBUG
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::tracing::__macro_support::Option::Some(
                                                        &format_args!("Remove key generated after {0} attempts.", i)
                                                            as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        return key;
                    }
                    None => continue,
                }
            }
        }
        fn remove_range(&mut self, idx_range: Range<usize>);
        fn remove_range_random(&mut self, selectivity: f64, rng: &mut impl Rng) {
            let num_keys = self.len();
            let range_len = (selectivity * (num_keys as f64)).floor() as usize;
            let start_range = 0..num_keys - range_len;
            let start_idx = rng.random_range(start_range);
            let end_idx = start_idx + range_len;
            self.remove_range(start_idx..end_idx);
        }
        fn get(&self, idx: usize) -> Option<&Key>;
        fn get_random(&self, rng: &mut impl Rng) -> &Key {
            loop {
                match self.get(rng.random_range(0..self.len())) {
                    Some(key) => return key,
                    None => continue,
                }
            }
        }
        fn contains(&self, key: &Key) -> bool;
        fn sort(&mut self);
    }
    pub struct VecKeySet {
        keys: Vec<Key>,
        sorted: bool,
    }
    impl KeySet for VecKeySet {
        fn new(capacity: usize) -> Self {
            return Self {
                keys: Vec::with_capacity(capacity),
                sorted: true,
            };
        }
        fn len(&self) -> usize {
            return self.keys.len();
        }
        fn is_empty(&self) -> bool {
            return self.keys.is_empty();
        }
        fn push(&mut self, key: Key) {
            if self.sorted && self.keys.last().is_some_and(|last_key| last_key > &key) {
                self.sorted = false;
            }
            self.keys.push(key);
        }
        fn remove(&mut self, idx: usize) -> Option<Key> {
            let len = self.keys.len();
            self.keys.swap(idx, len - 1);
            let key = self.keys.remove(idx);
            return Some(key);
        }
        fn remove_range(&mut self, idx_range: Range<usize>) {
            self.keys.drain(idx_range);
        }
        fn get(&self, idx: usize) -> Option<&Key> {
            return Some(&self.keys[idx]);
        }
        fn contains(&self, key: &Key) -> bool {
            return self.keys.contains(key);
        }
        fn sort(&mut self) {
            if !self.sorted {
                self.keys.sort();
                self.sorted = true;
            }
        }
    }
    pub struct VecOptionKeySet {
        keys: Vec<Option<Key>>,
        sorted: bool,
        none_count: usize,
    }
    /// The threshold for the percentage of None values to trigger a filter operation.
    const VEC_OPTION_KEY_SET_FILTER_THRESHOLD: f64 = 0.1;
    /// FIXME: this needs to implemented with "generation indexing" / "slotmap"
    impl VecOptionKeySet {
        fn maybe_flatten_in_place(&mut self) {
            if (self.none_count as f64 / self.keys.len() as f64)
                < VEC_OPTION_KEY_SET_FILTER_THRESHOLD
            {
                return;
            }
            self.keys.retain(Option::is_some);
            self.none_count = 0;
        }
    }
    impl KeySet for VecOptionKeySet {
        fn new(capacity: usize) -> Self {
            return Self {
                keys: Vec::with_capacity(capacity),
                sorted: true,
                none_count: 0,
            };
        }
        fn len(&self) -> usize {
            return self.keys.len();
        }
        fn is_empty(&self) -> bool {
            return self.keys.is_empty();
        }
        fn push(&mut self, key: Key) {
            if self.sorted
                && self
                    .keys
                    .last()
                    .is_some_and(|last_key| last_key.as_ref() > Some(&key))
            {
                self.sorted = false;
            }
            self.keys.push(Some(key));
        }
        fn remove(&mut self, idx: usize) -> Option<Key> {
            let key = self.keys[idx].take()?;
            self.none_count += 1;
            self.maybe_flatten_in_place();
            return Some(key);
        }
        fn remove_range(&mut self, idx_range: Range<usize>) {
            let taken_count = idx_range
                .filter(|&idx| self.keys[idx].take().is_some())
                .count();
            self.none_count += taken_count;
            self.maybe_flatten_in_place();
        }
        fn get(&self, idx: usize) -> Option<&Key> {
            return self.keys.get(idx).and_then(|k| k.as_ref());
        }
        fn contains(&self, key: &Key) -> bool {
            return self.keys.iter().any(|k| k.as_ref() == Some(key));
        }
        fn sort(&mut self) {
            self.maybe_flatten_in_place();
            if !self.sorted {
                self.keys.sort();
                self.sorted = true;
            }
        }
    }
    pub struct VecHashSetKeySet {
        keys: Vec<Key>,
        key_set: HashSet<Key>,
        sorted: bool,
    }
    impl KeySet for VecHashSetKeySet {
        fn new(capacity: usize) -> Self {
            return Self {
                keys: Vec::with_capacity(capacity),
                key_set: HashSet::with_capacity(capacity),
                sorted: true,
            };
        }
        fn len(&self) -> usize {
            return self.keys.len();
        }
        fn is_empty(&self) -> bool {
            return self.keys.is_empty();
        }
        fn push(&mut self, key: Key) {
            if self.sorted && self.keys.last().is_some_and(|last_key| last_key > &key) {
                self.sorted = false;
            }
            self.keys.push(key.clone());
            self.key_set.insert(key);
        }
        fn remove(&mut self, idx: usize) -> Option<Key> {
            let key = self.keys.remove(idx);
            self.key_set.remove(&key);
            return Some(key);
        }
        fn remove_range(&mut self, idx_range: Range<usize>) {
            for idx in idx_range.clone() {
                self.key_set.remove(&self.keys[idx]);
            }
            self.keys.drain(idx_range);
        }
        fn get(&self, idx: usize) -> Option<&Key> {
            return self.keys.get(idx);
        }
        fn contains(&self, key: &Key) -> bool {
            return self.key_set.contains(key);
        }
        fn sort(&mut self) {
            if !self.sorted {
                self.keys.sort();
                self.sorted = true;
            }
        }
    }
    pub struct VecBloomFilterKeySet {
        keys: Vec<Key>,
        bf: BloomFilter,
        sorted: bool,
    }
    impl KeySet for VecBloomFilterKeySet {
        fn new(capacity: usize) -> Self {
            return Self {
                keys: Vec::with_capacity(capacity),
                bf: BloomFilter::with_rate(0.01, max(1, capacity) as u32),
                sorted: true,
            };
        }
        fn len(&self) -> usize {
            return self.keys.len();
        }
        fn is_empty(&self) -> bool {
            return self.keys.is_empty();
        }
        fn push(&mut self, key: Key) {
            if self.sorted && self.keys.last().is_some_and(|last_key| last_key > &key) {
                self.sorted = false;
            }
            self.bf.insert(&key);
            self.keys.push(key);
        }
        fn remove(&mut self, idx: usize) -> Option<Key> {
            let key = self.keys.remove(idx);
            return Some(key);
        }
        fn remove_range(&mut self, idx_range: Range<usize>) {
            self.keys.drain(idx_range);
        }
        fn get(&self, idx: usize) -> Option<&Key> {
            return self.keys.get(idx);
        }
        fn contains(&self, key: &Key) -> bool {
            return self.bf.contains(key);
        }
        fn sort(&mut self) {
            if !self.sorted {
                self.keys.sort();
                self.sorted = true;
            }
        }
    }
    pub struct VecHashMapIndexKeySet {
        keys: Vec<Key>,
        key_to_index: HashMap<Key, usize>,
        sorted: bool,
    }
    impl KeySet for VecHashMapIndexKeySet {
        fn new(capacity: usize) -> Self {
            return Self {
                keys: Vec::with_capacity(capacity),
                key_to_index: HashMap::with_capacity(capacity),
                sorted: true,
            };
        }
        fn len(&self) -> usize {
            return self.keys.len();
        }
        fn is_empty(&self) -> bool {
            return self.keys.is_empty();
        }
        fn push(&mut self, key: Key) {
            if !self.key_to_index.contains_key(&key) {
                self.key_to_index.insert(key.clone(), self.keys.len());
                self.keys.push(key);
            }
        }
        fn remove(&mut self, idx: usize) -> Option<Key> {
            if !(idx < self.keys.len()) {
                ::core::panicking::panic("assertion failed: idx < self.keys.len()")
            }
            let swap_idx = self.keys.len() - 1;
            self.keys.swap(idx, swap_idx);
            let removed = self.keys.pop().unwrap();
            self.key_to_index.remove(&removed);
            if idx < self.keys.len() {
                let swapped_key = &self.keys[idx];
                self.key_to_index.insert(swapped_key.clone(), idx);
            }
            return Some(removed);
        }
        fn remove_range(&mut self, idx_range: Range<usize>) {
            for idx in idx_range.rev() {
                self.remove(idx);
            }
        }
        fn get(&self, idx: usize) -> Option<&Key> {
            return self.keys.get(idx);
        }
        fn contains(&self, key: &Key) -> bool {
            return self.key_to_index.contains_key(key);
        }
        fn sort(&mut self) {
            self.keys.sort();
            self.key_to_index.clear();
            for (i, key) in self.keys.iter().enumerate() {
                self.key_to_index.insert(key.clone(), i);
            }
        }
    }
    pub struct BTreeSetKeySet {
        keys: std::collections::BTreeSet<Key>,
    }
    impl KeySet for BTreeSetKeySet {
        fn new(capacity: usize) -> Self {
            let mut set = Self {
                keys: std::collections::BTreeSet::new(),
            };
            set.keys.extend_reserve(capacity);
            return set;
        }
        fn len(&self) -> usize {
            return self.keys.len();
        }
        fn is_empty(&self) -> bool {
            return self.keys.is_empty();
        }
        fn push(&mut self, key: Key) {
            self.keys.insert(key);
        }
        fn remove(&mut self, idx: usize) -> Option<Key> {
            let key = self.keys.iter().nth(idx).unwrap().clone();
            self.keys.remove(&key);
            return Some(key);
        }
        fn remove_range(&mut self, idx_range: Range<usize>) {
            let lower = self
                .keys
                .iter()
                .nth(idx_range.start)
                .expect("idx to be in range")
                .clone();
            let mut cursor = self.keys.lower_bound_mut(Bound::Included(&lower));
            let count = idx_range.end - idx_range.start;
            for _ in 0..count {
                cursor.remove_next();
            }
        }
        fn get(&self, idx: usize) -> Option<&Key> {
            return self.keys.iter().nth(idx);
        }
        fn get_random(&self, _rng: &mut impl Rng) -> &Key {
            return self.keys.iter().next().unwrap();
        }
        fn contains(&self, key: &Key) -> bool {
            return self.keys.contains(key);
        }
        fn sort(&mut self) {}
    }
}
pub mod spec {
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
    #[serde(tag = "type", rename_all = "snake_case")]
    enum DistributionConfig {
        Uniform { min: f32, max: f32 },
        Normal { mean: f32, std_dev: f32 },
        Exponential { lambda: f32 },
        Beta { alpha: f32, beta: f32 },
        Zipf { n: usize, s: f32 },
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for DistributionConfig {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 5",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "uniform" => _serde::__private::Ok(__Field::__field0),
                            "normal" => _serde::__private::Ok(__Field::__field1),
                            "exponential" => _serde::__private::Ok(__Field::__field2),
                            "beta" => _serde::__private::Ok(__Field::__field3),
                            "zipf" => _serde::__private::Ok(__Field::__field4),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"uniform" => _serde::__private::Ok(__Field::__field0),
                            b"normal" => _serde::__private::Ok(__Field::__field1),
                            b"exponential" => _serde::__private::Ok(__Field::__field2),
                            b"beta" => _serde::__private::Ok(__Field::__field3),
                            b"zipf" => _serde::__private::Ok(__Field::__field4),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &[
                    "uniform",
                    "normal",
                    "exponential",
                    "beta",
                    "zipf",
                ];
                let (__tag, __content) = _serde::Deserializer::deserialize_any(
                    __deserializer,
                    _serde::__private::de::TaggedContentVisitor::<
                        __Field,
                    >::new("type", "internally tagged enum DistributionConfig"),
                )?;
                let __deserializer = _serde::__private::de::ContentDeserializer::<
                    __D::Error,
                >::new(__content);
                match __tag {
                    __Field::__field0 => {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "min" => _serde::__private::Ok(__Field::__field0),
                                    "max" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"min" => _serde::__private::Ok(__Field::__field0),
                                    b"max" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<DistributionConfig>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = DistributionConfig;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct variant DistributionConfig::Uniform",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    f32,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct variant DistributionConfig::Uniform with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    f32,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct variant DistributionConfig::Uniform with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(DistributionConfig::Uniform {
                                    min: __field0,
                                    max: __field1,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<f32> = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<f32> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("min"),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("max"),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("min")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("max")?
                                    }
                                };
                                _serde::__private::Ok(DistributionConfig::Uniform {
                                    min: __field0,
                                    max: __field1,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["min", "max"];
                        _serde::Deserializer::deserialize_any(
                            __deserializer,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<
                                    DistributionConfig,
                                >,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                    __Field::__field1 => {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "mean" => _serde::__private::Ok(__Field::__field0),
                                    "std_dev" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"mean" => _serde::__private::Ok(__Field::__field0),
                                    b"std_dev" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<DistributionConfig>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = DistributionConfig;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct variant DistributionConfig::Normal",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    f32,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct variant DistributionConfig::Normal with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    f32,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct variant DistributionConfig::Normal with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(DistributionConfig::Normal {
                                    mean: __field0,
                                    std_dev: __field1,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<f32> = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<f32> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("mean"),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "std_dev",
                                                    ),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("mean")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("std_dev")?
                                    }
                                };
                                _serde::__private::Ok(DistributionConfig::Normal {
                                    mean: __field0,
                                    std_dev: __field1,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["mean", "std_dev"];
                        _serde::Deserializer::deserialize_any(
                            __deserializer,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<
                                    DistributionConfig,
                                >,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                    __Field::__field2 => {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "lambda" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"lambda" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<DistributionConfig>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = DistributionConfig;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct variant DistributionConfig::Exponential",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    f32,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct variant DistributionConfig::Exponential with 1 element",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(DistributionConfig::Exponential {
                                    lambda: __field0,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<f32> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("lambda"),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("lambda")?
                                    }
                                };
                                _serde::__private::Ok(DistributionConfig::Exponential {
                                    lambda: __field0,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["lambda"];
                        _serde::Deserializer::deserialize_any(
                            __deserializer,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<
                                    DistributionConfig,
                                >,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                    __Field::__field3 => {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "alpha" => _serde::__private::Ok(__Field::__field0),
                                    "beta" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"alpha" => _serde::__private::Ok(__Field::__field0),
                                    b"beta" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<DistributionConfig>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = DistributionConfig;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct variant DistributionConfig::Beta",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    f32,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct variant DistributionConfig::Beta with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    f32,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct variant DistributionConfig::Beta with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(DistributionConfig::Beta {
                                    alpha: __field0,
                                    beta: __field1,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<f32> = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<f32> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("alpha"),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("beta"),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("alpha")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("beta")?
                                    }
                                };
                                _serde::__private::Ok(DistributionConfig::Beta {
                                    alpha: __field0,
                                    beta: __field1,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["alpha", "beta"];
                        _serde::Deserializer::deserialize_any(
                            __deserializer,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<
                                    DistributionConfig,
                                >,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                    __Field::__field4 => {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "n" => _serde::__private::Ok(__Field::__field0),
                                    "s" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"n" => _serde::__private::Ok(__Field::__field0),
                                    b"s" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<DistributionConfig>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = DistributionConfig;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct variant DistributionConfig::Zipf",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    usize,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct variant DistributionConfig::Zipf with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    f32,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct variant DistributionConfig::Zipf with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(DistributionConfig::Zipf {
                                    n: __field0,
                                    s: __field1,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<usize> = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<f32> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("n"),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<usize>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("s"),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("n")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("s")?
                                    }
                                };
                                _serde::__private::Ok(DistributionConfig::Zipf {
                                    n: __field0,
                                    s: __field1,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["n", "s"];
                        _serde::Deserializer::deserialize_any(
                            __deserializer,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<
                                    DistributionConfig,
                                >,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            }
        }
    };
    const _: () = {
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for DistributionConfig {
            fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                schemars::_private::alloc::borrow::Cow::Borrowed("DistributionConfig")
            }
            fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                schemars::_private::alloc::borrow::Cow::Borrowed(
                    "tectonic::spec::DistributionConfig",
                )
            }
            fn json_schema(
                generator: &mut schemars::SchemaGenerator,
            ) -> schemars::Schema {
                {
                    {
                        let mut map = schemars::_private::serde_json::Map::new();
                        map.insert(
                            "oneOf".into(),
                            schemars::_private::serde_json::Value::Array({
                                let mut enum_values = schemars::_private::alloc::vec::Vec::new();
                                enum_values
                                    .push(
                                        {
                                            let mut schema = ::schemars::Schema::try_from(
                                                    ::serde_json::Value::Object({
                                                        let mut object = ::serde_json::Map::new();
                                                        let _ = object
                                                            .insert(
                                                                ("type").into(),
                                                                ::serde_json::to_value(&"object").unwrap(),
                                                            );
                                                        object
                                                    }),
                                                )
                                                .unwrap();
                                            {
                                                schemars::_private::insert_object_property(
                                                    &mut schema,
                                                    "min",
                                                    if generator.contract().is_serialize() {
                                                        false
                                                    } else {
                                                        false
                                                            || (!false
                                                                && <f32 as schemars::JsonSchema>::_schemars_private_is_option())
                                                    },
                                                    { generator.subschema_for::<f32>() },
                                                );
                                            }
                                            {
                                                schemars::_private::insert_object_property(
                                                    &mut schema,
                                                    "max",
                                                    if generator.contract().is_serialize() {
                                                        false
                                                    } else {
                                                        false
                                                            || (!false
                                                                && <f32 as schemars::JsonSchema>::_schemars_private_is_option())
                                                    },
                                                    { generator.subschema_for::<f32>() },
                                                );
                                            }
                                            schemars::_private::apply_internal_enum_variant_tag(
                                                &mut schema,
                                                "type",
                                                "uniform",
                                                false,
                                            );
                                            schema
                                        }
                                            .to_value(),
                                    );
                                enum_values
                                    .push(
                                        {
                                            let mut schema = ::schemars::Schema::try_from(
                                                    ::serde_json::Value::Object({
                                                        let mut object = ::serde_json::Map::new();
                                                        let _ = object
                                                            .insert(
                                                                ("type").into(),
                                                                ::serde_json::to_value(&"object").unwrap(),
                                                            );
                                                        object
                                                    }),
                                                )
                                                .unwrap();
                                            {
                                                schemars::_private::insert_object_property(
                                                    &mut schema,
                                                    "mean",
                                                    if generator.contract().is_serialize() {
                                                        false
                                                    } else {
                                                        false
                                                            || (!false
                                                                && <f32 as schemars::JsonSchema>::_schemars_private_is_option())
                                                    },
                                                    { generator.subschema_for::<f32>() },
                                                );
                                            }
                                            {
                                                schemars::_private::insert_object_property(
                                                    &mut schema,
                                                    "std_dev",
                                                    if generator.contract().is_serialize() {
                                                        false
                                                    } else {
                                                        false
                                                            || (!false
                                                                && <f32 as schemars::JsonSchema>::_schemars_private_is_option())
                                                    },
                                                    { generator.subschema_for::<f32>() },
                                                );
                                            }
                                            schemars::_private::apply_internal_enum_variant_tag(
                                                &mut schema,
                                                "type",
                                                "normal",
                                                false,
                                            );
                                            schema
                                        }
                                            .to_value(),
                                    );
                                enum_values
                                    .push(
                                        {
                                            let mut schema = ::schemars::Schema::try_from(
                                                    ::serde_json::Value::Object({
                                                        let mut object = ::serde_json::Map::new();
                                                        let _ = object
                                                            .insert(
                                                                ("type").into(),
                                                                ::serde_json::to_value(&"object").unwrap(),
                                                            );
                                                        object
                                                    }),
                                                )
                                                .unwrap();
                                            {
                                                schemars::_private::insert_object_property(
                                                    &mut schema,
                                                    "lambda",
                                                    if generator.contract().is_serialize() {
                                                        false
                                                    } else {
                                                        false
                                                            || (!false
                                                                && <f32 as schemars::JsonSchema>::_schemars_private_is_option())
                                                    },
                                                    { generator.subschema_for::<f32>() },
                                                );
                                            }
                                            schemars::_private::apply_internal_enum_variant_tag(
                                                &mut schema,
                                                "type",
                                                "exponential",
                                                false,
                                            );
                                            schema
                                        }
                                            .to_value(),
                                    );
                                enum_values
                                    .push(
                                        {
                                            let mut schema = ::schemars::Schema::try_from(
                                                    ::serde_json::Value::Object({
                                                        let mut object = ::serde_json::Map::new();
                                                        let _ = object
                                                            .insert(
                                                                ("type").into(),
                                                                ::serde_json::to_value(&"object").unwrap(),
                                                            );
                                                        object
                                                    }),
                                                )
                                                .unwrap();
                                            {
                                                schemars::_private::insert_object_property(
                                                    &mut schema,
                                                    "alpha",
                                                    if generator.contract().is_serialize() {
                                                        false
                                                    } else {
                                                        false
                                                            || (!false
                                                                && <f32 as schemars::JsonSchema>::_schemars_private_is_option())
                                                    },
                                                    { generator.subschema_for::<f32>() },
                                                );
                                            }
                                            {
                                                schemars::_private::insert_object_property(
                                                    &mut schema,
                                                    "beta",
                                                    if generator.contract().is_serialize() {
                                                        false
                                                    } else {
                                                        false
                                                            || (!false
                                                                && <f32 as schemars::JsonSchema>::_schemars_private_is_option())
                                                    },
                                                    { generator.subschema_for::<f32>() },
                                                );
                                            }
                                            schemars::_private::apply_internal_enum_variant_tag(
                                                &mut schema,
                                                "type",
                                                "beta",
                                                false,
                                            );
                                            schema
                                        }
                                            .to_value(),
                                    );
                                enum_values
                                    .push(
                                        {
                                            let mut schema = ::schemars::Schema::try_from(
                                                    ::serde_json::Value::Object({
                                                        let mut object = ::serde_json::Map::new();
                                                        let _ = object
                                                            .insert(
                                                                ("type").into(),
                                                                ::serde_json::to_value(&"object").unwrap(),
                                                            );
                                                        object
                                                    }),
                                                )
                                                .unwrap();
                                            {
                                                schemars::_private::insert_object_property(
                                                    &mut schema,
                                                    "n",
                                                    if generator.contract().is_serialize() {
                                                        false
                                                    } else {
                                                        false
                                                            || (!false
                                                                && <usize as schemars::JsonSchema>::_schemars_private_is_option())
                                                    },
                                                    { generator.subschema_for::<usize>() },
                                                );
                                            }
                                            {
                                                schemars::_private::insert_object_property(
                                                    &mut schema,
                                                    "s",
                                                    if generator.contract().is_serialize() {
                                                        false
                                                    } else {
                                                        false
                                                            || (!false
                                                                && <f32 as schemars::JsonSchema>::_schemars_private_is_option())
                                                    },
                                                    { generator.subschema_for::<f32>() },
                                                );
                                            }
                                            schemars::_private::apply_internal_enum_variant_tag(
                                                &mut schema,
                                                "type",
                                                "zipf",
                                                false,
                                            );
                                            schema
                                        }
                                            .to_value(),
                                    );
                                enum_values
                            }),
                        );
                        schemars::Schema::from(map)
                    }
                }
            }
            fn inline_schema() -> bool {
                false
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for DistributionConfig {
        #[inline]
        fn clone(&self) -> DistributionConfig {
            match self {
                DistributionConfig::Uniform { min: __self_0, max: __self_1 } => {
                    DistributionConfig::Uniform {
                        min: ::core::clone::Clone::clone(__self_0),
                        max: ::core::clone::Clone::clone(__self_1),
                    }
                }
                DistributionConfig::Normal { mean: __self_0, std_dev: __self_1 } => {
                    DistributionConfig::Normal {
                        mean: ::core::clone::Clone::clone(__self_0),
                        std_dev: ::core::clone::Clone::clone(__self_1),
                    }
                }
                DistributionConfig::Exponential { lambda: __self_0 } => {
                    DistributionConfig::Exponential {
                        lambda: ::core::clone::Clone::clone(__self_0),
                    }
                }
                DistributionConfig::Beta { alpha: __self_0, beta: __self_1 } => {
                    DistributionConfig::Beta {
                        alpha: ::core::clone::Clone::clone(__self_0),
                        beta: ::core::clone::Clone::clone(__self_1),
                    }
                }
                DistributionConfig::Zipf { n: __self_0, s: __self_1 } => {
                    DistributionConfig::Zipf {
                        n: ::core::clone::Clone::clone(__self_0),
                        s: ::core::clone::Clone::clone(__self_1),
                    }
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DistributionConfig {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DistributionConfig::Uniform { min: __self_0, max: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Uniform",
                        "min",
                        __self_0,
                        "max",
                        &__self_1,
                    )
                }
                DistributionConfig::Normal { mean: __self_0, std_dev: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Normal",
                        "mean",
                        __self_0,
                        "std_dev",
                        &__self_1,
                    )
                }
                DistributionConfig::Exponential { lambda: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Exponential",
                        "lambda",
                        &__self_0,
                    )
                }
                DistributionConfig::Beta { alpha: __self_0, beta: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Beta",
                        "alpha",
                        __self_0,
                        "beta",
                        &__self_1,
                    )
                }
                DistributionConfig::Zipf { n: __self_0, s: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Zipf",
                        "n",
                        __self_0,
                        "s",
                        &__self_1,
                    )
                }
            }
        }
    }
    #[serde(try_from = "DistributionConfig")]
    /// Different types of distributions that can be used to sample values.
    pub enum Distribution {
        /// Uniform distribution over the range [min, max).
        Uniform { min: f32, max: f32, distr: rand_distr::Uniform<f32> },
        /// Normal distribution with the given mean and standard deviation.
        Normal { mean: f32, std_dev: f32, distr: rand_distr::Normal<f32> },
        /// Exponential distribution with the given lambda parameter.
        Exponential { lambda: f32, distr: rand_distr::Exp<f32> },
        /// Beta distribution with the given alpha and beta parameters.
        Beta { alpha: f32, beta: f32, distr: rand_distr::Beta<f32> },
        /// Zipf distribution with the given n and s parameters.
        Zipf { n: usize, s: f32, distr: rand_distr::Zipf<f32> },
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Distribution {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                _serde::__private::Result::and_then(
                    <DistributionConfig as _serde::Deserialize>::deserialize(
                        __deserializer,
                    ),
                    |v| {
                        _serde::__private::TryFrom::try_from(v)
                            .map_err(_serde::de::Error::custom)
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for Distribution {
        #[inline]
        fn clone(&self) -> Distribution {
            match self {
                Distribution::Uniform {
                    min: __self_0,
                    max: __self_1,
                    distr: __self_2,
                } => {
                    Distribution::Uniform {
                        min: ::core::clone::Clone::clone(__self_0),
                        max: ::core::clone::Clone::clone(__self_1),
                        distr: ::core::clone::Clone::clone(__self_2),
                    }
                }
                Distribution::Normal {
                    mean: __self_0,
                    std_dev: __self_1,
                    distr: __self_2,
                } => {
                    Distribution::Normal {
                        mean: ::core::clone::Clone::clone(__self_0),
                        std_dev: ::core::clone::Clone::clone(__self_1),
                        distr: ::core::clone::Clone::clone(__self_2),
                    }
                }
                Distribution::Exponential { lambda: __self_0, distr: __self_1 } => {
                    Distribution::Exponential {
                        lambda: ::core::clone::Clone::clone(__self_0),
                        distr: ::core::clone::Clone::clone(__self_1),
                    }
                }
                Distribution::Beta {
                    alpha: __self_0,
                    beta: __self_1,
                    distr: __self_2,
                } => {
                    Distribution::Beta {
                        alpha: ::core::clone::Clone::clone(__self_0),
                        beta: ::core::clone::Clone::clone(__self_1),
                        distr: ::core::clone::Clone::clone(__self_2),
                    }
                }
                Distribution::Zipf { n: __self_0, s: __self_1, distr: __self_2 } => {
                    Distribution::Zipf {
                        n: ::core::clone::Clone::clone(__self_0),
                        s: ::core::clone::Clone::clone(__self_1),
                        distr: ::core::clone::Clone::clone(__self_2),
                    }
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Distribution {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Distribution::Uniform {
                    min: __self_0,
                    max: __self_1,
                    distr: __self_2,
                } => {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "Uniform",
                        "min",
                        __self_0,
                        "max",
                        __self_1,
                        "distr",
                        &__self_2,
                    )
                }
                Distribution::Normal {
                    mean: __self_0,
                    std_dev: __self_1,
                    distr: __self_2,
                } => {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "Normal",
                        "mean",
                        __self_0,
                        "std_dev",
                        __self_1,
                        "distr",
                        &__self_2,
                    )
                }
                Distribution::Exponential { lambda: __self_0, distr: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Exponential",
                        "lambda",
                        __self_0,
                        "distr",
                        &__self_1,
                    )
                }
                Distribution::Beta {
                    alpha: __self_0,
                    beta: __self_1,
                    distr: __self_2,
                } => {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "Beta",
                        "alpha",
                        __self_0,
                        "beta",
                        __self_1,
                        "distr",
                        &__self_2,
                    )
                }
                Distribution::Zipf { n: __self_0, s: __self_1, distr: __self_2 } => {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "Zipf",
                        "n",
                        __self_0,
                        "s",
                        __self_1,
                        "distr",
                        &__self_2,
                    )
                }
            }
        }
    }
    impl TryFrom<DistributionConfig> for Distribution {
        type Error = anyhow::Error;
        fn try_from(value: DistributionConfig) -> Result<Self, Self::Error> {
            use DistributionConfig as DC;
            let distr = match value {
                DC::Uniform { min, max } => {
                    Self::Uniform {
                        min,
                        max,
                        distr: rand_distr::Uniform::new(min, max)?,
                    }
                }
                DC::Normal { mean, std_dev } => {
                    Self::Normal {
                        mean,
                        std_dev,
                        distr: rand_distr::Normal::new(mean, std_dev)?,
                    }
                }
                DC::Exponential { lambda } => {
                    Self::Exponential {
                        lambda,
                        distr: rand_distr::Exp::new(lambda)?,
                    }
                }
                DC::Beta { alpha, beta } => {
                    Self::Beta {
                        alpha,
                        beta,
                        distr: rand_distr::Beta::new(alpha, beta)?,
                    }
                }
                DC::Zipf { n, s } => {
                    Self::Zipf {
                        n,
                        s,
                        distr: rand_distr::Zipf::new(n as f32, s)?,
                    }
                }
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
    #[serde(untagged)]
    pub enum Expr {
        Constant(f32),
        Sampled(Distribution),
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Expr {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                let __content = <_serde::__private::de::Content as _serde::Deserialize>::deserialize(
                    __deserializer,
                )?;
                let __deserializer = _serde::__private::de::ContentRefDeserializer::<
                    __D::Error,
                >::new(&__content);
                if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
                    <f32 as _serde::Deserialize>::deserialize(__deserializer),
                    Expr::Constant,
                ) {
                    return _serde::__private::Ok(__ok);
                }
                if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
                    <Distribution as _serde::Deserialize>::deserialize(__deserializer),
                    Expr::Sampled,
                ) {
                    return _serde::__private::Ok(__ok);
                }
                _serde::__private::Err(
                    _serde::de::Error::custom(
                        "data did not match any variant of untagged enum Expr",
                    ),
                )
            }
        }
    };
    const _: () = {
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for Expr {
            fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                schemars::_private::alloc::borrow::Cow::Borrowed("Expr")
            }
            fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                schemars::_private::alloc::borrow::Cow::Borrowed("tectonic::spec::Expr")
            }
            fn json_schema(
                generator: &mut schemars::SchemaGenerator,
            ) -> schemars::Schema {
                {
                    {
                        let mut map = schemars::_private::serde_json::Map::new();
                        map.insert(
                            "anyOf".into(),
                            schemars::_private::serde_json::Value::Array({
                                let mut enum_values = schemars::_private::alloc::vec::Vec::new();
                                enum_values
                                    .push(
                                        {
                                            let mut schema = generator.subschema_for::<f32>();
                                            if generator.settings().untagged_enum_variant_titles {
                                                schema.insert("title".to_owned(), "Constant".into());
                                            }
                                            schema
                                        }
                                            .to_value(),
                                    );
                                enum_values
                                    .push(
                                        {
                                            let mut schema = generator.subschema_for::<Distribution>();
                                            if generator.settings().untagged_enum_variant_titles {
                                                schema.insert("title".to_owned(), "Sampled".into());
                                            }
                                            schema
                                        }
                                            .to_value(),
                                    );
                                enum_values
                            }),
                        );
                        schemars::Schema::from(map)
                    }
                }
            }
            fn inline_schema() -> bool {
                false
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for Expr {
        #[inline]
        fn clone(&self) -> Expr {
            match self {
                Expr::Constant(__self_0) => {
                    Expr::Constant(::core::clone::Clone::clone(__self_0))
                }
                Expr::Sampled(__self_0) => {
                    Expr::Sampled(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Expr {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Expr::Constant(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Constant",
                        &__self_0,
                    )
                }
                Expr::Sampled(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Sampled",
                        &__self_0,
                    )
                }
            }
        }
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
    #[serde(rename_all = "snake_case")]
    /// Different selection strategies for keys in a workload.
    pub enum KeyDistribution {
        #[default]
        Uniform,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for KeyDistribution {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 1",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "uniform" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"uniform" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<KeyDistribution>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = KeyDistribution;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum KeyDistribution",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(KeyDistribution::Uniform)
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &["uniform"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "KeyDistribution",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<KeyDistribution>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    const _: () = {
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for KeyDistribution {
            fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                schemars::_private::alloc::borrow::Cow::Borrowed("KeyDistribution")
            }
            fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                schemars::_private::alloc::borrow::Cow::Borrowed(
                    "tectonic::spec::KeyDistribution",
                )
            }
            fn json_schema(
                generator: &mut schemars::SchemaGenerator,
            ) -> schemars::Schema {
                {
                    let mut schema = {
                        let mut map = schemars::_private::serde_json::Map::new();
                        map.insert("type".into(), "string".into());
                        map.insert(
                            "enum".into(),
                            schemars::_private::serde_json::Value::Array({
                                let mut enum_values = schemars::_private::alloc::vec::Vec::new();
                                enum_values.push(("uniform").into());
                                enum_values
                            }),
                        );
                        schemars::Schema::from(map)
                    };
                    const title_and_description: (&str, &str) = schemars::_private::get_title_and_description(
                        "Different selection strategies for keys in a workload.",
                    );
                    schemars::_private::insert_metadata_property_if_nonempty(
                        &mut schema,
                        "title",
                        title_and_description.0,
                    );
                    schemars::_private::insert_metadata_property_if_nonempty(
                        &mut schema,
                        "description",
                        title_and_description.1,
                    );
                    schema
                }
            }
            fn inline_schema() -> bool {
                false
            }
        }
    };
    #[automatically_derived]
    impl ::core::default::Default for KeyDistribution {
        #[inline]
        fn default() -> KeyDistribution {
            Self::Uniform
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for KeyDistribution {
        #[inline]
        fn clone(&self) -> KeyDistribution {
            KeyDistribution::Uniform
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for KeyDistribution {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "Uniform")
        }
    }
    /// Inserts specification.
    pub struct Inserts {
        /// Number of inserts
        pub(crate) amount: Expr,
        /// Key length
        pub(crate) key_len: Expr,
        /// Value length
        pub(crate) val_len: Expr,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Inserts {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "amount" => _serde::__private::Ok(__Field::__field0),
                            "key_len" => _serde::__private::Ok(__Field::__field1),
                            "val_len" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"amount" => _serde::__private::Ok(__Field::__field0),
                            b"key_len" => _serde::__private::Ok(__Field::__field1),
                            b"val_len" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Inserts>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Inserts;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Inserts",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Expr,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Inserts with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Expr,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Inserts with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            Expr,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Inserts with 3 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Inserts {
                            amount: __field0,
                            key_len: __field1,
                            val_len: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Expr> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Expr> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Expr> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("amount"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Expr>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "key_len",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Expr>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "val_len",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Expr>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("amount")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("key_len")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("val_len")?
                            }
                        };
                        _serde::__private::Ok(Inserts {
                            amount: __field0,
                            key_len: __field1,
                            val_len: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "amount",
                    "key_len",
                    "val_len",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Inserts",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Inserts>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    const _: () = {
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for Inserts {
            fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                schemars::_private::alloc::borrow::Cow::Borrowed("Inserts")
            }
            fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                schemars::_private::alloc::borrow::Cow::Borrowed(
                    "tectonic::spec::Inserts",
                )
            }
            fn json_schema(
                generator: &mut schemars::SchemaGenerator,
            ) -> schemars::Schema {
                {
                    let mut schema = ::schemars::Schema::try_from(
                            ::serde_json::Value::Object({
                                let mut object = ::serde_json::Map::new();
                                let _ = object
                                    .insert(
                                        ("type").into(),
                                        ::serde_json::to_value(&"object").unwrap(),
                                    );
                                object
                            }),
                        )
                        .unwrap();
                    {
                        schemars::_private::insert_object_property(
                            &mut schema,
                            "amount",
                            if generator.contract().is_serialize() {
                                false
                            } else {
                                false
                                    || (!false
                                        && <Expr as schemars::JsonSchema>::_schemars_private_is_option())
                            },
                            {
                                let mut schema = generator.subschema_for::<Expr>();
                                const title_and_description: (&str, &str) = schemars::_private::get_title_and_description(
                                    "Number of inserts",
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "title",
                                    title_and_description.0,
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "description",
                                    title_and_description.1,
                                );
                                schema
                            },
                        );
                    }
                    {
                        schemars::_private::insert_object_property(
                            &mut schema,
                            "key_len",
                            if generator.contract().is_serialize() {
                                false
                            } else {
                                false
                                    || (!false
                                        && <Expr as schemars::JsonSchema>::_schemars_private_is_option())
                            },
                            {
                                let mut schema = generator.subschema_for::<Expr>();
                                const title_and_description: (&str, &str) = schemars::_private::get_title_and_description(
                                    "Key length",
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "title",
                                    title_and_description.0,
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "description",
                                    title_and_description.1,
                                );
                                schema
                            },
                        );
                    }
                    {
                        schemars::_private::insert_object_property(
                            &mut schema,
                            "val_len",
                            if generator.contract().is_serialize() {
                                false
                            } else {
                                false
                                    || (!false
                                        && <Expr as schemars::JsonSchema>::_schemars_private_is_option())
                            },
                            {
                                let mut schema = generator.subschema_for::<Expr>();
                                const title_and_description: (&str, &str) = schemars::_private::get_title_and_description(
                                    "Value length",
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "title",
                                    title_and_description.0,
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "description",
                                    title_and_description.1,
                                );
                                schema
                            },
                        );
                    }
                    const title_and_description: (&str, &str) = schemars::_private::get_title_and_description(
                        "Inserts specification.",
                    );
                    schemars::_private::insert_metadata_property_if_nonempty(
                        &mut schema,
                        "title",
                        title_and_description.0,
                    );
                    schemars::_private::insert_metadata_property_if_nonempty(
                        &mut schema,
                        "description",
                        title_and_description.1,
                    );
                    schema
                }
            }
            fn inline_schema() -> bool {
                false
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for Inserts {
        #[inline]
        fn clone(&self) -> Inserts {
            Inserts {
                amount: ::core::clone::Clone::clone(&self.amount),
                key_len: ::core::clone::Clone::clone(&self.key_len),
                val_len: ::core::clone::Clone::clone(&self.val_len),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Inserts {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Inserts",
                "amount",
                &self.amount,
                "key_len",
                &self.key_len,
                "val_len",
                &&self.val_len,
            )
        }
    }
    /// Updates specification.
    pub struct Updates {
        /// Number of updates
        pub(crate) amount: Expr,
        /// Value length
        pub(crate) val_len: Expr,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Updates {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "amount" => _serde::__private::Ok(__Field::__field0),
                            "val_len" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"amount" => _serde::__private::Ok(__Field::__field0),
                            b"val_len" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Updates>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Updates;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Updates",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Expr,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Updates with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Expr,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Updates with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Updates {
                            amount: __field0,
                            val_len: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Expr> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Expr> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("amount"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Expr>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "val_len",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Expr>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("amount")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("val_len")?
                            }
                        };
                        _serde::__private::Ok(Updates {
                            amount: __field0,
                            val_len: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["amount", "val_len"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Updates",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Updates>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    const _: () = {
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for Updates {
            fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                schemars::_private::alloc::borrow::Cow::Borrowed("Updates")
            }
            fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                schemars::_private::alloc::borrow::Cow::Borrowed(
                    "tectonic::spec::Updates",
                )
            }
            fn json_schema(
                generator: &mut schemars::SchemaGenerator,
            ) -> schemars::Schema {
                {
                    let mut schema = ::schemars::Schema::try_from(
                            ::serde_json::Value::Object({
                                let mut object = ::serde_json::Map::new();
                                let _ = object
                                    .insert(
                                        ("type").into(),
                                        ::serde_json::to_value(&"object").unwrap(),
                                    );
                                object
                            }),
                        )
                        .unwrap();
                    {
                        schemars::_private::insert_object_property(
                            &mut schema,
                            "amount",
                            if generator.contract().is_serialize() {
                                false
                            } else {
                                false
                                    || (!false
                                        && <Expr as schemars::JsonSchema>::_schemars_private_is_option())
                            },
                            {
                                let mut schema = generator.subschema_for::<Expr>();
                                const title_and_description: (&str, &str) = schemars::_private::get_title_and_description(
                                    "Number of updates",
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "title",
                                    title_and_description.0,
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "description",
                                    title_and_description.1,
                                );
                                schema
                            },
                        );
                    }
                    {
                        schemars::_private::insert_object_property(
                            &mut schema,
                            "val_len",
                            if generator.contract().is_serialize() {
                                false
                            } else {
                                false
                                    || (!false
                                        && <Expr as schemars::JsonSchema>::_schemars_private_is_option())
                            },
                            {
                                let mut schema = generator.subschema_for::<Expr>();
                                const title_and_description: (&str, &str) = schemars::_private::get_title_and_description(
                                    "Value length",
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "title",
                                    title_and_description.0,
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "description",
                                    title_and_description.1,
                                );
                                schema
                            },
                        );
                    }
                    const title_and_description: (&str, &str) = schemars::_private::get_title_and_description(
                        "Updates specification.",
                    );
                    schemars::_private::insert_metadata_property_if_nonempty(
                        &mut schema,
                        "title",
                        title_and_description.0,
                    );
                    schemars::_private::insert_metadata_property_if_nonempty(
                        &mut schema,
                        "description",
                        title_and_description.1,
                    );
                    schema
                }
            }
            fn inline_schema() -> bool {
                false
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for Updates {
        #[inline]
        fn clone(&self) -> Updates {
            Updates {
                amount: ::core::clone::Clone::clone(&self.amount),
                val_len: ::core::clone::Clone::clone(&self.val_len),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Updates {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Updates",
                "amount",
                &self.amount,
                "val_len",
                &&self.val_len,
            )
        }
    }
    /// Non-empty point deletes specification.
    pub struct PointDeletes {
        /// Number of non-empty point deletes
        pub(crate) amount: Expr,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for PointDeletes {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "amount" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"amount" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<PointDeletes>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = PointDeletes;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct PointDeletes",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Expr,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct PointDeletes with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(PointDeletes { amount: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Expr> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("amount"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Expr>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("amount")?
                            }
                        };
                        _serde::__private::Ok(PointDeletes { amount: __field0 })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["amount"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "PointDeletes",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<PointDeletes>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    const _: () = {
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for PointDeletes {
            fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                schemars::_private::alloc::borrow::Cow::Borrowed("PointDeletes")
            }
            fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                schemars::_private::alloc::borrow::Cow::Borrowed(
                    "tectonic::spec::PointDeletes",
                )
            }
            fn json_schema(
                generator: &mut schemars::SchemaGenerator,
            ) -> schemars::Schema {
                {
                    let mut schema = ::schemars::Schema::try_from(
                            ::serde_json::Value::Object({
                                let mut object = ::serde_json::Map::new();
                                let _ = object
                                    .insert(
                                        ("type").into(),
                                        ::serde_json::to_value(&"object").unwrap(),
                                    );
                                object
                            }),
                        )
                        .unwrap();
                    {
                        schemars::_private::insert_object_property(
                            &mut schema,
                            "amount",
                            if generator.contract().is_serialize() {
                                false
                            } else {
                                false
                                    || (!false
                                        && <Expr as schemars::JsonSchema>::_schemars_private_is_option())
                            },
                            {
                                let mut schema = generator.subschema_for::<Expr>();
                                const title_and_description: (&str, &str) = schemars::_private::get_title_and_description(
                                    "Number of non-empty point deletes",
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "title",
                                    title_and_description.0,
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "description",
                                    title_and_description.1,
                                );
                                schema
                            },
                        );
                    }
                    const title_and_description: (&str, &str) = schemars::_private::get_title_and_description(
                        "Non-empty point deletes specification.",
                    );
                    schemars::_private::insert_metadata_property_if_nonempty(
                        &mut schema,
                        "title",
                        title_and_description.0,
                    );
                    schemars::_private::insert_metadata_property_if_nonempty(
                        &mut schema,
                        "description",
                        title_and_description.1,
                    );
                    schema
                }
            }
            fn inline_schema() -> bool {
                false
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for PointDeletes {
        #[inline]
        fn clone(&self) -> PointDeletes {
            PointDeletes {
                amount: ::core::clone::Clone::clone(&self.amount),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PointDeletes {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "PointDeletes",
                "amount",
                &&self.amount,
            )
        }
    }
    /// Empty point deletes specification.
    pub struct EmptyPointDeletes {
        /// Number of empty point deletes
        pub(crate) amount: Expr,
        /// Key length
        pub(crate) key_len: Expr,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for EmptyPointDeletes {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "amount" => _serde::__private::Ok(__Field::__field0),
                            "key_len" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"amount" => _serde::__private::Ok(__Field::__field0),
                            b"key_len" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<EmptyPointDeletes>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = EmptyPointDeletes;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct EmptyPointDeletes",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Expr,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct EmptyPointDeletes with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Expr,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct EmptyPointDeletes with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(EmptyPointDeletes {
                            amount: __field0,
                            key_len: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Expr> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Expr> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("amount"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Expr>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "key_len",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Expr>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("amount")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("key_len")?
                            }
                        };
                        _serde::__private::Ok(EmptyPointDeletes {
                            amount: __field0,
                            key_len: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["amount", "key_len"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "EmptyPointDeletes",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<EmptyPointDeletes>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    const _: () = {
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for EmptyPointDeletes {
            fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                schemars::_private::alloc::borrow::Cow::Borrowed("EmptyPointDeletes")
            }
            fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                schemars::_private::alloc::borrow::Cow::Borrowed(
                    "tectonic::spec::EmptyPointDeletes",
                )
            }
            fn json_schema(
                generator: &mut schemars::SchemaGenerator,
            ) -> schemars::Schema {
                {
                    let mut schema = ::schemars::Schema::try_from(
                            ::serde_json::Value::Object({
                                let mut object = ::serde_json::Map::new();
                                let _ = object
                                    .insert(
                                        ("type").into(),
                                        ::serde_json::to_value(&"object").unwrap(),
                                    );
                                object
                            }),
                        )
                        .unwrap();
                    {
                        schemars::_private::insert_object_property(
                            &mut schema,
                            "amount",
                            if generator.contract().is_serialize() {
                                false
                            } else {
                                false
                                    || (!false
                                        && <Expr as schemars::JsonSchema>::_schemars_private_is_option())
                            },
                            {
                                let mut schema = generator.subschema_for::<Expr>();
                                const title_and_description: (&str, &str) = schemars::_private::get_title_and_description(
                                    "Number of empty point deletes",
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "title",
                                    title_and_description.0,
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "description",
                                    title_and_description.1,
                                );
                                schema
                            },
                        );
                    }
                    {
                        schemars::_private::insert_object_property(
                            &mut schema,
                            "key_len",
                            if generator.contract().is_serialize() {
                                false
                            } else {
                                false
                                    || (!false
                                        && <Expr as schemars::JsonSchema>::_schemars_private_is_option())
                            },
                            {
                                let mut schema = generator.subschema_for::<Expr>();
                                const title_and_description: (&str, &str) = schemars::_private::get_title_and_description(
                                    "Key length",
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "title",
                                    title_and_description.0,
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "description",
                                    title_and_description.1,
                                );
                                schema
                            },
                        );
                    }
                    const title_and_description: (&str, &str) = schemars::_private::get_title_and_description(
                        "Empty point deletes specification.",
                    );
                    schemars::_private::insert_metadata_property_if_nonempty(
                        &mut schema,
                        "title",
                        title_and_description.0,
                    );
                    schemars::_private::insert_metadata_property_if_nonempty(
                        &mut schema,
                        "description",
                        title_and_description.1,
                    );
                    schema
                }
            }
            fn inline_schema() -> bool {
                false
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for EmptyPointDeletes {
        #[inline]
        fn clone(&self) -> EmptyPointDeletes {
            EmptyPointDeletes {
                amount: ::core::clone::Clone::clone(&self.amount),
                key_len: ::core::clone::Clone::clone(&self.key_len),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for EmptyPointDeletes {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "EmptyPointDeletes",
                "amount",
                &self.amount,
                "key_len",
                &&self.key_len,
            )
        }
    }
    /// Range deletes specification.
    pub struct RangeDeletes {
        /// Number of range deletes
        pub(crate) amount: Expr,
        /// Selectivity of range deletes. Based off of the range of valid keys, not the full key space.
        pub(crate) selectivity: Expr,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for RangeDeletes {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "amount" => _serde::__private::Ok(__Field::__field0),
                            "selectivity" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"amount" => _serde::__private::Ok(__Field::__field0),
                            b"selectivity" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<RangeDeletes>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = RangeDeletes;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct RangeDeletes",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Expr,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct RangeDeletes with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Expr,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct RangeDeletes with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(RangeDeletes {
                            amount: __field0,
                            selectivity: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Expr> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Expr> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("amount"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Expr>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "selectivity",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Expr>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("amount")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("selectivity")?
                            }
                        };
                        _serde::__private::Ok(RangeDeletes {
                            amount: __field0,
                            selectivity: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["amount", "selectivity"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "RangeDeletes",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<RangeDeletes>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    const _: () = {
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for RangeDeletes {
            fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                schemars::_private::alloc::borrow::Cow::Borrowed("RangeDeletes")
            }
            fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                schemars::_private::alloc::borrow::Cow::Borrowed(
                    "tectonic::spec::RangeDeletes",
                )
            }
            fn json_schema(
                generator: &mut schemars::SchemaGenerator,
            ) -> schemars::Schema {
                {
                    let mut schema = ::schemars::Schema::try_from(
                            ::serde_json::Value::Object({
                                let mut object = ::serde_json::Map::new();
                                let _ = object
                                    .insert(
                                        ("type").into(),
                                        ::serde_json::to_value(&"object").unwrap(),
                                    );
                                object
                            }),
                        )
                        .unwrap();
                    {
                        schemars::_private::insert_object_property(
                            &mut schema,
                            "amount",
                            if generator.contract().is_serialize() {
                                false
                            } else {
                                false
                                    || (!false
                                        && <Expr as schemars::JsonSchema>::_schemars_private_is_option())
                            },
                            {
                                let mut schema = generator.subschema_for::<Expr>();
                                const title_and_description: (&str, &str) = schemars::_private::get_title_and_description(
                                    "Number of range deletes",
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "title",
                                    title_and_description.0,
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "description",
                                    title_and_description.1,
                                );
                                schema
                            },
                        );
                    }
                    {
                        schemars::_private::insert_object_property(
                            &mut schema,
                            "selectivity",
                            if generator.contract().is_serialize() {
                                false
                            } else {
                                false
                                    || (!false
                                        && <Expr as schemars::JsonSchema>::_schemars_private_is_option())
                            },
                            {
                                let mut schema = generator.subschema_for::<Expr>();
                                const title_and_description: (&str, &str) = schemars::_private::get_title_and_description(
                                    "Selectivity of range deletes. Based off of the range of valid keys, not the full key space.",
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "title",
                                    title_and_description.0,
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "description",
                                    title_and_description.1,
                                );
                                schema
                            },
                        );
                    }
                    const title_and_description: (&str, &str) = schemars::_private::get_title_and_description(
                        "Range deletes specification.",
                    );
                    schemars::_private::insert_metadata_property_if_nonempty(
                        &mut schema,
                        "title",
                        title_and_description.0,
                    );
                    schemars::_private::insert_metadata_property_if_nonempty(
                        &mut schema,
                        "description",
                        title_and_description.1,
                    );
                    schema
                }
            }
            fn inline_schema() -> bool {
                false
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for RangeDeletes {
        #[inline]
        fn clone(&self) -> RangeDeletes {
            RangeDeletes {
                amount: ::core::clone::Clone::clone(&self.amount),
                selectivity: ::core::clone::Clone::clone(&self.selectivity),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for RangeDeletes {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "RangeDeletes",
                "amount",
                &self.amount,
                "selectivity",
                &&self.selectivity,
            )
        }
    }
    /// Non-empty point queries specification.
    pub struct PointQueries {
        /// Number of point queries
        pub(crate) amount: Expr,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for PointQueries {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "amount" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"amount" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<PointQueries>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = PointQueries;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct PointQueries",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Expr,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct PointQueries with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(PointQueries { amount: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Expr> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("amount"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Expr>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("amount")?
                            }
                        };
                        _serde::__private::Ok(PointQueries { amount: __field0 })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["amount"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "PointQueries",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<PointQueries>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    const _: () = {
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for PointQueries {
            fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                schemars::_private::alloc::borrow::Cow::Borrowed("PointQueries")
            }
            fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                schemars::_private::alloc::borrow::Cow::Borrowed(
                    "tectonic::spec::PointQueries",
                )
            }
            fn json_schema(
                generator: &mut schemars::SchemaGenerator,
            ) -> schemars::Schema {
                {
                    let mut schema = ::schemars::Schema::try_from(
                            ::serde_json::Value::Object({
                                let mut object = ::serde_json::Map::new();
                                let _ = object
                                    .insert(
                                        ("type").into(),
                                        ::serde_json::to_value(&"object").unwrap(),
                                    );
                                object
                            }),
                        )
                        .unwrap();
                    {
                        schemars::_private::insert_object_property(
                            &mut schema,
                            "amount",
                            if generator.contract().is_serialize() {
                                false
                            } else {
                                false
                                    || (!false
                                        && <Expr as schemars::JsonSchema>::_schemars_private_is_option())
                            },
                            {
                                let mut schema = generator.subschema_for::<Expr>();
                                const title_and_description: (&str, &str) = schemars::_private::get_title_and_description(
                                    "Number of point queries",
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "title",
                                    title_and_description.0,
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "description",
                                    title_and_description.1,
                                );
                                schema
                            },
                        );
                    }
                    const title_and_description: (&str, &str) = schemars::_private::get_title_and_description(
                        "Non-empty point queries specification.",
                    );
                    schemars::_private::insert_metadata_property_if_nonempty(
                        &mut schema,
                        "title",
                        title_and_description.0,
                    );
                    schemars::_private::insert_metadata_property_if_nonempty(
                        &mut schema,
                        "description",
                        title_and_description.1,
                    );
                    schema
                }
            }
            fn inline_schema() -> bool {
                false
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for PointQueries {
        #[inline]
        fn clone(&self) -> PointQueries {
            PointQueries {
                amount: ::core::clone::Clone::clone(&self.amount),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PointQueries {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "PointQueries",
                "amount",
                &&self.amount,
            )
        }
    }
    /// Empty point queries specification.
    pub struct EmptyPointQueries {
        /// Number of point queries
        pub(crate) amount: Expr,
        /// Key length
        pub(crate) key_len: Expr,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for EmptyPointQueries {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "amount" => _serde::__private::Ok(__Field::__field0),
                            "key_len" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"amount" => _serde::__private::Ok(__Field::__field0),
                            b"key_len" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<EmptyPointQueries>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = EmptyPointQueries;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct EmptyPointQueries",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Expr,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct EmptyPointQueries with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Expr,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct EmptyPointQueries with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(EmptyPointQueries {
                            amount: __field0,
                            key_len: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Expr> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Expr> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("amount"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Expr>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "key_len",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Expr>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("amount")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("key_len")?
                            }
                        };
                        _serde::__private::Ok(EmptyPointQueries {
                            amount: __field0,
                            key_len: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["amount", "key_len"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "EmptyPointQueries",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<EmptyPointQueries>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    const _: () = {
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for EmptyPointQueries {
            fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                schemars::_private::alloc::borrow::Cow::Borrowed("EmptyPointQueries")
            }
            fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                schemars::_private::alloc::borrow::Cow::Borrowed(
                    "tectonic::spec::EmptyPointQueries",
                )
            }
            fn json_schema(
                generator: &mut schemars::SchemaGenerator,
            ) -> schemars::Schema {
                {
                    let mut schema = ::schemars::Schema::try_from(
                            ::serde_json::Value::Object({
                                let mut object = ::serde_json::Map::new();
                                let _ = object
                                    .insert(
                                        ("type").into(),
                                        ::serde_json::to_value(&"object").unwrap(),
                                    );
                                object
                            }),
                        )
                        .unwrap();
                    {
                        schemars::_private::insert_object_property(
                            &mut schema,
                            "amount",
                            if generator.contract().is_serialize() {
                                false
                            } else {
                                false
                                    || (!false
                                        && <Expr as schemars::JsonSchema>::_schemars_private_is_option())
                            },
                            {
                                let mut schema = generator.subschema_for::<Expr>();
                                const title_and_description: (&str, &str) = schemars::_private::get_title_and_description(
                                    "Number of point queries",
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "title",
                                    title_and_description.0,
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "description",
                                    title_and_description.1,
                                );
                                schema
                            },
                        );
                    }
                    {
                        schemars::_private::insert_object_property(
                            &mut schema,
                            "key_len",
                            if generator.contract().is_serialize() {
                                false
                            } else {
                                false
                                    || (!false
                                        && <Expr as schemars::JsonSchema>::_schemars_private_is_option())
                            },
                            {
                                let mut schema = generator.subschema_for::<Expr>();
                                const title_and_description: (&str, &str) = schemars::_private::get_title_and_description(
                                    "Key length",
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "title",
                                    title_and_description.0,
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "description",
                                    title_and_description.1,
                                );
                                schema
                            },
                        );
                    }
                    const title_and_description: (&str, &str) = schemars::_private::get_title_and_description(
                        "Empty point queries specification.",
                    );
                    schemars::_private::insert_metadata_property_if_nonempty(
                        &mut schema,
                        "title",
                        title_and_description.0,
                    );
                    schemars::_private::insert_metadata_property_if_nonempty(
                        &mut schema,
                        "description",
                        title_and_description.1,
                    );
                    schema
                }
            }
            fn inline_schema() -> bool {
                false
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for EmptyPointQueries {
        #[inline]
        fn clone(&self) -> EmptyPointQueries {
            EmptyPointQueries {
                amount: ::core::clone::Clone::clone(&self.amount),
                key_len: ::core::clone::Clone::clone(&self.key_len),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for EmptyPointQueries {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "EmptyPointQueries",
                "amount",
                &self.amount,
                "key_len",
                &&self.key_len,
            )
        }
    }
    /// Range queries specification.
    pub struct RangeQueries {
        /// Number of range queries
        pub(crate) amount: Expr,
        /// Selectivity of range queries. Based off of the range of valid keys, not the full key-space.
        pub(crate) selectivity: Expr,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for RangeQueries {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "amount" => _serde::__private::Ok(__Field::__field0),
                            "selectivity" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"amount" => _serde::__private::Ok(__Field::__field0),
                            b"selectivity" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<RangeQueries>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = RangeQueries;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct RangeQueries",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Expr,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct RangeQueries with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Expr,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct RangeQueries with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(RangeQueries {
                            amount: __field0,
                            selectivity: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Expr> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Expr> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("amount"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Expr>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "selectivity",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Expr>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("amount")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("selectivity")?
                            }
                        };
                        _serde::__private::Ok(RangeQueries {
                            amount: __field0,
                            selectivity: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["amount", "selectivity"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "RangeQueries",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<RangeQueries>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    const _: () = {
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for RangeQueries {
            fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                schemars::_private::alloc::borrow::Cow::Borrowed("RangeQueries")
            }
            fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                schemars::_private::alloc::borrow::Cow::Borrowed(
                    "tectonic::spec::RangeQueries",
                )
            }
            fn json_schema(
                generator: &mut schemars::SchemaGenerator,
            ) -> schemars::Schema {
                {
                    let mut schema = ::schemars::Schema::try_from(
                            ::serde_json::Value::Object({
                                let mut object = ::serde_json::Map::new();
                                let _ = object
                                    .insert(
                                        ("type").into(),
                                        ::serde_json::to_value(&"object").unwrap(),
                                    );
                                object
                            }),
                        )
                        .unwrap();
                    {
                        schemars::_private::insert_object_property(
                            &mut schema,
                            "amount",
                            if generator.contract().is_serialize() {
                                false
                            } else {
                                false
                                    || (!false
                                        && <Expr as schemars::JsonSchema>::_schemars_private_is_option())
                            },
                            {
                                let mut schema = generator.subschema_for::<Expr>();
                                const title_and_description: (&str, &str) = schemars::_private::get_title_and_description(
                                    "Number of range queries",
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "title",
                                    title_and_description.0,
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "description",
                                    title_and_description.1,
                                );
                                schema
                            },
                        );
                    }
                    {
                        schemars::_private::insert_object_property(
                            &mut schema,
                            "selectivity",
                            if generator.contract().is_serialize() {
                                false
                            } else {
                                false
                                    || (!false
                                        && <Expr as schemars::JsonSchema>::_schemars_private_is_option())
                            },
                            {
                                let mut schema = generator.subschema_for::<Expr>();
                                const title_and_description: (&str, &str) = schemars::_private::get_title_and_description(
                                    "Selectivity of range queries. Based off of the range of valid keys, not the full key-space.",
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "title",
                                    title_and_description.0,
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "description",
                                    title_and_description.1,
                                );
                                schema
                            },
                        );
                    }
                    const title_and_description: (&str, &str) = schemars::_private::get_title_and_description(
                        "Range queries specification.",
                    );
                    schemars::_private::insert_metadata_property_if_nonempty(
                        &mut schema,
                        "title",
                        title_and_description.0,
                    );
                    schemars::_private::insert_metadata_property_if_nonempty(
                        &mut schema,
                        "description",
                        title_and_description.1,
                    );
                    schema
                }
            }
            fn inline_schema() -> bool {
                false
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for RangeQueries {
        #[inline]
        fn clone(&self) -> RangeQueries {
            RangeQueries {
                amount: ::core::clone::Clone::clone(&self.amount),
                selectivity: ::core::clone::Clone::clone(&self.selectivity),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for RangeQueries {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "RangeQueries",
                "amount",
                &self.amount,
                "selectivity",
                &&self.selectivity,
            )
        }
    }
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
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for WorkloadSpecGroup {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            6u64 => _serde::__private::Ok(__Field::__field6),
                            7u64 => _serde::__private::Ok(__Field::__field7),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "inserts" => _serde::__private::Ok(__Field::__field0),
                            "updates" => _serde::__private::Ok(__Field::__field1),
                            "point_deletes" => _serde::__private::Ok(__Field::__field2),
                            "empty_point_deletes" => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            "range_deletes" => _serde::__private::Ok(__Field::__field4),
                            "point_queries" => _serde::__private::Ok(__Field::__field5),
                            "empty_point_queries" => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            "range_queries" => _serde::__private::Ok(__Field::__field7),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"inserts" => _serde::__private::Ok(__Field::__field0),
                            b"updates" => _serde::__private::Ok(__Field::__field1),
                            b"point_deletes" => _serde::__private::Ok(__Field::__field2),
                            b"empty_point_deletes" => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            b"range_deletes" => _serde::__private::Ok(__Field::__field4),
                            b"point_queries" => _serde::__private::Ok(__Field::__field5),
                            b"empty_point_queries" => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            b"range_queries" => _serde::__private::Ok(__Field::__field7),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<WorkloadSpecGroup>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = WorkloadSpecGroup;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct WorkloadSpecGroup",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Option<Inserts>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct WorkloadSpecGroup with 8 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Option<Updates>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct WorkloadSpecGroup with 8 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            Option<PointDeletes>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct WorkloadSpecGroup with 8 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            Option<EmptyPointDeletes>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct WorkloadSpecGroup with 8 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            Option<RangeDeletes>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct WorkloadSpecGroup with 8 elements",
                                    ),
                                );
                            }
                        };
                        let __field5 = match _serde::de::SeqAccess::next_element::<
                            Option<PointQueries>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        5usize,
                                        &"struct WorkloadSpecGroup with 8 elements",
                                    ),
                                );
                            }
                        };
                        let __field6 = match _serde::de::SeqAccess::next_element::<
                            Option<EmptyPointQueries>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        6usize,
                                        &"struct WorkloadSpecGroup with 8 elements",
                                    ),
                                );
                            }
                        };
                        let __field7 = match _serde::de::SeqAccess::next_element::<
                            Option<RangeQueries>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        7usize,
                                        &"struct WorkloadSpecGroup with 8 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(WorkloadSpecGroup {
                            inserts: __field0,
                            updates: __field1,
                            point_deletes: __field2,
                            empty_point_deletes: __field3,
                            range_deletes: __field4,
                            point_queries: __field5,
                            empty_point_queries: __field6,
                            range_queries: __field7,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Option<Inserts>> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Option<Updates>> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<
                            Option<PointDeletes>,
                        > = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<
                            Option<EmptyPointDeletes>,
                        > = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<
                            Option<RangeDeletes>,
                        > = _serde::__private::None;
                        let mut __field5: _serde::__private::Option<
                            Option<PointQueries>,
                        > = _serde::__private::None;
                        let mut __field6: _serde::__private::Option<
                            Option<EmptyPointQueries>,
                        > = _serde::__private::None;
                        let mut __field7: _serde::__private::Option<
                            Option<RangeQueries>,
                        > = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "inserts",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<Inserts>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "updates",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<Updates>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "point_deletes",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<PointDeletes>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "empty_point_deletes",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<EmptyPointDeletes>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "range_deletes",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<RangeDeletes>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "point_queries",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<PointQueries>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::__private::Option::is_some(&__field6) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "empty_point_queries",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<EmptyPointQueries>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field7 => {
                                    if _serde::__private::Option::is_some(&__field7) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "range_queries",
                                            ),
                                        );
                                    }
                                    __field7 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<RangeQueries>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("inserts")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("updates")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("point_deletes")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("empty_point_deletes")?
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("range_deletes")?
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("point_queries")?
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::__private::Some(__field6) => __field6,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("empty_point_queries")?
                            }
                        };
                        let __field7 = match __field7 {
                            _serde::__private::Some(__field7) => __field7,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("range_queries")?
                            }
                        };
                        _serde::__private::Ok(WorkloadSpecGroup {
                            inserts: __field0,
                            updates: __field1,
                            point_deletes: __field2,
                            empty_point_deletes: __field3,
                            range_deletes: __field4,
                            point_queries: __field5,
                            empty_point_queries: __field6,
                            range_queries: __field7,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "inserts",
                    "updates",
                    "point_deletes",
                    "empty_point_deletes",
                    "range_deletes",
                    "point_queries",
                    "empty_point_queries",
                    "range_queries",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "WorkloadSpecGroup",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<WorkloadSpecGroup>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    const _: () = {
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for WorkloadSpecGroup {
            fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                schemars::_private::alloc::borrow::Cow::Borrowed("WorkloadSpecGroup")
            }
            fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                schemars::_private::alloc::borrow::Cow::Borrowed(
                    "tectonic::spec::WorkloadSpecGroup",
                )
            }
            fn json_schema(
                generator: &mut schemars::SchemaGenerator,
            ) -> schemars::Schema {
                {
                    let mut schema = ::schemars::Schema::try_from(
                            ::serde_json::Value::Object({
                                let mut object = ::serde_json::Map::new();
                                let _ = object
                                    .insert(
                                        ("type").into(),
                                        ::serde_json::to_value(&"object").unwrap(),
                                    );
                                object
                            }),
                        )
                        .unwrap();
                    {
                        schemars::_private::insert_object_property(
                            &mut schema,
                            "inserts",
                            if generator.contract().is_serialize() {
                                false
                            } else {
                                false
                                    || (!false
                                        && <Option<
                                            Inserts,
                                        > as schemars::JsonSchema>::_schemars_private_is_option())
                            },
                            { generator.subschema_for::<Option<Inserts>>() },
                        );
                    }
                    {
                        schemars::_private::insert_object_property(
                            &mut schema,
                            "updates",
                            if generator.contract().is_serialize() {
                                false
                            } else {
                                false
                                    || (!false
                                        && <Option<
                                            Updates,
                                        > as schemars::JsonSchema>::_schemars_private_is_option())
                            },
                            { generator.subschema_for::<Option<Updates>>() },
                        );
                    }
                    {
                        schemars::_private::insert_object_property(
                            &mut schema,
                            "point_deletes",
                            if generator.contract().is_serialize() {
                                false
                            } else {
                                false
                                    || (!false
                                        && <Option<
                                            PointDeletes,
                                        > as schemars::JsonSchema>::_schemars_private_is_option())
                            },
                            { generator.subschema_for::<Option<PointDeletes>>() },
                        );
                    }
                    {
                        schemars::_private::insert_object_property(
                            &mut schema,
                            "empty_point_deletes",
                            if generator.contract().is_serialize() {
                                false
                            } else {
                                false
                                    || (!false
                                        && <Option<
                                            EmptyPointDeletes,
                                        > as schemars::JsonSchema>::_schemars_private_is_option())
                            },
                            { generator.subschema_for::<Option<EmptyPointDeletes>>() },
                        );
                    }
                    {
                        schemars::_private::insert_object_property(
                            &mut schema,
                            "range_deletes",
                            if generator.contract().is_serialize() {
                                false
                            } else {
                                false
                                    || (!false
                                        && <Option<
                                            RangeDeletes,
                                        > as schemars::JsonSchema>::_schemars_private_is_option())
                            },
                            { generator.subschema_for::<Option<RangeDeletes>>() },
                        );
                    }
                    {
                        schemars::_private::insert_object_property(
                            &mut schema,
                            "point_queries",
                            if generator.contract().is_serialize() {
                                false
                            } else {
                                false
                                    || (!false
                                        && <Option<
                                            PointQueries,
                                        > as schemars::JsonSchema>::_schemars_private_is_option())
                            },
                            { generator.subschema_for::<Option<PointQueries>>() },
                        );
                    }
                    {
                        schemars::_private::insert_object_property(
                            &mut schema,
                            "empty_point_queries",
                            if generator.contract().is_serialize() {
                                false
                            } else {
                                false
                                    || (!false
                                        && <Option<
                                            EmptyPointQueries,
                                        > as schemars::JsonSchema>::_schemars_private_is_option())
                            },
                            { generator.subschema_for::<Option<EmptyPointQueries>>() },
                        );
                    }
                    {
                        schemars::_private::insert_object_property(
                            &mut schema,
                            "range_queries",
                            if generator.contract().is_serialize() {
                                false
                            } else {
                                false
                                    || (!false
                                        && <Option<
                                            RangeQueries,
                                        > as schemars::JsonSchema>::_schemars_private_is_option())
                            },
                            { generator.subschema_for::<Option<RangeQueries>>() },
                        );
                    }
                    schema
                }
            }
            fn inline_schema() -> bool {
                false
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for WorkloadSpecGroup {
        #[inline]
        fn clone(&self) -> WorkloadSpecGroup {
            WorkloadSpecGroup {
                inserts: ::core::clone::Clone::clone(&self.inserts),
                updates: ::core::clone::Clone::clone(&self.updates),
                point_deletes: ::core::clone::Clone::clone(&self.point_deletes),
                empty_point_deletes: ::core::clone::Clone::clone(
                    &self.empty_point_deletes,
                ),
                range_deletes: ::core::clone::Clone::clone(&self.range_deletes),
                point_queries: ::core::clone::Clone::clone(&self.point_queries),
                empty_point_queries: ::core::clone::Clone::clone(
                    &self.empty_point_queries,
                ),
                range_queries: ::core::clone::Clone::clone(&self.range_queries),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for WorkloadSpecGroup {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "inserts",
                "updates",
                "point_deletes",
                "empty_point_deletes",
                "range_deletes",
                "point_queries",
                "empty_point_queries",
                "range_queries",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.inserts,
                &self.updates,
                &self.point_deletes,
                &self.empty_point_deletes,
                &self.range_deletes,
                &self.point_queries,
                &self.empty_point_queries,
                &&self.range_queries,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "WorkloadSpecGroup",
                names,
                values,
            )
        }
    }
    #[serde(rename_all = "snake_case")]
    pub(crate) enum KeySpace {
        #[default]
        Alphanumeric,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for KeySpace {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 1",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "alphanumeric" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"alphanumeric" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<KeySpace>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = KeySpace;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum KeySpace",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(KeySpace::Alphanumeric)
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &["alphanumeric"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "KeySpace",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<KeySpace>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    const _: () = {
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for KeySpace {
            fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                schemars::_private::alloc::borrow::Cow::Borrowed("KeySpace")
            }
            fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                schemars::_private::alloc::borrow::Cow::Borrowed(
                    "tectonic::spec::KeySpace",
                )
            }
            fn json_schema(
                generator: &mut schemars::SchemaGenerator,
            ) -> schemars::Schema {
                {
                    {
                        let mut map = schemars::_private::serde_json::Map::new();
                        map.insert("type".into(), "string".into());
                        map.insert(
                            "enum".into(),
                            schemars::_private::serde_json::Value::Array({
                                let mut enum_values = schemars::_private::alloc::vec::Vec::new();
                                enum_values.push(("alphanumeric").into());
                                enum_values
                            }),
                        );
                        schemars::Schema::from(map)
                    }
                }
            }
            fn inline_schema() -> bool {
                false
            }
        }
    };
    #[automatically_derived]
    impl ::core::default::Default for KeySpace {
        #[inline]
        fn default() -> KeySpace {
            Self::Alphanumeric
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for KeySpace {
        #[inline]
        fn clone(&self) -> KeySpace {
            KeySpace::Alphanumeric
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for KeySpace {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "Alphanumeric")
        }
    }
    pub(crate) struct WorkloadSpecSection {
        /// A list of groups. Groups share valid keys between operations.
        ///
        /// E.g., non-empty point queries will use a key from an insert in this group.
        pub(crate) groups: Vec<WorkloadSpecGroup>,
        /// The domain from which the keys will be created from.
        #[serde(default = "KeySpace::default")]
        pub(crate) key_space: KeySpace,
        /// The domain from which the keys will be created from.
        #[serde(default = "KeyDistribution::default")]
        pub(crate) key_distribution: KeyDistribution,
        /// Whether to skip the check that a generated key is in the valid key set for inserts and empty point queries/deletes.
        ///
        /// This is useful when the keyspace is much larger than the number of keys being generated, as it can greatly decrease generation time.
        #[serde(default)]
        pub(crate) skip_key_contains_check: bool,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for WorkloadSpecSection {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "groups" => _serde::__private::Ok(__Field::__field0),
                            "key_space" => _serde::__private::Ok(__Field::__field1),
                            "key_distribution" => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            "skip_key_contains_check" => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"groups" => _serde::__private::Ok(__Field::__field0),
                            b"key_space" => _serde::__private::Ok(__Field::__field1),
                            b"key_distribution" => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            b"skip_key_contains_check" => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<WorkloadSpecSection>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = WorkloadSpecSection;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct WorkloadSpecSection",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Vec<WorkloadSpecGroup>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct WorkloadSpecSection with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            KeySpace,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => KeySpace::default(),
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            KeyDistribution,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => KeyDistribution::default(),
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                _serde::__private::Default::default()
                            }
                        };
                        _serde::__private::Ok(WorkloadSpecSection {
                            groups: __field0,
                            key_space: __field1,
                            key_distribution: __field2,
                            skip_key_contains_check: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<
                            Vec<WorkloadSpecGroup>,
                        > = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<KeySpace> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<KeyDistribution> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<bool> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("groups"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Vec<WorkloadSpecGroup>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "key_space",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<KeySpace>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "key_distribution",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            KeyDistribution,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "skip_key_contains_check",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("groups")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => KeySpace::default(),
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => KeyDistribution::default(),
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::Default::default()
                            }
                        };
                        _serde::__private::Ok(WorkloadSpecSection {
                            groups: __field0,
                            key_space: __field1,
                            key_distribution: __field2,
                            skip_key_contains_check: __field3,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "groups",
                    "key_space",
                    "key_distribution",
                    "skip_key_contains_check",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "WorkloadSpecSection",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<WorkloadSpecSection>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    const _: () = {
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for WorkloadSpecSection {
            fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                schemars::_private::alloc::borrow::Cow::Borrowed("WorkloadSpecSection")
            }
            fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                schemars::_private::alloc::borrow::Cow::Borrowed(
                    "tectonic::spec::WorkloadSpecSection",
                )
            }
            fn json_schema(
                generator: &mut schemars::SchemaGenerator,
            ) -> schemars::Schema {
                {
                    let mut schema = ::schemars::Schema::try_from(
                            ::serde_json::Value::Object({
                                let mut object = ::serde_json::Map::new();
                                let _ = object
                                    .insert(
                                        ("type").into(),
                                        ::serde_json::to_value(&"object").unwrap(),
                                    );
                                object
                            }),
                        )
                        .unwrap();
                    {
                        schemars::_private::insert_object_property(
                            &mut schema,
                            "groups",
                            if generator.contract().is_serialize() {
                                false
                            } else {
                                false
                                    || (!false
                                        && <Vec<
                                            WorkloadSpecGroup,
                                        > as schemars::JsonSchema>::_schemars_private_is_option())
                            },
                            {
                                let mut schema = generator
                                    .subschema_for::<Vec<WorkloadSpecGroup>>();
                                const title_and_description: (&str, &str) = schemars::_private::get_title_and_description(
                                    "A list of groups. Groups share valid keys between operations.\n\nE.g., non-empty point queries will use a key from an insert in this group.",
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "title",
                                    title_and_description.0,
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "description",
                                    title_and_description.1,
                                );
                                schema
                            },
                        );
                    }
                    {
                        schemars::_private::insert_object_property(
                            &mut schema,
                            "key_space",
                            if generator.contract().is_serialize() {
                                false
                            } else {
                                true
                                    || (!false
                                        && <KeySpace as schemars::JsonSchema>::_schemars_private_is_option())
                            },
                            {
                                let mut schema = generator.subschema_for::<KeySpace>();
                                const title_and_description: (&str, &str) = schemars::_private::get_title_and_description(
                                    "The domain from which the keys will be created from.",
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "title",
                                    title_and_description.0,
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "description",
                                    title_and_description.1,
                                );
                                Some(KeySpace::default())
                                    .and_then(|d| {
                                        #[allow(unused_imports)]
                                        use ::schemars::_private::{
                                            MaybeSerializeWrapper, NoSerialize as _,
                                        };
                                        MaybeSerializeWrapper(d).maybe_to_value()
                                    })
                                    .map(|d| schema.insert("default".to_owned(), d));
                                schema
                            },
                        );
                    }
                    {
                        schemars::_private::insert_object_property(
                            &mut schema,
                            "key_distribution",
                            if generator.contract().is_serialize() {
                                false
                            } else {
                                true
                                    || (!false
                                        && <KeyDistribution as schemars::JsonSchema>::_schemars_private_is_option())
                            },
                            {
                                let mut schema = generator
                                    .subschema_for::<KeyDistribution>();
                                const title_and_description: (&str, &str) = schemars::_private::get_title_and_description(
                                    "The domain from which the keys will be created from.",
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "title",
                                    title_and_description.0,
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "description",
                                    title_and_description.1,
                                );
                                Some(KeyDistribution::default())
                                    .and_then(|d| {
                                        #[allow(unused_imports)]
                                        use ::schemars::_private::{
                                            MaybeSerializeWrapper, NoSerialize as _,
                                        };
                                        MaybeSerializeWrapper(d).maybe_to_value()
                                    })
                                    .map(|d| schema.insert("default".to_owned(), d));
                                schema
                            },
                        );
                    }
                    {
                        schemars::_private::insert_object_property(
                            &mut schema,
                            "skip_key_contains_check",
                            if generator.contract().is_serialize() {
                                false
                            } else {
                                true
                                    || (!false
                                        && <bool as schemars::JsonSchema>::_schemars_private_is_option())
                            },
                            {
                                let mut schema = generator.subschema_for::<bool>();
                                const title_and_description: (&str, &str) = schemars::_private::get_title_and_description(
                                    "Whether to skip the check that a generated key is in the valid key set for inserts and empty point queries/deletes.\n\nThis is useful when the keyspace is much larger than the number of keys being generated, as it can greatly decrease generation time.",
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "title",
                                    title_and_description.0,
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "description",
                                    title_and_description.1,
                                );
                                Some(<bool>::default())
                                    .and_then(|d| {
                                        #[allow(unused_imports)]
                                        use ::schemars::_private::{
                                            MaybeSerializeWrapper, NoSerialize as _,
                                        };
                                        MaybeSerializeWrapper(d).maybe_to_value()
                                    })
                                    .map(|d| schema.insert("default".to_owned(), d));
                                schema
                            },
                        );
                    }
                    schema
                }
            }
            fn inline_schema() -> bool {
                false
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for WorkloadSpecSection {
        #[inline]
        fn clone(&self) -> WorkloadSpecSection {
            WorkloadSpecSection {
                groups: ::core::clone::Clone::clone(&self.groups),
                key_space: ::core::clone::Clone::clone(&self.key_space),
                key_distribution: ::core::clone::Clone::clone(&self.key_distribution),
                skip_key_contains_check: ::core::clone::Clone::clone(
                    &self.skip_key_contains_check,
                ),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for WorkloadSpecSection {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "WorkloadSpecSection",
                "groups",
                &self.groups,
                "key_space",
                &self.key_space,
                "key_distribution",
                &self.key_distribution,
                "skip_key_contains_check",
                &&self.skip_key_contains_check,
            )
        }
    }
    pub struct WorkloadSpec {
        /// Sections of a workload where a key from one will (probably) not appear in another.
        pub(crate) sections: Vec<WorkloadSpecSection>,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for WorkloadSpec {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "sections" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"sections" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<WorkloadSpec>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = WorkloadSpec;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct WorkloadSpec",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Vec<WorkloadSpecSection>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct WorkloadSpec with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(WorkloadSpec { sections: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<
                            Vec<WorkloadSpecSection>,
                        > = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "sections",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Vec<WorkloadSpecSection>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("sections")?
                            }
                        };
                        _serde::__private::Ok(WorkloadSpec { sections: __field0 })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["sections"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "WorkloadSpec",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<WorkloadSpec>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    const _: () = {
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for WorkloadSpec {
            fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                schemars::_private::alloc::borrow::Cow::Borrowed("WorkloadSpec")
            }
            fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                schemars::_private::alloc::borrow::Cow::Borrowed(
                    "tectonic::spec::WorkloadSpec",
                )
            }
            fn json_schema(
                generator: &mut schemars::SchemaGenerator,
            ) -> schemars::Schema {
                {
                    let mut schema = ::schemars::Schema::try_from(
                            ::serde_json::Value::Object({
                                let mut object = ::serde_json::Map::new();
                                let _ = object
                                    .insert(
                                        ("type").into(),
                                        ::serde_json::to_value(&"object").unwrap(),
                                    );
                                object
                            }),
                        )
                        .unwrap();
                    {
                        schemars::_private::insert_object_property(
                            &mut schema,
                            "sections",
                            if generator.contract().is_serialize() {
                                false
                            } else {
                                false
                                    || (!false
                                        && <Vec<
                                            WorkloadSpecSection,
                                        > as schemars::JsonSchema>::_schemars_private_is_option())
                            },
                            {
                                let mut schema = generator
                                    .subschema_for::<Vec<WorkloadSpecSection>>();
                                const title_and_description: (&str, &str) = schemars::_private::get_title_and_description(
                                    "Sections of a workload where a key from one will (probably) not appear in another.",
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "title",
                                    title_and_description.0,
                                );
                                schemars::_private::insert_metadata_property_if_nonempty(
                                    &mut schema,
                                    "description",
                                    title_and_description.1,
                                );
                                schema
                            },
                        );
                    }
                    schema
                }
            }
            fn inline_schema() -> bool {
                false
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for WorkloadSpec {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "WorkloadSpec",
                "sections",
                &&self.sections,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for WorkloadSpec {
        #[inline]
        fn clone(&self) -> WorkloadSpec {
            WorkloadSpec {
                sections: ::core::clone::Clone::clone(&self.sections),
            }
        }
    }
}
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
#[automatically_derived]
impl ::core::fmt::Debug for Op {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                Op::Insert => "Insert",
                Op::Update => "Update",
                Op::PointDelete => "PointDelete",
                Op::PointDeleteEmpty => "PointDeleteEmpty",
                Op::RangeDelete => "RangeDelete",
                Op::PointQuery => "PointQuery",
                Op::EmptyPointQuery => "EmptyPointQuery",
                Op::RangeQuery => "RangeQuery",
            },
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Op {}
#[automatically_derived]
impl ::core::clone::Clone for Op {
    #[inline]
    fn clone(&self) -> Op {
        *self
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Op {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::cmp::Ord for Op {
    #[inline]
    fn cmp(&self, other: &Op) -> ::core::cmp::Ordering {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for Op {
    #[inline]
    fn partial_cmp(&self, other: &Op) -> ::core::option::Option<::core::cmp::Ordering> {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Op {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Op {
    #[inline]
    fn eq(&self, other: &Op) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
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
    for section in &workload.sections {
        let mut keys_valid = keyset_constructor(0);
        for group in &section.groups {
            let rng_ref = &mut rng;
            let mut markers: Vec<Op> = Vec::with_capacity(0);
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
                return ::anyhow::__private::Err({
                    let error = ::anyhow::__private::format_err(
                        format_args!(
                            "Cannot have more point deletes than existing valid keys.",
                        ),
                    );
                    error
                });
            }
            if more_delete_range_than_keys {
                return ::anyhow::__private::Err({
                    let error = ::anyhow::__private::format_err(
                        format_args!(
                            "Cannot have more range deletes than existing valid keys.",
                        ),
                    );
                    error
                });
            }
            if keys_valid.is_empty() {
                if insert_count == 0 {
                    return ::anyhow::__private::Err({
                        let error = ::anyhow::__private::format_err(
                            format_args!(
                                "Invalid workload spec. Group must have existing valid keys or have insert operations.",
                            ),
                        );
                        error
                    });
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
                        let is = group
                            .inserts
                            .as_ref()
                            .ok_or_else(|| {
                                ::anyhow::__private::must_use({
                                    let error = ::anyhow::__private::format_err(
                                        format_args!(
                                            "Insert marker can only appear when inserts is not None",
                                        ),
                                    );
                                    error
                                })
                            })?;
                        let key_len = is.key_len.evaluate(rng_ref) as usize;
                        let key = gen_string(rng_ref, key_len);
                        let len1 = is.val_len.evaluate(rng_ref) as usize;
                        let val = gen_string(rng_ref, len1);
                        AsciiOperationFormatter::write_insert(writer, &key, &val)?;
                        keys_valid.push(key);
                    }
                    Op::Update => {
                        let us = group
                            .updates
                            .as_ref()
                            .ok_or_else(|| {
                                ::anyhow::__private::must_use({
                                    let error = ::anyhow::__private::format_err(
                                        format_args!(
                                            "Update marker can only appear when updates is not None",
                                        ),
                                    );
                                    error
                                })
                            })?;
                        if keys_valid.is_empty() {
                            return ::anyhow::__private::Err({
                                let error = ::anyhow::__private::format_err(
                                    format_args!(
                                        "Cannot have updates when there are no valid keys.",
                                    ),
                                );
                                error
                            });
                        }
                        let key = keys_valid.get_random(rng_ref);
                        let len = us.val_len.evaluate(rng_ref) as usize;
                        let val = gen_string(rng_ref, len);
                        AsciiOperationFormatter::write_update(writer, key, &val)?;
                    }
                    Op::PointDelete => {
                        let key = keys_valid.remove_random(rng_ref);
                        AsciiOperationFormatter::write_point_delete(writer, &key)?;
                    }
                    Op::PointQuery => {
                        if keys_valid.is_empty() {
                            return ::anyhow::__private::Err({
                                let error = ::anyhow::__private::format_err(
                                    format_args!(
                                        "Cannot have point queries when there are no valid keys.",
                                    ),
                                );
                                error
                            });
                        }
                        let key = keys_valid.get_random(rng_ref);
                        AsciiOperationFormatter::write_point_query(writer, key)?
                    }
                    Op::PointDeleteEmpty => {
                        let epd = group
                            .empty_point_deletes
                            .as_ref()
                            .ok_or_else(|| {
                                ::anyhow::__private::must_use({
                                    let error = ::anyhow::__private::format_err(
                                        format_args!(
                                            "Empty point delete marker can only appear when empty_point_deletes is not None",
                                        ),
                                    );
                                    error
                                })
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
                        let epq = group
                            .empty_point_queries
                            .as_ref()
                            .ok_or_else(|| {
                                ::anyhow::__private::must_use({
                                    let error = ::anyhow::__private::format_err(
                                        format_args!(
                                            "Empty point query marker can only appear when empty_point_queries is not None",
                                        ),
                                    );
                                    error
                                })
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
                        let rqs = group
                            .range_queries
                            .as_ref()
                            .ok_or_else(|| {
                                ::anyhow::__private::must_use({
                                    let error = ::anyhow::__private::format_err(
                                        format_args!(
                                            "Range query marker can only appear when range_queries is not None",
                                        ),
                                    );
                                    error
                                })
                            })?;
                        if keys_valid.is_empty() {
                            return ::anyhow::__private::Err({
                                let error = ::anyhow::__private::format_err(
                                    format_args!(
                                        "Cannot have range queries when there are no valid keys.",
                                    ),
                                );
                                error
                            });
                        }
                        keys_valid.sort();
                        let num_items = (rqs.selectivity.evaluate(rng_ref)
                            * (keys_valid.len() as f32).floor()) as usize;
                        let start_range = 0..keys_valid.len() - num_items;
                        let start_idx = rng_ref.random_range(start_range);
                        let key1 = keys_valid
                            .get(start_idx)
                            .expect("index to be in range");
                        let key2 = keys_valid
                            .get(start_idx + num_items)
                            .expect("index to be in range");
                        AsciiOperationFormatter::write_range_query(writer, key1, key2)?
                    }
                    Op::RangeDelete => {
                        let rds = group
                            .range_deletes
                            .as_ref()
                            .ok_or_else(|| {
                                ::anyhow::__private::must_use({
                                    let error = ::anyhow::__private::format_err(
                                        format_args!(
                                            "RangeDelete marker can only appear when range_deletes is not None",
                                        ),
                                    );
                                    error
                                })
                            })?;
                        if keys_valid.is_empty() {
                            return ::anyhow::__private::Err({
                                let error = ::anyhow::__private::format_err(
                                    format_args!(
                                        "Cannot have range deletes when there are no valid keys.",
                                    ),
                                );
                                error
                            });
                        }
                        keys_valid.sort();
                        let num_items = (rds.selectivity.evaluate(rng_ref)
                            * (keys_valid.len() as f32).floor()) as usize;
                        let start_range = 0..keys_valid.len() - num_items;
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
pub fn generate_workload(
    workload_spec_string: &str,
    output_file: PathBuf,
) -> Result<()> {
    let workload_spec: WorkloadSpec = serde_json::from_str(workload_spec_string)
        .context("Parsing spec file")?;
    let mut buf_writer = BufWriter::with_capacity(
        1024 * 1024,
        File::create(output_file)?,
    );
    write_operations(&mut buf_writer, &workload_spec)?;
    buf_writer.flush()?;
    Ok(())
}
pub fn generate_workload_spec_schema() -> serde_json::Result<String> {
    let schema = ::schemars::SchemaGenerator::default()
        .into_root_schema_for::<crate::spec::WorkloadSpec>();
    return serde_json::to_string_pretty(&schema);
}
