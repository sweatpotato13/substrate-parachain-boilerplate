

//! Autogenerated weights for pallet_utility
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-29, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("wisp-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/wisp
// benchmark
// pallet
// --chain=wisp-dev
// --steps=50
// --repeat=20
// --pallet=pallet_utility
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./scripts/benchmarking/frame-weights-output/pallet_utility.rs
// --template=.github/resources/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_utility.
pub trait WeightInfo {
    fn batch(c: u32, ) -> Weight;
    fn as_derivative() -> Weight;
    fn batch_all(c: u32, ) -> Weight;
    fn dispatch_as() -> Weight;
    fn force_batch(c: u32, ) -> Weight;
}

/// Weights for pallet_utility using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for SubstrateWeight<T> {
    fn batch(c: u32, ) -> Weight {
        Weight::from_ref_time(22_752_000)
            // Standard Error: 3_000
            .saturating_add(Weight::from_ref_time(4_059_000).saturating_mul(c as u64))
    }
    fn as_derivative() -> Weight {
        Weight::from_ref_time(5_360_000)
    }
    fn batch_all(c: u32, ) -> Weight {
        Weight::from_ref_time(30_482_000)
            // Standard Error: 3_000
            .saturating_add(Weight::from_ref_time(4_239_000).saturating_mul(c as u64))
    }
    fn dispatch_as() -> Weight {
        Weight::from_ref_time(12_883_000)
    }
    fn force_batch(c: u32, ) -> Weight {
        Weight::from_ref_time(28_793_000)
            // Standard Error: 3_000
            .saturating_add(Weight::from_ref_time(4_043_000).saturating_mul(c as u64))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn batch(c: u32, ) -> Weight {
        Weight::from_ref_time(22_752_000)
            // Standard Error: 3_000
            .saturating_add(Weight::from_ref_time(4_059_000).saturating_mul(c as u64))
    }
    fn as_derivative() -> Weight {
        Weight::from_ref_time(5_360_000)
    }
    fn batch_all(c: u32, ) -> Weight {
        Weight::from_ref_time(30_482_000)
            // Standard Error: 3_000
            .saturating_add(Weight::from_ref_time(4_239_000).saturating_mul(c as u64))
    }
    fn dispatch_as() -> Weight {
        Weight::from_ref_time(12_883_000)
    }
    fn force_batch(c: u32, ) -> Weight {
        Weight::from_ref_time(28_793_000)
            // Standard Error: 3_000
            .saturating_add(Weight::from_ref_time(4_043_000).saturating_mul(c as u64))
    }
}
