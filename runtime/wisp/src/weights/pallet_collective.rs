

//! Autogenerated weights for pallet_collective
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
// --pallet=pallet_collective
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./scripts/benchmarking/frame-weights-output/pallet_collective.rs
// --template=.github/resources/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_collective.
pub trait WeightInfo {
    fn set_members(m: u32, n: u32, p: u32, ) -> Weight;
    fn execute(b: u32, m: u32, ) -> Weight;
    fn propose_execute(b: u32, m: u32, ) -> Weight;
    fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight;
    fn vote(m: u32, ) -> Weight;
    fn close_early_disapproved(m: u32, p: u32, ) -> Weight;
    fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight;
    fn close_disapproved(m: u32, p: u32, ) -> Weight;
    fn close_approved(b: u32, m: u32, p: u32, ) -> Weight;
    fn disapprove_proposal(p: u32, ) -> Weight;
}

/// Weights for pallet_collective using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for SubstrateWeight<T> {
    // Storage: Council Members (r:1 w:1)
    // Storage: Council Proposals (r:1 w:0)
    // Storage: Council Voting (r:100 w:100)
    // Storage: Council Prime (r:0 w:1)
    fn set_members(m: u32, n: u32, p: u32, ) -> Weight {
        Weight::from_ref_time(0)
            // Standard Error: 7_000
            .saturating_add(Weight::from_ref_time(13_737_000).saturating_mul(m as u64))
            // Standard Error: 7_000
            .saturating_add(Weight::from_ref_time(33_000).saturating_mul(n as u64))
            // Standard Error: 7_000
            .saturating_add(Weight::from_ref_time(17_006_000).saturating_mul(p as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(p as u64)))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
            .saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(p as u64)))
    }
    // Storage: Council Members (r:1 w:0)
    fn execute(b: u32, m: u32, ) -> Weight {
        Weight::from_ref_time(18_411_000)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(2_000).saturating_mul(b as u64))
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(35_000).saturating_mul(m as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
    }
    // Storage: Council Members (r:1 w:0)
    // Storage: Council ProposalOf (r:1 w:0)
    fn propose_execute(b: u32, m: u32, ) -> Weight {
        Weight::from_ref_time(20_600_000)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(3_000).saturating_mul(b as u64))
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(68_000).saturating_mul(m as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
    }
    // Storage: Council Members (r:1 w:0)
    // Storage: Council ProposalOf (r:1 w:1)
    // Storage: Council Proposals (r:1 w:1)
    // Storage: Council ProposalCount (r:1 w:1)
    // Storage: Council Voting (r:0 w:1)
    fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
        Weight::from_ref_time(26_311_000)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(7_000).saturating_mul(b as u64))
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(52_000).saturating_mul(m as u64))
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(213_000).saturating_mul(p as u64))
            .saturating_add(T::DbWeight::get().reads(4 as u64))
            .saturating_add(T::DbWeight::get().writes(4 as u64))
    }
    // Storage: Council Members (r:1 w:0)
    // Storage: Council Voting (r:1 w:1)
    fn vote(m: u32, ) -> Weight {
        Weight::from_ref_time(34_738_000)
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(92_000).saturating_mul(m as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Council Voting (r:1 w:1)
    // Storage: Council Members (r:1 w:0)
    // Storage: Council Proposals (r:1 w:1)
    // Storage: Council ProposalOf (r:0 w:1)
    fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
        Weight::from_ref_time(33_338_000)
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(74_000).saturating_mul(m as u64))
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(160_000).saturating_mul(p as u64))
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: Council Voting (r:1 w:1)
    // Storage: Council Members (r:1 w:0)
    // Storage: Council ProposalOf (r:1 w:1)
    // Storage: Council Proposals (r:1 w:1)
    fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
        Weight::from_ref_time(36_217_000)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(7_000).saturating_mul(b as u64))
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(89_000).saturating_mul(m as u64))
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(207_000).saturating_mul(p as u64))
            .saturating_add(T::DbWeight::get().reads(4 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: Council Voting (r:1 w:1)
    // Storage: Council Members (r:1 w:0)
    // Storage: Council Prime (r:1 w:0)
    // Storage: Council Proposals (r:1 w:1)
    // Storage: Council ProposalOf (r:0 w:1)
    fn close_disapproved(m: u32, p: u32, ) -> Weight {
        Weight::from_ref_time(35_604_000)
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(89_000).saturating_mul(m as u64))
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(157_000).saturating_mul(p as u64))
            .saturating_add(T::DbWeight::get().reads(4 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: Council Voting (r:1 w:1)
    // Storage: Council Members (r:1 w:0)
    // Storage: Council Prime (r:1 w:0)
    // Storage: Council ProposalOf (r:1 w:1)
    // Storage: Council Proposals (r:1 w:1)
    fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
        Weight::from_ref_time(37_220_000)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(8_000).saturating_mul(b as u64))
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(99_000).saturating_mul(m as u64))
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(215_000).saturating_mul(p as u64))
            .saturating_add(T::DbWeight::get().reads(5 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: Council Proposals (r:1 w:1)
    // Storage: Council Voting (r:0 w:1)
    // Storage: Council ProposalOf (r:0 w:1)
    fn disapprove_proposal(p: u32, ) -> Weight {
        Weight::from_ref_time(21_135_000)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(212_000).saturating_mul(p as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    // Storage: Council Members (r:1 w:1)
    // Storage: Council Proposals (r:1 w:0)
    // Storage: Council Voting (r:100 w:100)
    // Storage: Council Prime (r:0 w:1)
    fn set_members(m: u32, n: u32, p: u32, ) -> Weight {
        Weight::from_ref_time(0)
            // Standard Error: 7_000
            .saturating_add(Weight::from_ref_time(13_737_000).saturating_mul(m as u64))
            // Standard Error: 7_000
            .saturating_add(Weight::from_ref_time(33_000).saturating_mul(n as u64))
            // Standard Error: 7_000
            .saturating_add(Weight::from_ref_time(17_006_000).saturating_mul(p as u64))
            .saturating_add(RocksDbWeight::get().reads(2 as u64))
            .saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(p as u64)))
            .saturating_add(RocksDbWeight::get().writes(2 as u64))
            .saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(p as u64)))
    }
    // Storage: Council Members (r:1 w:0)
    fn execute(b: u32, m: u32, ) -> Weight {
        Weight::from_ref_time(18_411_000)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(2_000).saturating_mul(b as u64))
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(35_000).saturating_mul(m as u64))
            .saturating_add(RocksDbWeight::get().reads(1 as u64))
    }
    // Storage: Council Members (r:1 w:0)
    // Storage: Council ProposalOf (r:1 w:0)
    fn propose_execute(b: u32, m: u32, ) -> Weight {
        Weight::from_ref_time(20_600_000)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(3_000).saturating_mul(b as u64))
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(68_000).saturating_mul(m as u64))
            .saturating_add(RocksDbWeight::get().reads(2 as u64))
    }
    // Storage: Council Members (r:1 w:0)
    // Storage: Council ProposalOf (r:1 w:1)
    // Storage: Council Proposals (r:1 w:1)
    // Storage: Council ProposalCount (r:1 w:1)
    // Storage: Council Voting (r:0 w:1)
    fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
        Weight::from_ref_time(26_311_000)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(7_000).saturating_mul(b as u64))
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(52_000).saturating_mul(m as u64))
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(213_000).saturating_mul(p as u64))
            .saturating_add(RocksDbWeight::get().reads(4 as u64))
            .saturating_add(RocksDbWeight::get().writes(4 as u64))
    }
    // Storage: Council Members (r:1 w:0)
    // Storage: Council Voting (r:1 w:1)
    fn vote(m: u32, ) -> Weight {
        Weight::from_ref_time(34_738_000)
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(92_000).saturating_mul(m as u64))
            .saturating_add(RocksDbWeight::get().reads(2 as u64))
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    // Storage: Council Voting (r:1 w:1)
    // Storage: Council Members (r:1 w:0)
    // Storage: Council Proposals (r:1 w:1)
    // Storage: Council ProposalOf (r:0 w:1)
    fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
        Weight::from_ref_time(33_338_000)
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(74_000).saturating_mul(m as u64))
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(160_000).saturating_mul(p as u64))
            .saturating_add(RocksDbWeight::get().reads(3 as u64))
            .saturating_add(RocksDbWeight::get().writes(3 as u64))
    }
    // Storage: Council Voting (r:1 w:1)
    // Storage: Council Members (r:1 w:0)
    // Storage: Council ProposalOf (r:1 w:1)
    // Storage: Council Proposals (r:1 w:1)
    fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
        Weight::from_ref_time(36_217_000)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(7_000).saturating_mul(b as u64))
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(89_000).saturating_mul(m as u64))
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(207_000).saturating_mul(p as u64))
            .saturating_add(RocksDbWeight::get().reads(4 as u64))
            .saturating_add(RocksDbWeight::get().writes(3 as u64))
    }
    // Storage: Council Voting (r:1 w:1)
    // Storage: Council Members (r:1 w:0)
    // Storage: Council Prime (r:1 w:0)
    // Storage: Council Proposals (r:1 w:1)
    // Storage: Council ProposalOf (r:0 w:1)
    fn close_disapproved(m: u32, p: u32, ) -> Weight {
        Weight::from_ref_time(35_604_000)
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(89_000).saturating_mul(m as u64))
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(157_000).saturating_mul(p as u64))
            .saturating_add(RocksDbWeight::get().reads(4 as u64))
            .saturating_add(RocksDbWeight::get().writes(3 as u64))
    }
    // Storage: Council Voting (r:1 w:1)
    // Storage: Council Members (r:1 w:0)
    // Storage: Council Prime (r:1 w:0)
    // Storage: Council ProposalOf (r:1 w:1)
    // Storage: Council Proposals (r:1 w:1)
    fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
        Weight::from_ref_time(37_220_000)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(8_000).saturating_mul(b as u64))
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(99_000).saturating_mul(m as u64))
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(215_000).saturating_mul(p as u64))
            .saturating_add(RocksDbWeight::get().reads(5 as u64))
            .saturating_add(RocksDbWeight::get().writes(3 as u64))
    }
    // Storage: Council Proposals (r:1 w:1)
    // Storage: Council Voting (r:0 w:1)
    // Storage: Council ProposalOf (r:0 w:1)
    fn disapprove_proposal(p: u32, ) -> Weight {
        Weight::from_ref_time(21_135_000)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(212_000).saturating_mul(p as u64))
            .saturating_add(RocksDbWeight::get().reads(1 as u64))
            .saturating_add(RocksDbWeight::get().writes(3 as u64))
    }
}
