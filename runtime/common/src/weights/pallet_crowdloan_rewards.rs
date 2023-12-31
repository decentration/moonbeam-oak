// Copyright 2019-2022 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_crowdloan_rewards`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-18, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-0-0-176`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("moonbase-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/moonbeam
// benchmark
// pallet
// --chain=moonbase-dev
// --steps=50
// --repeat=20
// --pallet=pallet_crowdloan_rewards
// --extrinsic=*
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/common/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_crowdloan_rewards`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_crowdloan_rewards::WeightInfo for WeightInfo<T> {
	/// Storage: CrowdloanRewards Initialized (r:1 w:0)
	/// Proof Skipped: CrowdloanRewards Initialized (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards InitializedRewardAmount (r:1 w:1)
	/// Proof Skipped: CrowdloanRewards InitializedRewardAmount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards TotalContributors (r:1 w:1)
	/// Proof Skipped: CrowdloanRewards TotalContributors (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:501 w:501)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// Storage: CrowdloanRewards ClaimedRelayChainIds (r:500 w:500)
	/// Proof Skipped: CrowdloanRewards ClaimedRelayChainIds (max_values: None, max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards UnassociatedContributions (r:500 w:0)
	/// Proof Skipped: CrowdloanRewards UnassociatedContributions (max_values: None, max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards AccountsPayable (r:500 w:500)
	/// Proof Skipped: CrowdloanRewards AccountsPayable (max_values: None, max_size: None, mode: Measured)
	/// The range of component `x` is `[1, 500]`.
	fn initialize_reward_vec(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `75944 + x * (659 ±0)`
		//  Estimated: `66066 + x * (3161 ±5)`
		// Minimum execution time: 114_774_000 picoseconds.
		Weight::from_parts(82_464_744, 0)
			.saturating_add(Weight::from_parts(0, 66066))
			// Standard Error: 29_101
			.saturating_add(Weight::from_parts(35_418_767, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(x.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(x.into())))
			.saturating_add(Weight::from_parts(0, 3161).saturating_mul(x.into()))
	}
	/// Storage: CrowdloanRewards Initialized (r:1 w:1)
	/// Proof Skipped: CrowdloanRewards Initialized (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards InitRelayBlock (r:1 w:0)
	/// Proof Skipped: CrowdloanRewards InitRelayBlock (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards InitializedRewardAmount (r:1 w:0)
	/// Proof Skipped: CrowdloanRewards InitializedRewardAmount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// Storage: CrowdloanRewards TotalContributors (r:1 w:0)
	/// Proof Skipped: CrowdloanRewards TotalContributors (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards EndRelayBlock (r:0 w:1)
	/// Proof Skipped: CrowdloanRewards EndRelayBlock (max_values: Some(1), max_size: None, mode: Measured)
	fn complete_initialization() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `513`
		//  Estimated: `3581`
		// Minimum execution time: 13_100_000 picoseconds.
		Weight::from_parts(13_556_000, 0)
			.saturating_add(Weight::from_parts(0, 3581))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: CrowdloanRewards Initialized (r:1 w:0)
	/// Proof Skipped: CrowdloanRewards Initialized (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards AccountsPayable (r:1 w:1)
	/// Proof Skipped: CrowdloanRewards AccountsPayable (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainSystem ValidationData (r:1 w:0)
	/// Proof Skipped: ParachainSystem ValidationData (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards InitRelayBlock (r:1 w:0)
	/// Proof Skipped: CrowdloanRewards InitRelayBlock (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards EndRelayBlock (r:1 w:0)
	/// Proof Skipped: CrowdloanRewards EndRelayBlock (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	fn claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `935`
		//  Estimated: `6172`
		// Minimum execution time: 40_876_000 picoseconds.
		Weight::from_parts(41_946_000, 0)
			.saturating_add(Weight::from_parts(0, 6172))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: CrowdloanRewards AccountsPayable (r:2 w:2)
	/// Proof Skipped: CrowdloanRewards AccountsPayable (max_values: None, max_size: None, mode: Measured)
	fn update_reward_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `397`
		//  Estimated: `6337`
		// Minimum execution time: 13_598_000 picoseconds.
		Weight::from_parts(14_043_000, 0)
			.saturating_add(Weight::from_parts(0, 6337))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: CrowdloanRewards UnassociatedContributions (r:1 w:1)
	/// Proof Skipped: CrowdloanRewards UnassociatedContributions (max_values: None, max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards ClaimedRelayChainIds (r:1 w:1)
	/// Proof Skipped: CrowdloanRewards ClaimedRelayChainIds (max_values: None, max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards AccountsPayable (r:1 w:1)
	/// Proof Skipped: CrowdloanRewards AccountsPayable (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	fn associate_native_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `769`
		//  Estimated: `6172`
		// Minimum execution time: 91_513_000 picoseconds.
		Weight::from_parts(92_998_000, 0)
			.saturating_add(Weight::from_parts(0, 6172))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: CrowdloanRewards AccountsPayable (r:2 w:2)
	/// Proof Skipped: CrowdloanRewards AccountsPayable (max_values: None, max_size: None, mode: Measured)
	/// The range of component `x` is `[1, 500]`.
	fn change_association_with_relay_keys(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `366 + x * (32 ±0)`
		//  Estimated: `6306 + x * (32 ±0)`
		// Minimum execution time: 69_231_000 picoseconds.
		Weight::from_parts(9_275_220, 0)
			.saturating_add(Weight::from_parts(0, 6306))
			// Standard Error: 2_627
			.saturating_add(Weight::from_parts(53_802_763, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(x.into()))
	}
}
