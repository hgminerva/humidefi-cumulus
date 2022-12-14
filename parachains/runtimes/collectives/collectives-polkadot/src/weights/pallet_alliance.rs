// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_alliance`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-11, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm4`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("collectives-polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./artifacts/polkadot-parachain
// benchmark
// pallet
// --chain=collectives-polkadot-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_alliance
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./parachains/runtimes/collectives/collectives-polkadot/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_alliance`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_alliance::WeightInfo for WeightInfo<T> {
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion ProposalCount (r:1 w:1)
	// Storage: AllianceMotion Voting (r:0 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[0, 90]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(_b: u32, _x: u32, _y: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(53_023_000 as u64)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(132_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Alliance Members (r:2 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	/// The range of component `x` is `[3, 10]`.
	/// The range of component `y` is `[2, 90]`.
	fn vote(x: u32, y: u32, ) -> Weight {
		Weight::from_ref_time(33_003_000 as u64)
			// Standard Error: 106_000
			.saturating_add(Weight::from_ref_time(312_000 as u64).saturating_mul(x as u64))
			// Standard Error: 4_000
			.saturating_add(Weight::from_ref_time(82_000 as u64).saturating_mul(y as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion Voting (r:0 w:1)
	/// The range of component `p` is `[1, 100]`.
	fn veto(p: u32, ) -> Weight {
		Weight::from_ref_time(27_289_000 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(131_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion ProposalOf (r:0 w:1)
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[2, 90]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(x: u32, y: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(31_884_000 as u64)
			// Standard Error: 72_000
			.saturating_add(Weight::from_ref_time(358_000 as u64).saturating_mul(x as u64))
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(109_000 as u64).saturating_mul(y as u64))
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(109_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[2, 90]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, x: u32, y: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(43_205_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(2_000 as u64).saturating_mul(b as u64))
			// Standard Error: 73_000
			.saturating_add(Weight::from_ref_time(123_000 as u64).saturating_mul(x as u64))
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(91_000 as u64).saturating_mul(y as u64))
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(114_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion Prime (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: Alliance Rule (r:0 w:1)
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[2, 90]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(x: u32, y: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(49_769_000 as u64)
			// Standard Error: 69_000
			.saturating_add(Weight::from_ref_time(352_000 as u64).saturating_mul(x as u64))
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(81_000 as u64).saturating_mul(y as u64))
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(100_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion Prime (r:1 w:0)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion ProposalOf (r:0 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[2, 90]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(b: u32, _x: u32, y: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(40_901_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(3_000 as u64).saturating_mul(b as u64))
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(79_000 as u64).saturating_mul(y as u64))
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(102_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:3 w:3)
	// Storage: AllianceMotion Members (r:1 w:1)
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[0, 90]`.
	/// The range of component `z` is `[0, 100]`.
	fn init_members(_x: u32, y: u32, z: u32, ) -> Weight {
		Weight::from_ref_time(39_647_000 as u64)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(123_000 as u64).saturating_mul(y as u64))
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(116_000 as u64).saturating_mul(z as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Alliance Rule (r:0 w:1)
	fn set_rule() -> Weight {
		Weight::from_ref_time(14_689_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Alliance Announcements (r:1 w:1)
	fn announce() -> Weight {
		Weight::from_ref_time(15_988_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Alliance Announcements (r:1 w:1)
	fn remove_announcement() -> Weight {
		Weight::from_ref_time(16_823_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Alliance UnscrupulousAccounts (r:1 w:0)
	// Storage: Alliance Members (r:4 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Alliance DepositOf (r:0 w:1)
	fn join_alliance() -> Weight {
		Weight::from_ref_time(46_340_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:4 w:1)
	// Storage: Alliance UnscrupulousAccounts (r:1 w:0)
	fn nominate_ally() -> Weight {
		Weight::from_ref_time(36_225_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Alliance Members (r:3 w:2)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	fn elevate_ally() -> Weight {
		Weight::from_ref_time(30_236_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Alliance UpForKicking (r:1 w:0)
	// Storage: Alliance Members (r:3 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: Alliance DepositOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	fn retire() -> Weight {
		Weight::from_ref_time(44_311_000 as u64)
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Alliance Members (r:3 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: Alliance DepositOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	// Storage: Alliance UpForKicking (r:0 w:1)
	fn kick_member() -> Weight {
		Weight::from_ref_time(46_112_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Alliance UnscrupulousAccounts (r:1 w:1)
	// Storage: Alliance UnscrupulousWebsites (r:1 w:1)
	/// The range of component `n` is `[1, 100]`.
	/// The range of component `l` is `[1, 255]`.
	fn add_unscrupulous_items(n: u32, l: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(1_300_000 as u64).saturating_mul(n as u64))
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(125_000 as u64).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Alliance UnscrupulousAccounts (r:1 w:1)
	// Storage: Alliance UnscrupulousWebsites (r:1 w:1)
	/// The range of component `n` is `[1, 100]`.
	/// The range of component `l` is `[1, 255]`.
	fn remove_unscrupulous_items(n: u32, l: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 197_000
			.saturating_add(Weight::from_ref_time(34_156_000 as u64).saturating_mul(n as u64))
			// Standard Error: 87_000
			.saturating_add(Weight::from_ref_time(7_042_000 as u64).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
