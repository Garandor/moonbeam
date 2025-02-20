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

//! Autogenerated weights for xcm_transactor
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-08-31, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/moonbeam
// benchmark
// pallet
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// xcm_transactor
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --template=./benchmarking/frame-weight-template.hbs
// --record-proof<
// --json-file
// raw.json
// --output
// weights.rs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for xcm_transactor.
pub trait WeightInfo {
	#[rustfmt::skip]
	fn register() -> Weight;
	#[rustfmt::skip]
	fn deregister() -> Weight;
	#[rustfmt::skip]
	fn set_transact_info() -> Weight;
	#[rustfmt::skip]
	fn remove_transact_info() -> Weight;
	#[rustfmt::skip]
	fn set_fee_per_second() -> Weight;
	#[rustfmt::skip]
	fn transact_through_derivative() -> Weight;
	#[rustfmt::skip]
	fn transact_through_sovereign() -> Weight;
	#[rustfmt::skip]
	fn transact_through_signed() -> Weight;
}

/// Weights for xcm_transactor using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: XcmTransactor IndexToAccount (r:1 w:1)
	#[rustfmt::skip]
	fn register() -> Weight {
		(17_998_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: XcmTransactor IndexToAccount (r:0 w:1)
	#[rustfmt::skip]
	fn deregister() -> Weight {
		(14_817_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: XcmTransactor TransactInfoWithWeightLimit (r:0 w:1)
	#[rustfmt::skip]
	fn set_transact_info() -> Weight {
		(17_470_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: XcmTransactor TransactInfoWithWeightLimit (r:0 w:1)
	#[rustfmt::skip]
	fn remove_transact_info() -> Weight {
		(16_694_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: XcmTransactor DestinationFeePerSecond (r:0 w:1)
	#[rustfmt::skip]
	fn set_fee_per_second() -> Weight {
		(16_640_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: AssetManager AssetIdType (r:1 w:0)
	// Storage: XcmTransactor IndexToAccount (r:1 w:0)
	// Storage: XcmTransactor TransactInfoWithWeightLimit (r:1 w:0)
	// Storage: XcmTransactor DestinationAssetFeePerSecond (r:1 w:0)
	// Storage: AssetManager AssetTypeId (r:1 w:0)
	// Storage: Assets Asset (r:1 w:0)
	#[rustfmt::skip]
	fn transact_through_derivative() -> Weight {
		(30_613_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
	}
	// Storage: AssetManager AssetIdType (r:1 w:0)
	// Storage: XcmTransactor TransactInfoWithWeightLimit (r:1 w:0)
	// Storage: XcmTransactor DestinationAssetFeePerSecond (r:1 w:0)
	// Storage: AssetManager AssetTypeId (r:1 w:0)
	// Storage: Assets Asset (r:1 w:0)
	#[rustfmt::skip]
	fn transact_through_sovereign() -> Weight {
		(26_933_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
	}
	// Storage: AssetManager AssetIdType (r:1 w:0)
	// Storage: XcmTransactor TransactInfoWithWeightLimit (r:1 w:0)
	// Storage: XcmTransactor DestinationAssetFeePerSecond (r:1 w:0)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	#[rustfmt::skip]
	fn transact_through_signed() -> Weight {
		(65_476_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: XcmTransactor IndexToAccount (r:1 w:1)
	#[rustfmt::skip]
	fn register() -> Weight {
		(17_998_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: XcmTransactor IndexToAccount (r:0 w:1)
	#[rustfmt::skip]
	fn deregister() -> Weight {
		(14_817_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: XcmTransactor TransactInfoWithWeightLimit (r:0 w:1)
	#[rustfmt::skip]
	fn set_transact_info() -> Weight {
		(17_470_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: XcmTransactor TransactInfoWithWeightLimit (r:0 w:1)
	#[rustfmt::skip]
	fn remove_transact_info() -> Weight {
		(16_694_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: XcmTransactor DestinationFeePerSecond (r:0 w:1)
	#[rustfmt::skip]
	fn set_fee_per_second() -> Weight {
		(16_640_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: AssetManager AssetIdType (r:1 w:0)
	// Storage: XcmTransactor IndexToAccount (r:1 w:0)
	// Storage: XcmTransactor TransactInfoWithWeightLimit (r:1 w:0)
	// Storage: XcmTransactor DestinationAssetFeePerSecond (r:1 w:0)
	// Storage: AssetManager AssetTypeId (r:1 w:0)
	// Storage: Assets Asset (r:1 w:0)
	#[rustfmt::skip]
	fn transact_through_derivative() -> Weight {
		(30_613_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
	}
	// Storage: AssetManager AssetIdType (r:1 w:0)
	// Storage: XcmTransactor TransactInfoWithWeightLimit (r:1 w:0)
	// Storage: XcmTransactor DestinationAssetFeePerSecond (r:1 w:0)
	// Storage: AssetManager AssetTypeId (r:1 w:0)
	// Storage: Assets Asset (r:1 w:0)
	#[rustfmt::skip]
	fn transact_through_sovereign() -> Weight {
		(26_933_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
	}
	// Storage: AssetManager AssetIdType (r:1 w:0)
	// Storage: XcmTransactor TransactInfoWithWeightLimit (r:1 w:0)
	// Storage: XcmTransactor DestinationAssetFeePerSecond (r:1 w:0)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	#[rustfmt::skip]
	fn transact_through_signed() -> Weight {
		(65_476_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
}
