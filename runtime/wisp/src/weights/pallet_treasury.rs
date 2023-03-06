// Copyright 2020-2023 Manta Network.
// This file is part of Manta.
//
// Manta is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Manta is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Manta.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_treasury
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-29, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("wisp-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/manta
// benchmark
// pallet
// --chain=wisp-dev
// --steps=50
// --repeat=20
// --pallet=pallet_treasury
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./scripts/benchmarking/frame-weights-output/pallet_treasury.rs
// --template=.github/resources/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_treasury.
pub trait WeightInfo {
    fn spend() -> Weight;
    fn propose_spend() -> Weight;
    fn reject_proposal() -> Weight;
    fn approve_proposal(p: u32, ) -> Weight;
    fn remove_approval() -> Weight;
    fn on_initialize_proposals(p: u32, ) -> Weight;
}

/// Weights for pallet_treasury using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_treasury::WeightInfo for SubstrateWeight<T> {
    fn spend() -> Weight {
        Weight::from_ref_time(145_000)
    }
    // Storage: Treasury ProposalCount (r:1 w:1)
    // Storage: Treasury Proposals (r:0 w:1)
    fn propose_spend() -> Weight {
        Weight::from_ref_time(27_978_000)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    // Storage: Treasury Proposals (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    fn reject_proposal() -> Weight {
        Weight::from_ref_time(51_493_000)
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    // Storage: Treasury Proposals (r:1 w:0)
    // Storage: Treasury Approvals (r:1 w:1)
    fn approve_proposal(p: u32, ) -> Weight {
        Weight::from_ref_time(12_942_000)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(116_000).saturating_mul(p as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Treasury Approvals (r:1 w:1)
    fn remove_approval() -> Weight {
        Weight::from_ref_time(8_605_000)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Treasury Approvals (r:1 w:1)
    // Storage: Treasury Proposals (r:2 w:2)
    // Storage: System Account (r:4 w:4)
    fn on_initialize_proposals(p: u32, ) -> Weight {
        Weight::from_ref_time(31_550_000)
            // Standard Error: 34_000
            .saturating_add(Weight::from_ref_time(32_737_000).saturating_mul(p as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(p as u64)))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
            .saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(p as u64)))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn spend() -> Weight {
        Weight::from_ref_time(145_000)
    }
    // Storage: Treasury ProposalCount (r:1 w:1)
    // Storage: Treasury Proposals (r:0 w:1)
    fn propose_spend() -> Weight {
        Weight::from_ref_time(27_978_000)
            .saturating_add(RocksDbWeight::get().reads(1 as u64))
            .saturating_add(RocksDbWeight::get().writes(2 as u64))
    }
    // Storage: Treasury Proposals (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    fn reject_proposal() -> Weight {
        Weight::from_ref_time(51_493_000)
            .saturating_add(RocksDbWeight::get().reads(2 as u64))
            .saturating_add(RocksDbWeight::get().writes(2 as u64))
    }
    // Storage: Treasury Proposals (r:1 w:0)
    // Storage: Treasury Approvals (r:1 w:1)
    fn approve_proposal(p: u32, ) -> Weight {
        Weight::from_ref_time(12_942_000)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(116_000).saturating_mul(p as u64))
            .saturating_add(RocksDbWeight::get().reads(2 as u64))
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    // Storage: Treasury Approvals (r:1 w:1)
    fn remove_approval() -> Weight {
        Weight::from_ref_time(8_605_000)
            .saturating_add(RocksDbWeight::get().reads(1 as u64))
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    // Storage: Treasury Approvals (r:1 w:1)
    // Storage: Treasury Proposals (r:2 w:2)
    // Storage: System Account (r:4 w:4)
    fn on_initialize_proposals(p: u32, ) -> Weight {
        Weight::from_ref_time(31_550_000)
            // Standard Error: 34_000
            .saturating_add(Weight::from_ref_time(32_737_000).saturating_mul(p as u64))
            .saturating_add(RocksDbWeight::get().reads(1 as u64))
            .saturating_add(RocksDbWeight::get().reads((3 as u64).saturating_mul(p as u64)))
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
            .saturating_add(RocksDbWeight::get().writes((3 as u64).saturating_mul(p as u64)))
    }
}