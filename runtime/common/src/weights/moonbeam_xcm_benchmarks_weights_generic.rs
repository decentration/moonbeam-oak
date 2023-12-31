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
//! Autogenerated weights for `moonbeam_xcm_benchmarks::weights::generic`
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
// --pallet=moonbeam_xcm_benchmarks::weights::generic
// --extrinsic=*
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/common/src/weights/moonbeam_xcm_benchmarks_weights_generic.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `moonbeam_xcm_benchmarks::weights::generic`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> moonbeam_xcm_benchmarks::weights::generic::WeightInfo for WeightInfo<T> {
	/// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	/// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	/// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	fn report_holding() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `145`
		//  Estimated: `3610`
		// Minimum execution time: 130_619_000 picoseconds.
		Weight::from_parts(132_953_000, 0)
			.saturating_add(Weight::from_parts(0, 3610))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: AssetManager SupportedFeePaymentAssets (r:1 w:0)
	/// Proof Skipped: AssetManager SupportedFeePaymentAssets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AssetManager AssetTypeUnitsPerSecond (r:1 w:0)
	/// Proof Skipped: AssetManager AssetTypeUnitsPerSecond (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager AssetTypeId (r:1 w:0)
	/// Proof Skipped: AssetManager AssetTypeId (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(174), added: 2649, mode: MaxEncodedLen)
	fn buy_execution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2334`
		//  Estimated: `5799`
		// Minimum execution time: 53_807_000 picoseconds.
		Weight::from_parts(55_871_000, 0)
			.saturating_add(Weight::from_parts(0, 5799))
			.saturating_add(T::DbWeight::get().reads(4))
	}
	/// Storage: PolkadotXcm Queries (r:1 w:0)
	/// Proof Skipped: PolkadotXcm Queries (max_values: None, max_size: None, mode: Measured)
	fn query_response() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `69`
		//  Estimated: `3534`
		// Minimum execution time: 6_392_000 picoseconds.
		Weight::from_parts(6_578_000, 0)
			.saturating_add(Weight::from_parts(0, 3534))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: MaintenanceMode MaintenanceMode (r:1 w:0)
	/// Proof Skipped: MaintenanceMode MaintenanceMode (max_values: Some(1), max_size: None, mode: Measured)
	fn transact() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `1527`
		// Minimum execution time: 8_003_000 picoseconds.
		Weight::from_parts(8_246_000, 0)
			.saturating_add(Weight::from_parts(0, 1527))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	fn refund_surplus() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_152_000 picoseconds.
		Weight::from_parts(4_526_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	fn set_error_handler() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_043_000 picoseconds.
		Weight::from_parts(1_116_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	fn set_appendix() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_057_000 picoseconds.
		Weight::from_parts(1_108_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	fn clear_error() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_003_000 picoseconds.
		Weight::from_parts(1_075_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	fn descend_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_131_000 picoseconds.
		Weight::from_parts(1_205_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	fn clear_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 996_000 picoseconds.
		Weight::from_parts(1_025_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	/// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	/// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	fn report_error() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `145`
		//  Estimated: `3610`
		// Minimum execution time: 13_023_000 picoseconds.
		Weight::from_parts(13_350_000, 0)
			.saturating_add(Weight::from_parts(0, 3610))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: PolkadotXcm AssetTraps (r:1 w:1)
	/// Proof Skipped: PolkadotXcm AssetTraps (max_values: None, max_size: None, mode: Measured)
	fn claim_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `126`
		//  Estimated: `3591`
		// Minimum execution time: 8_533_000 picoseconds.
		Weight::from_parts(8_721_000, 0)
			.saturating_add(Weight::from_parts(0, 3591))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn trap() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_026_000 picoseconds.
		Weight::from_parts(1_063_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: PolkadotXcm VersionNotifyTargets (r:1 w:1)
	/// Proof Skipped: PolkadotXcm VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	/// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	/// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	fn subscribe_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `145`
		//  Estimated: `3610`
		// Minimum execution time: 15_382_000 picoseconds.
		Weight::from_parts(15_848_000, 0)
			.saturating_add(Weight::from_parts(0, 3610))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: PolkadotXcm VersionNotifyTargets (r:0 w:1)
	/// Proof Skipped: PolkadotXcm VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	fn unsubscribe_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_897_000 picoseconds.
		Weight::from_parts(1_944_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	/// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	/// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	fn initiate_reserve_withdraw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `145`
		//  Estimated: `3610`
		// Minimum execution time: 135_138_000 picoseconds.
		Weight::from_parts(136_692_000, 0)
			.saturating_add(Weight::from_parts(0, 3610))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	fn burn_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 46_164_000 picoseconds.
		Weight::from_parts(47_025_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	fn expect_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 12_272_000 picoseconds.
		Weight::from_parts(13_048_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	fn expect_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_019_000 picoseconds.
		Weight::from_parts(1_063_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	fn expect_error() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_011_000 picoseconds.
		Weight::from_parts(1_050_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	fn expect_transact_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 991_000 picoseconds.
		Weight::from_parts(1_040_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	/// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	/// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	fn query_pallet() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `145`
		//  Estimated: `3610`
		// Minimum execution time: 15_040_000 picoseconds.
		Weight::from_parts(15_351_000, 0)
			.saturating_add(Weight::from_parts(0, 3610))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	fn expect_pallet() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_058_000 picoseconds.
		Weight::from_parts(2_128_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	/// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	/// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	fn report_transact_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `145`
		//  Estimated: `3610`
		// Minimum execution time: 13_180_000 picoseconds.
		Weight::from_parts(13_582_000, 0)
			.saturating_add(Weight::from_parts(0, 3610))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	fn clear_transact_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 976_000 picoseconds.
		Weight::from_parts(1_057_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	fn set_topic() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_024_000 picoseconds.
		Weight::from_parts(1_070_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	fn clear_topic() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_009_000 picoseconds.
		Weight::from_parts(1_047_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: Benchmark Override (r:0 w:0)
	/// Proof Skipped: Benchmark Override (max_values: None, max_size: None, mode: Measured)
	fn exchange_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_446_744_073_709_551_000 picoseconds.
		Weight::from_parts(18_446_744_073_709_551_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: Benchmark Override (r:0 w:0)
	/// Proof Skipped: Benchmark Override (max_values: None, max_size: None, mode: Measured)
	fn universal_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_446_744_073_709_551_000 picoseconds.
		Weight::from_parts(18_446_744_073_709_551_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: Benchmark Override (r:0 w:0)
	/// Proof Skipped: Benchmark Override (max_values: None, max_size: None, mode: Measured)
	/// The range of component `x` is `[1, 1000]`.
	fn export_message() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_446_744_073_709_551_000 picoseconds.
		Weight::from_parts(18_446_744_073_709_551_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	fn set_fees_mode() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 987_000 picoseconds.
		Weight::from_parts(1_032_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: Benchmark Override (r:0 w:0)
	/// Proof Skipped: Benchmark Override (max_values: None, max_size: None, mode: Measured)
	fn lock_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_446_744_073_709_551_000 picoseconds.
		Weight::from_parts(18_446_744_073_709_551_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: Benchmark Override (r:0 w:0)
	/// Proof Skipped: Benchmark Override (max_values: None, max_size: None, mode: Measured)
	fn unlock_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_446_744_073_709_551_000 picoseconds.
		Weight::from_parts(18_446_744_073_709_551_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: Benchmark Override (r:0 w:0)
	/// Proof Skipped: Benchmark Override (max_values: None, max_size: None, mode: Measured)
	fn note_unlockable() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_446_744_073_709_551_000 picoseconds.
		Weight::from_parts(18_446_744_073_709_551_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: Benchmark Override (r:0 w:0)
	/// Proof Skipped: Benchmark Override (max_values: None, max_size: None, mode: Measured)
	fn request_unlock() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_446_744_073_709_551_000 picoseconds.
		Weight::from_parts(18_446_744_073_709_551_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	fn unpaid_execution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 980_000 picoseconds.
		Weight::from_parts(1_035_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
}
