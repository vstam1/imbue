
//! Autogenerated weights for `pallet_briefs`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-11-07, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Justs-MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("local")`, DB CACHE: 1024

// Executed Command:
// ./target/debug/imbue
// benchmark
// pallet
// --chain
// local
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet-briefs
// --extrinsic
// *
// --output
// ./pallets/briefs/src/weights.rs
// --steps
// 50
// --repeat
// 20

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_briefs`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> crate::WeightInfoT for WeightInfo<T> {
	/// Storage: `ImbueBriefs::Briefs` (r:1 w:1)
	/// Proof: `ImbueBriefs::Briefs` (`max_values`: None, `max_size`: Some(3366), added: 5841, mode: `MaxEncodedLen`)
	/// Storage: `Deposits::TicketId` (r:1 w:1)
	/// Proof: `Deposits::TicketId` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `ImbueBriefs::BriefContributions` (r:1 w:1)
	/// Proof: `ImbueBriefs::BriefContributions` (`max_values`: None, `max_size`: Some(5250), added: 7725, mode: `MaxEncodedLen`)
	/// Storage: `ImbueBriefs::CounterForBriefs` (r:1 w:1)
	/// Proof: `ImbueBriefs::CounterForBriefs` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Deposits::CurrentDeposits` (r:0 w:1)
	/// Proof: `Deposits::CurrentDeposits` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	fn create_brief() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `434`
		//  Estimated: `8715`
		// Minimum execution time: 678_000_000 picoseconds.
		Weight::from_parts(702_000_000, 0)
			.saturating_add(Weight::from_parts(0, 8715))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `ImbueBriefs::Briefs` (r:1 w:0)
	/// Proof: `ImbueBriefs::Briefs` (`max_values`: None, `max_size`: Some(3366), added: 5841, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `ImbueBriefs::BriefContributions` (r:1 w:1)
	/// Proof: `ImbueBriefs::BriefContributions` (`max_values`: None, `max_size`: Some(5250), added: 7725, mode: `MaxEncodedLen`)
	fn contribute_to_brief() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3860`
		//  Estimated: `8715`
		// Minimum execution time: 373_000_000 picoseconds.
		Weight::from_parts(403_000_000, 0)
			.saturating_add(Weight::from_parts(0, 8715))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `ImbueBriefs::Briefs` (r:1 w:1)
	/// Proof: `ImbueBriefs::Briefs` (`max_values`: None, `max_size`: Some(3366), added: 5841, mode: `MaxEncodedLen`)
	/// Storage: `ImbueBriefs::BriefContributions` (r:1 w:1)
	/// Proof: `ImbueBriefs::BriefContributions` (`max_values`: None, `max_size`: Some(5250), added: 7725, mode: `MaxEncodedLen`)
	/// Storage: `Deposits::CurrentDeposits` (r:1 w:2)
	/// Proof: `Deposits::CurrentDeposits` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:3)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `ImbueProposals::ProjectCount` (r:1 w:1)
	/// Proof: `ImbueProposals::ProjectCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Deposits::TicketId` (r:1 w:1)
	/// Proof: `Deposits::TicketId` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `ImbueBriefs::CounterForBriefs` (r:1 w:1)
	/// Proof: `ImbueBriefs::CounterForBriefs` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `ImbueProposals::Projects` (r:0 w:1)
	/// Proof: `ImbueProposals::Projects` (`max_values`: None, `max_size`: Some(260823), added: 263298, mode: `MaxEncodedLen`)
	/// Storage: `ImbueProposals::IndividualVoteStore` (r:0 w:1)
	/// Proof: `ImbueProposals::IndividualVoteStore` (`max_values`: None, `max_size`: Some(8250321), added: 8252796, mode: `MaxEncodedLen`)
	fn commence_work() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4520`
		//  Estimated: `8799`
		// Minimum execution time: 1_763_000_000 picoseconds.
		Weight::from_parts(1_805_000_000, 0)
			.saturating_add(Weight::from_parts(0, 8799))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(12))
	}
	/// Storage: `ImbueBriefs::Briefs` (r:1 w:1)
	/// Proof: `ImbueBriefs::Briefs` (`max_values`: None, `max_size`: Some(3366), added: 5841, mode: `MaxEncodedLen`)
	/// Storage: `Deposits::CurrentDeposits` (r:1 w:1)
	/// Proof: `Deposits::CurrentDeposits` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `ImbueBriefs::BriefContributions` (r:1 w:1)
	/// Proof: `ImbueBriefs::BriefContributions` (`max_values`: None, `max_size`: Some(5250), added: 7725, mode: `MaxEncodedLen`)
	/// Storage: `ImbueBriefs::CounterForBriefs` (r:1 w:1)
	/// Proof: `ImbueBriefs::CounterForBriefs` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn cancel_brief() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4094`
		//  Estimated: `8715`
		// Minimum execution time: 727_000_000 picoseconds.
		Weight::from_parts(738_000_000, 0)
			.saturating_add(Weight::from_parts(0, 8715))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
}
