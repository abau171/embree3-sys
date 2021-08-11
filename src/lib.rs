//! Note: This documentation is generated from a specific configuration of
//! Embree and may not be accurate if your local configuration is different. The
//! [docs.rs](https://docs.rs/embree3-sys/) version should reflect Embree's
//! default configuration.
//!
//! In particular, if you have increased `EMBREE_MAX_INSTANCE_LEVEL_COUNT` from
//! its default of `1` to enable nested instanceing, then the length of
//! [`RTCIntersectContext`](RTCIntersectContext)'s `instID` array will change to
//! reflect this on your local build.

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
