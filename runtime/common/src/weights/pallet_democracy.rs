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
//! Autogenerated weights for `pallet_democracy`
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
// --pallet=pallet_democracy
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

/// Weight functions for `pallet_democracy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_democracy::WeightInfo for WeightInfo<T> {
	/// Storage: Democracy PublicPropCount (r:1 w:1)
	/// Proof: Democracy PublicPropCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Democracy PublicProps (r:1 w:1)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(15502), added: 15997, mode: MaxEncodedLen)
	/// Storage: Democracy Blacklist (r:1 w:0)
	/// Proof: Democracy Blacklist (max_values: None, max_size: Some(2038), added: 4513, mode: MaxEncodedLen)
	/// Storage: Democracy DepositOf (r:0 w:1)
	/// Proof: Democracy DepositOf (max_values: None, max_size: Some(2030), added: 4505, mode: MaxEncodedLen)
	fn propose() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3843`
		//  Estimated: `16987`
		// Minimum execution time: 32_803_000 picoseconds.
		Weight::from_parts(34_103_000, 0)
			.saturating_add(Weight::from_parts(0, 16987))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy DepositOf (r:1 w:1)
	/// Proof: Democracy DepositOf (max_values: None, max_size: Some(2030), added: 4505, mode: MaxEncodedLen)
	fn second() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2559`
		//  Estimated: `5495`
		// Minimum execution time: 27_837_000 picoseconds.
		Weight::from_parts(29_793_000, 0)
			.saturating_add(Weight::from_parts(0, 5495))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy VotingOf (r:1 w:1)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3783), added: 6258, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1287), added: 3762, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(37), added: 2512, mode: MaxEncodedLen)
	fn vote_new() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3511`
		//  Estimated: `7248`
		// Minimum execution time: 50_224_000 picoseconds.
		Weight::from_parts(52_539_000, 0)
			.saturating_add(Weight::from_parts(0, 7248))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy VotingOf (r:1 w:1)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3783), added: 6258, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1287), added: 3762, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(37), added: 2512, mode: MaxEncodedLen)
	fn vote_existing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3533`
		//  Estimated: `7248`
		// Minimum execution time: 51_739_000 picoseconds.
		Weight::from_parts(54_051_000, 0)
			.saturating_add(Weight::from_parts(0, 7248))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy Cancellations (r:1 w:1)
	/// Proof: Democracy Cancellations (max_values: None, max_size: Some(33), added: 2508, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn emergency_cancel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `299`
		//  Estimated: `3666`
		// Minimum execution time: 16_811_000 picoseconds.
		Weight::from_parts(17_479_000, 0)
			.saturating_add(Weight::from_parts(0, 3666))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy PublicProps (r:1 w:1)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(15502), added: 15997, mode: MaxEncodedLen)
	/// Storage: Democracy DepositOf (r:1 w:1)
	/// Proof: Democracy DepositOf (max_values: None, max_size: Some(2030), added: 4505, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:3 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	/// Storage: Democracy NextExternal (r:1 w:1)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy Blacklist (r:0 w:1)
	/// Proof: Democracy Blacklist (max_values: None, max_size: Some(2038), added: 4513, mode: MaxEncodedLen)
	fn blacklist() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4659`
		//  Estimated: `16987`
		// Minimum execution time: 66_491_000 picoseconds.
		Weight::from_parts(68_824_000, 0)
			.saturating_add(Weight::from_parts(0, 16987))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: Democracy NextExternal (r:1 w:1)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Democracy Blacklist (r:1 w:0)
	/// Proof: Democracy Blacklist (max_values: None, max_size: Some(2038), added: 4513, mode: MaxEncodedLen)
	fn external_propose() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2161`
		//  Estimated: `5503`
		// Minimum execution time: 10_383_000 picoseconds.
		Weight::from_parts(10_791_000, 0)
			.saturating_add(Weight::from_parts(0, 5503))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy NextExternal (r:0 w:1)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	fn external_propose_majority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_724_000 picoseconds.
		Weight::from_parts(1_844_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy NextExternal (r:0 w:1)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	fn external_propose_default() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_710_000 picoseconds.
		Weight::from_parts(1_827_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy NextExternal (r:1 w:1)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumCount (r:1 w:1)
	/// Proof: Democracy ReferendumCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:2)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	fn fast_track() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `219`
		//  Estimated: `3518`
		// Minimum execution time: 16_946_000 picoseconds.
		Weight::from_parts(17_218_000, 0)
			.saturating_add(Weight::from_parts(0, 3518))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Democracy NextExternal (r:1 w:1)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Democracy Blacklist (r:1 w:1)
	/// Proof: Democracy Blacklist (max_values: None, max_size: Some(2038), added: 4513, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn veto_external() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2264`
		//  Estimated: `5503`
		// Minimum execution time: 19_829_000 picoseconds.
		Weight::from_parts(20_342_000, 0)
			.saturating_add(Weight::from_parts(0, 5503))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy PublicProps (r:1 w:1)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(15502), added: 15997, mode: MaxEncodedLen)
	/// Storage: Democracy DepositOf (r:1 w:1)
	/// Proof: Democracy DepositOf (max_values: None, max_size: Some(2030), added: 4505, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn cancel_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4558`
		//  Estimated: `16987`
		// Minimum execution time: 52_685_000 picoseconds.
		Weight::from_parts(53_529_000, 0)
			.saturating_add(Weight::from_parts(0, 16987))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	fn cancel_referendum() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `204`
		//  Estimated: `3518`
		// Minimum execution time: 12_161_000 picoseconds.
		Weight::from_parts(12_468_000, 0)
			.saturating_add(Weight::from_parts(0, 3518))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Democracy LowestUnbaked (r:1 w:1)
	/// Proof: Democracy LowestUnbaked (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumCount (r:1 w:0)
	/// Proof: Democracy ReferendumCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:99 w:0)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 99]`.
	fn on_initialize_base(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `177 + r * (86 ±0)`
		//  Estimated: `1489 + r * (2676 ±0)`
		// Minimum execution time: 5_777_000 picoseconds.
		Weight::from_parts(7_012_043, 0)
			.saturating_add(Weight::from_parts(0, 1489))
			// Standard Error: 5_978
			.saturating_add(Weight::from_parts(2_502_390, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 2676).saturating_mul(r.into()))
	}
	/// Storage: Democracy LowestUnbaked (r:1 w:1)
	/// Proof: Democracy LowestUnbaked (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumCount (r:1 w:0)
	/// Proof: Democracy ReferendumCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Democracy LastTabledWasExternal (r:1 w:0)
	/// Proof: Democracy LastTabledWasExternal (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: Democracy NextExternal (r:1 w:0)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Democracy PublicProps (r:1 w:0)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(15502), added: 15997, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:99 w:0)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 99]`.
	fn on_initialize_base_with_launch_period(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `177 + r * (86 ±0)`
		//  Estimated: `16987 + r * (2676 ±0)`
		// Minimum execution time: 7_477_000 picoseconds.
		Weight::from_parts(8_822_468, 0)
			.saturating_add(Weight::from_parts(0, 16987))
			// Standard Error: 6_000
			.saturating_add(Weight::from_parts(2_499_007, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 2676).saturating_mul(r.into()))
	}
	/// Storage: Democracy VotingOf (r:3 w:3)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3783), added: 6258, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:99 w:99)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1287), added: 3762, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(37), added: 2512, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 99]`.
	fn delegate(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `832 + r * (108 ±0)`
		//  Estimated: `19764 + r * (2676 ±0)`
		// Minimum execution time: 30_870_000 picoseconds.
		Weight::from_parts(34_012_983, 0)
			.saturating_add(Weight::from_parts(0, 19764))
			// Standard Error: 7_720
			.saturating_add(Weight::from_parts(3_312_623, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 2676).saturating_mul(r.into()))
	}
	/// Storage: Democracy VotingOf (r:2 w:2)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3783), added: 6258, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:99 w:99)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 99]`.
	fn undelegate(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `388 + r * (108 ±0)`
		//  Estimated: `13506 + r * (2676 ±0)`
		// Minimum execution time: 15_068_000 picoseconds.
		Weight::from_parts(14_614_221, 0)
			.saturating_add(Weight::from_parts(0, 13506))
			// Standard Error: 8_448
			.saturating_add(Weight::from_parts(3_321_360, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 2676).saturating_mul(r.into()))
	}
	/// Storage: Democracy PublicProps (r:0 w:1)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(15502), added: 15997, mode: MaxEncodedLen)
	fn clear_public_proposals() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_857_000 picoseconds.
		Weight::from_parts(1_928_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy VotingOf (r:1 w:1)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3783), added: 6258, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1287), added: 3762, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(37), added: 2512, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 99]`.
	fn unlock_remove(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `421`
		//  Estimated: `7248`
		// Minimum execution time: 15_518_000 picoseconds.
		Weight::from_parts(24_973_704, 0)
			.saturating_add(Weight::from_parts(0, 7248))
			// Standard Error: 2_611
			.saturating_add(Weight::from_parts(110_327, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy VotingOf (r:1 w:1)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3783), added: 6258, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1287), added: 3762, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(37), added: 2512, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 99]`.
	fn unlock_set(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `422 + r * (22 ±0)`
		//  Estimated: `7248`
		// Minimum execution time: 22_004_000 picoseconds.
		Weight::from_parts(24_651_431, 0)
			.saturating_add(Weight::from_parts(0, 7248))
			// Standard Error: 1_686
			.saturating_add(Weight::from_parts(148_896, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy VotingOf (r:1 w:1)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3783), added: 6258, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 100]`.
	fn remove_vote(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `649 + r * (26 ±0)`
		//  Estimated: `7248`
		// Minimum execution time: 11_501_000 picoseconds.
		Weight::from_parts(13_983_240, 0)
			.saturating_add(Weight::from_parts(0, 7248))
			// Standard Error: 2_216
			.saturating_add(Weight::from_parts(151_939, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy VotingOf (r:1 w:1)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3783), added: 6258, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 100]`.
	fn remove_other_vote(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `649 + r * (26 ±0)`
		//  Estimated: `7248`
		// Minimum execution time: 11_544_000 picoseconds.
		Weight::from_parts(13_763_613, 0)
			.saturating_add(Weight::from_parts(0, 7248))
			// Standard Error: 2_635
			.saturating_add(Weight::from_parts(163_738, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Democracy NextExternal (r:1 w:0)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Preimage StatusFor (r:1 w:0)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(79), added: 2554, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:0 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn set_external_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `289`
		//  Estimated: `3544`
		// Minimum execution time: 13_407_000 picoseconds.
		Weight::from_parts(13_786_000, 0)
			.saturating_add(Weight::from_parts(0, 3544))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy NextExternal (r:1 w:0)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn clear_external_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `219`
		//  Estimated: `3518`
		// Minimum execution time: 12_151_000 picoseconds.
		Weight::from_parts(12_429_000, 0)
			.saturating_add(Weight::from_parts(0, 3518))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy PublicProps (r:1 w:0)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(15502), added: 15997, mode: MaxEncodedLen)
	/// Storage: Preimage StatusFor (r:1 w:0)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(79), added: 2554, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:0 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn set_proposal_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3621`
		//  Estimated: `16987`
		// Minimum execution time: 34_981_000 picoseconds.
		Weight::from_parts(36_458_000, 0)
			.saturating_add(Weight::from_parts(0, 16987))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy PublicProps (r:1 w:0)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(15502), added: 15997, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn clear_proposal_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3555`
		//  Estimated: `16987`
		// Minimum execution time: 28_952_000 picoseconds.
		Weight::from_parts(29_835_000, 0)
			.saturating_add(Weight::from_parts(0, 16987))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Preimage StatusFor (r:1 w:0)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(79), added: 2554, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:0 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn set_referendum_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `144`
		//  Estimated: `3544`
		// Minimum execution time: 9_258_000 picoseconds.
		Weight::from_parts(9_459_000, 0)
			.saturating_add(Weight::from_parts(0, 3544))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy ReferendumInfoOf (r:1 w:0)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn clear_referendum_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `235`
		//  Estimated: `3666`
		// Minimum execution time: 13_244_000 picoseconds.
		Weight::from_parts(13_701_000, 0)
			.saturating_add(Weight::from_parts(0, 3666))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
