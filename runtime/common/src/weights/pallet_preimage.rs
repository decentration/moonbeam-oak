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
//! Autogenerated weights for `pallet_preimage`
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
// --pallet=pallet_preimage
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

/// Weight functions for `pallet_preimage`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_preimage::WeightInfo for WeightInfo<T> {
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(79), added: 2554, mode: MaxEncodedLen)
	/// Storage: Preimage PreimageFor (r:0 w:1)
	/// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 4194304]`.
	fn note_preimage(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `275`
		//  Estimated: `3544`
		// Minimum execution time: 17_479_000 picoseconds.
		Weight::from_parts(17_948_000, 0)
			.saturating_add(Weight::from_parts(0, 3544))
			// Standard Error: 2
			.saturating_add(Weight::from_parts(1_515, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(79), added: 2554, mode: MaxEncodedLen)
	/// Storage: Preimage PreimageFor (r:0 w:1)
	/// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 4194304]`.
	fn note_requested_preimage(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `106`
		//  Estimated: `3544`
		// Minimum execution time: 9_432_000 picoseconds.
		Weight::from_parts(9_706_000, 0)
			.saturating_add(Weight::from_parts(0, 3544))
			// Standard Error: 8
			.saturating_add(Weight::from_parts(1_544, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(79), added: 2554, mode: MaxEncodedLen)
	/// Storage: Preimage PreimageFor (r:0 w:1)
	/// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 4194304]`.
	fn note_no_deposit_preimage(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `106`
		//  Estimated: `3544`
		// Minimum execution time: 9_246_000 picoseconds.
		Weight::from_parts(9_530_000, 0)
			.saturating_add(Weight::from_parts(0, 3544))
			// Standard Error: 19
			.saturating_add(Weight::from_parts(1_812, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(79), added: 2554, mode: MaxEncodedLen)
	/// Storage: Preimage PreimageFor (r:0 w:1)
	/// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: MaxEncodedLen)
	fn unnote_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `409`
		//  Estimated: `3544`
		// Minimum execution time: 45_116_000 picoseconds.
		Weight::from_parts(48_539_000, 0)
			.saturating_add(Weight::from_parts(0, 3544))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(79), added: 2554, mode: MaxEncodedLen)
	/// Storage: Preimage PreimageFor (r:0 w:1)
	/// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: MaxEncodedLen)
	fn unnote_no_deposit_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `144`
		//  Estimated: `3544`
		// Minimum execution time: 28_035_000 picoseconds.
		Weight::from_parts(31_391_000, 0)
			.saturating_add(Weight::from_parts(0, 3544))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(79), added: 2554, mode: MaxEncodedLen)
	fn request_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `176`
		//  Estimated: `3544`
		// Minimum execution time: 31_809_000 picoseconds.
		Weight::from_parts(35_417_000, 0)
			.saturating_add(Weight::from_parts(0, 3544))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(79), added: 2554, mode: MaxEncodedLen)
	fn request_no_deposit_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `144`
		//  Estimated: `3544`
		// Minimum execution time: 16_663_000 picoseconds.
		Weight::from_parts(18_857_000, 0)
			.saturating_add(Weight::from_parts(0, 3544))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(79), added: 2554, mode: MaxEncodedLen)
	fn request_unnoted_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3544`
		// Minimum execution time: 14_564_000 picoseconds.
		Weight::from_parts(17_944_000, 0)
			.saturating_add(Weight::from_parts(0, 3544))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(79), added: 2554, mode: MaxEncodedLen)
	fn request_requested_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `106`
		//  Estimated: `3544`
		// Minimum execution time: 7_593_000 picoseconds.
		Weight::from_parts(8_147_000, 0)
			.saturating_add(Weight::from_parts(0, 3544))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(79), added: 2554, mode: MaxEncodedLen)
	/// Storage: Preimage PreimageFor (r:0 w:1)
	/// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: MaxEncodedLen)
	fn unrequest_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `144`
		//  Estimated: `3544`
		// Minimum execution time: 30_123_000 picoseconds.
		Weight::from_parts(31_994_000, 0)
			.saturating_add(Weight::from_parts(0, 3544))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(79), added: 2554, mode: MaxEncodedLen)
	fn unrequest_unnoted_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `106`
		//  Estimated: `3544`
		// Minimum execution time: 6_841_000 picoseconds.
		Weight::from_parts(7_384_000, 0)
			.saturating_add(Weight::from_parts(0, 3544))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(79), added: 2554, mode: MaxEncodedLen)
	fn unrequest_multi_referenced_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `106`
		//  Estimated: `3544`
		// Minimum execution time: 6_389_000 picoseconds.
		Weight::from_parts(7_079_000, 0)
			.saturating_add(Weight::from_parts(0, 3544))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
