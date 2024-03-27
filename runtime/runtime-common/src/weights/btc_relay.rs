
//! Autogenerated weights for btc_relay
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-07, STEPS: `50`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `interlay-rust-runner-2mz2v-jrrg4`, CPU: `AMD EPYC 7502P 32-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("interlay-dev"), DB CACHE: 1024

// Executed Command:
// target/release/interbtc-parachain
// benchmark
// pallet
// --pallet
// *
// --extrinsic
// *
// --chain
// interlay-dev
// --execution=wasm
// --wasm-execution=compiled
// --steps
// 50
// --repeat
// 10
// --output
// parachain/runtime/interlay/src/weights/
// --template
// .deploy/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for btc_relay using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);

impl<T: frame_system::Config> btc_relay::WeightInfo for WeightInfo<T> {

	/// Storage: BTCRelay BestBlock (r:1 w:1)
	/// Proof: BTCRelay BestBlock (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: BTCRelay DisableDifficultyCheck (r:1 w:0)
	/// Proof: BTCRelay DisableDifficultyCheck (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainCounter (r:1 w:1)
	/// Proof: BTCRelay ChainCounter (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay Chains (r:1 w:1)
	/// Proof: BTCRelay Chains (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: Security ActiveBlockCount (r:1 w:0)
	/// Proof: Security ActiveBlockCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainsHashes (r:0 w:1)
	/// Proof: BTCRelay ChainsHashes (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: BTCRelay StartBlockHeight (r:0 w:1)
	/// Proof: BTCRelay StartBlockHeight (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BestBlockHeight (r:0 w:1)
	/// Proof: BTCRelay BestBlockHeight (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainsIndex (r:0 w:1)
	/// Proof: BTCRelay ChainsIndex (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	/// Storage: BTCRelay BlockHeaders (r:0 w:1)
	/// Proof: BTCRelay BlockHeaders (max_values: None, max_size: Some(200), added: 2675, mode: MaxEncodedLen)
	fn initialize	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `403`
		//  Estimated: `3489`
		// Minimum execution time: 71_783_000 picoseconds.
		Weight::from_parts(74_009_000, 3489)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	/// Storage: BTCRelay ChainCounter (r:1 w:0)
	/// Proof: BTCRelay ChainCounter (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BlockHeaders (r:2 w:1)
	/// Proof: BTCRelay BlockHeaders (max_values: None, max_size: Some(200), added: 2675, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainsIndex (r:1 w:1)
	/// Proof: BTCRelay ChainsIndex (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	/// Storage: BTCRelay DisableDifficultyCheck (r:1 w:0)
	/// Proof: BTCRelay DisableDifficultyCheck (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainsHashes (r:1 w:1)
	/// Proof: BTCRelay ChainsHashes (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Security ActiveBlockCount (r:1 w:0)
	/// Proof: Security ActiveBlockCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BestBlock (r:0 w:1)
	/// Proof: BTCRelay BestBlock (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: BTCRelay BestBlockHeight (r:0 w:1)
	/// Proof: BTCRelay BestBlockHeight (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn store_block_header	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `874`
		//  Estimated: `6340`
		// Minimum execution time: 88_808_000 picoseconds.
		Weight::from_parts(90_482_000, 6340)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}

	fn update_store_utxo	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `874`
		//  Estimated: `6340`
		// Minimum execution time: 88_808_000 picoseconds.
		Weight::from_parts(90_482_000, 6340)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}

	fn store_monitor_utxo	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `874`
		//  Estimated: `6340`
		// Minimum execution time: 88_808_000 picoseconds.
		Weight::from_parts(90_482_000, 6340)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: BTCRelay ChainCounter (r:1 w:1)
	/// Proof: BTCRelay ChainCounter (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BlockHeaders (r:2 w:1)
	/// Proof: BTCRelay BlockHeaders (max_values: None, max_size: Some(200), added: 2675, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainsIndex (r:6 w:1)
	/// Proof: BTCRelay ChainsIndex (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	/// Storage: BTCRelay DisableDifficultyCheck (r:1 w:0)
	/// Proof: BTCRelay DisableDifficultyCheck (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: BTCRelay Chains (r:7 w:1)
	/// Proof: BTCRelay Chains (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: Security ActiveBlockCount (r:1 w:0)
	/// Proof: Security ActiveBlockCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BestBlock (r:1 w:0)
	/// Proof: BTCRelay BestBlock (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainsHashes (r:0 w:1)
	/// Proof: BTCRelay ChainsHashes (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// The range of component `f` is `[1, 6]`.
	fn store_block_header_new_fork_sorted	(f: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `806 + f * (101 ±0)`
		//  Estimated: `6340 + f * (2507 ±0)`
		// Minimum execution time: 101_073_000 picoseconds.
		Weight::from_parts(95_176_412, 6340)
			// Standard Error: 452_083
			.saturating_add(Weight::from_parts(11_888_980, 0).saturating_mul(f.into()))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(f.into())))
			.saturating_add(T::DbWeight::get().writes(5_u64))
			.saturating_add(Weight::from_parts(0, 2507).saturating_mul(f.into()))
	}
	/// Storage: BTCRelay ChainCounter (r:1 w:1)
	/// Proof: BTCRelay ChainCounter (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BlockHeaders (r:2 w:1)
	/// Proof: BTCRelay BlockHeaders (max_values: None, max_size: Some(200), added: 2675, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainsIndex (r:2 w:1)
	/// Proof: BTCRelay ChainsIndex (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	/// Storage: BTCRelay DisableDifficultyCheck (r:1 w:0)
	/// Proof: BTCRelay DisableDifficultyCheck (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: BTCRelay Chains (r:7 w:6)
	/// Proof: BTCRelay Chains (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: Security ActiveBlockCount (r:1 w:0)
	/// Proof: Security ActiveBlockCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BestBlock (r:1 w:0)
	/// Proof: BTCRelay BestBlock (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainsHashes (r:0 w:1)
	/// Proof: BTCRelay ChainsHashes (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// The range of component `f` is `[1, 6]`.
	fn store_block_header_new_fork_unsorted	(f: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `842 + f * (99 ±0)`
		//  Estimated: `6340 + f * (2499 ±0)`
		// Minimum execution time: 97_675_000 picoseconds.
		Weight::from_parts(89_104_041, 6340)
			// Standard Error: 196_333
			.saturating_add(Weight::from_parts(14_328_939, 0).saturating_mul(f.into()))
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(f.into())))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(f.into())))
			.saturating_add(Weight::from_parts(0, 2499).saturating_mul(f.into()))
	}
	/// Storage: BTCRelay ChainCounter (r:1 w:0)
	/// Proof: BTCRelay ChainCounter (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BlockHeaders (r:20 w:18)
	/// Proof: BTCRelay BlockHeaders (max_values: None, max_size: Some(200), added: 2675, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainsIndex (r:3 w:2)
	/// Proof: BTCRelay ChainsIndex (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	/// Storage: BTCRelay DisableDifficultyCheck (r:1 w:0)
	/// Proof: BTCRelay DisableDifficultyCheck (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainsHashes (r:13 w:24)
	/// Proof: BTCRelay ChainsHashes (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Security ActiveBlockCount (r:1 w:0)
	/// Proof: Security ActiveBlockCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay Chains (r:6 w:0)
	/// Proof: BTCRelay Chains (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: BTCRelay StableBitcoinConfirmations (r:1 w:0)
	/// Proof: BTCRelay StableBitcoinConfirmations (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BestBlock (r:0 w:1)
	/// Proof: BTCRelay BestBlock (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: BTCRelay BestBlockHeight (r:0 w:1)
	/// Proof: BTCRelay BestBlockHeight (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// The range of component `f` is `[3, 6]`.
	fn store_block_header_reorganize_chains	(f: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4741 + f * (240 ±0)`
		//  Estimated: `54490 + f * (1340 ±45)`
		// Minimum execution time: 506_604_000 picoseconds.
		Weight::from_parts(465_095_071, 54490)
			// Standard Error: 430_820
			.saturating_add(Weight::from_parts(18_597_675, 0).saturating_mul(f.into()))
			.saturating_add(T::DbWeight::get().reads(42_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(f.into())))
			.saturating_add(T::DbWeight::get().writes(46_u64))
			.saturating_add(Weight::from_parts(0, 1340).saturating_mul(f.into()))
	}
}