
//! Autogenerated weights for `pallet_proposals`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-11-07, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Justs-MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("local")`, DB CACHE: 1024

// Executed Command:
// ./target/release/imbue
// benchmark
// pallet
// --chain
// local
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet-proposals
// --extrinsic
// *
// --output
// ./pallets/proposals/src/weights.rs
// --steps
// 50
// --repeat
// 20
// --heap-pages
// 4096

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_proposals`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> crate::WeightInfoT for WeightInfo<T> {
	/// Storage: `ImbueProposals::Projects` (r:1 w:0)
	/// Proof: `ImbueProposals::Projects` (`max_values`: None, `max_size`: Some(2862), added: 5337, mode: `MaxEncodedLen`)
	/// Storage: `ImbueProposals::RoundsExpiring` (r:1 w:1)
	/// Proof: `ImbueProposals::RoundsExpiring` (`max_values`: None, `max_size`: Some(111), added: 2586, mode: `MaxEncodedLen`)
	/// Storage: `ImbueProposals::IndividualVoteStore` (r:1 w:1)
	/// Proof: `ImbueProposals::IndividualVoteStore` (`max_values`: None, `max_size`: Some(16571), added: 19046, mode: `MaxEncodedLen`)
	/// Storage: `ImbueProposals::MilestoneVotes` (r:1 w:1)
	/// Proof: `ImbueProposals::MilestoneVotes` (`max_values`: None, `max_size`: Some(375), added: 2850, mode: `MaxEncodedLen`)
	/// Storage: `ImbueProposals::Rounds` (r:0 w:1)
	/// Proof: `ImbueProposals::Rounds` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	fn submit_milestone() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `496`
		//  Estimated: `20036`
		// Minimum execution time: 30_000_000 picoseconds.
		Weight::from_parts(31_000_000, 0)
			.saturating_add(Weight::from_parts(0, 20036))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `ImbueProposals::Projects` (r:1 w:1)
	/// Proof: `ImbueProposals::Projects` (`max_values`: None, `max_size`: Some(2862), added: 5337, mode: `MaxEncodedLen`)
	/// Storage: `ImbueProposals::Rounds` (r:1 w:1)
	/// Proof: `ImbueProposals::Rounds` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	/// Storage: `ImbueProposals::IndividualVoteStore` (r:1 w:1)
	/// Proof: `ImbueProposals::IndividualVoteStore` (`max_values`: None, `max_size`: Some(16571), added: 19046, mode: `MaxEncodedLen`)
	/// Storage: `ImbueProposals::MilestoneVotes` (r:1 w:1)
	/// Proof: `ImbueProposals::MilestoneVotes` (`max_values`: None, `max_size`: Some(375), added: 2850, mode: `MaxEncodedLen`)
	/// Storage: `ImbueProposals::RoundsExpiring` (r:0 w:1)
	/// Proof: `ImbueProposals::RoundsExpiring` (`max_values`: None, `max_size`: Some(111), added: 2586, mode: `MaxEncodedLen`)
	fn vote_on_milestone() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `639`
		//  Estimated: `20036`
		// Minimum execution time: 43_000_000 picoseconds.
		Weight::from_parts(46_000_000, 0)
			.saturating_add(Weight::from_parts(0, 20036))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `ImbueProposals::Projects` (r:1 w:1)
	/// Proof: `ImbueProposals::Projects` (`max_values`: None, `max_size`: Some(2862), added: 5337, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:3)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Deposits::CurrentDeposits` (r:1 w:1)
	/// Proof: `Deposits::CurrentDeposits` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	/// Storage: `ImbueProposals::CompletedProjects` (r:1 w:1)
	/// Proof: `ImbueProposals::CompletedProjects` (`max_values`: None, `max_size`: Some(262184), added: 264659, mode: `MaxEncodedLen`)
	fn withdraw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `997`
		//  Estimated: `265649`
		// Minimum execution time: 130_000_000 picoseconds.
		Weight::from_parts(144_000_000, 0)
			.saturating_add(Weight::from_parts(0, 265649))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `ImbueProposals::Projects` (r:1 w:0)
	/// Proof: `ImbueProposals::Projects` (`max_values`: None, `max_size`: Some(2862), added: 5337, mode: `MaxEncodedLen`)
	/// Storage: `ImbueProposals::NoConfidenceVotes` (r:1 w:1)
	/// Proof: `ImbueProposals::NoConfidenceVotes` (`max_values`: None, `max_size`: Some(37), added: 2512, mode: `MaxEncodedLen`)
	/// Storage: `ImbueProposals::RoundsExpiring` (r:1 w:1)
	/// Proof: `ImbueProposals::RoundsExpiring` (`max_values`: None, `max_size`: Some(111), added: 2586, mode: `MaxEncodedLen`)
	/// Storage: `ImbueProposals::UserHasVoted` (r:1 w:1)
	/// Proof: `ImbueProposals::UserHasVoted` (`max_values`: None, `max_size`: Some(1667), added: 4142, mode: `MaxEncodedLen`)
	/// Storage: `ImbueProposals::Rounds` (r:0 w:1)
	/// Proof: `ImbueProposals::Rounds` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	fn raise_vote_of_no_confidence() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `475`
		//  Estimated: `6327`
		// Minimum execution time: 24_000_000 picoseconds.
		Weight::from_parts(26_000_000, 0)
			.saturating_add(Weight::from_parts(0, 6327))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `ImbueProposals::Rounds` (r:1 w:0)
	/// Proof: `ImbueProposals::Rounds` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	/// Storage: `ImbueProposals::Projects` (r:1 w:0)
	/// Proof: `ImbueProposals::Projects` (`max_values`: None, `max_size`: Some(2862), added: 5337, mode: `MaxEncodedLen`)
	/// Storage: `ImbueProposals::NoConfidenceVotes` (r:1 w:1)
	/// Proof: `ImbueProposals::NoConfidenceVotes` (`max_values`: None, `max_size`: Some(37), added: 2512, mode: `MaxEncodedLen`)
	/// Storage: `ImbueProposals::UserHasVoted` (r:1 w:1)
	/// Proof: `ImbueProposals::UserHasVoted` (`max_values`: None, `max_size`: Some(1667), added: 4142, mode: `MaxEncodedLen`)
	fn vote_on_no_confidence_round() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `700`
		//  Estimated: `6327`
		// Minimum execution time: 24_000_000 picoseconds.
		Weight::from_parts(25_000_000, 0)
			.saturating_add(Weight::from_parts(0, 6327))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `ImbueProposals::RoundsExpiring` (r:1 w:1)
	/// Proof: `ImbueProposals::RoundsExpiring` (`max_values`: None, `max_size`: Some(111), added: 2586, mode: `MaxEncodedLen`)
	/// Storage: `ImbueProposals::MilestoneVotes` (r:1 w:1)
	/// Proof: `ImbueProposals::MilestoneVotes` (`max_values`: None, `max_size`: Some(375), added: 2850, mode: `MaxEncodedLen`)
	/// Storage: `ImbueProposals::IndividualVoteStore` (r:1 w:1)
	/// Proof: `ImbueProposals::IndividualVoteStore` (`max_values`: None, `max_size`: Some(16571), added: 19046, mode: `MaxEncodedLen`)
	/// Storage: `ImbueProposals::Rounds` (r:0 w:1)
	/// Proof: `ImbueProposals::Rounds` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	fn on_initialize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `97`
		//  Estimated: `20036`
		// Minimum execution time: 10_000_000 picoseconds.
		Weight::from_parts(11_000_000, 0)
			.saturating_add(Weight::from_parts(0, 20036))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
