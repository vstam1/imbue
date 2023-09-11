#![cfg_attr(not(feature = "std"), no_std)]

//FELIX REVIEW: Eventually it will be nice to have a short introduction here explaining what this pallet does and the
// avaliable methods etc.

// 1: Raise dispute using DisputeRaiser from pallet_proposals
// - It takes the raiser_id,project_id as dispute_key, list of jury(randomly selected upto 7 to 9 count), reason, fund_account
// - Exisiting implementation looks good, need to update the votes while inserting the new dispute

// 2: Vote on dispute. 
// Get the vote as single yes or no and divide based on the number of the voters
// Need to come up with a way to change the votes that might require the storing the votes of each voter 

// 3: finalise it in the on_initialize hook.
// Signal that this is ready for continuation. pallet-refund/pallet-proposals.
// Refund, Everythings ok.

// 4: an extrinsic is called claim_back(parameter: who, where.)

pub use pallet::*;
//pub mod impls;
pub mod traits;
pub mod weights;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use codec::{FullCodec, FullEncode};
    use frame_support::{
        dispatch::fmt::Debug, pallet_prelude::*, weights::Weight, BoundedBTreeMap,
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{AtLeast32BitUnsigned, Saturating};
    use traits::DisputeHooks;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    pub(crate) type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

    #[pallet::config]
    //FELIX Review: Comment each of the config items so we know exactly what they are doing.
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// The weights generated by the benchmarks.
        type WeightInfo: WeightInfoT;
        //Felix teachings --> to keep an associated type into storage it needs to implement certain traits
        type DisputeKey: AtLeast32BitUnsigned
            + FullEncode
            + FullCodec
            + MaxEncodedLen
            + TypeInfo
            + Debug
            + Copy;
        /// This is the max length for specifying the reason while raising the dispute
        type MaxReasonLength: Get<u32>;
        /// This is number of juries that can be assigned to a given dispute
        type MaxJurySize: Get<u32>;
        /// The amount of time a dispute takes to finalise.
        type VotingTimeLimit: Get<<Self as frame_system::Config>::BlockNumber>;

        type ForceOrigin: EnsureOrigin<Self::RuntimeOrigin>;
    }

    /// Used to store the disputes that is being raised, given the dispute key it returns the Dispute
    /// Key: DisputeKey
    /// Value: Dispute<T>
    #[pallet::storage]
    #[pallet::getter(fn disputes)]
    pub type Disputes<T: Config> =
        StorageMap<_, Blake2_128Concat, T::DisputeKey, Dispute<T>, OptionQuery>;

    /// Stores the dispute keys that will finalise on a given block.
    /// Key: BlockNumber
    /// Value: Vec<DisputeKey>
    #[pallet::storage]
    pub type DisputesFinaliseOn<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        <T as frame_system::Config>::BlockNumber,
        BoundedVec<T::DisputeKey, ConstU32<1000>>,
        ValueQuery,
    >;

    #[pallet::event]
    // FELIX REVIEW: the below generate_deposit line is depricated in the 9.0.43 so you can remove it completely.
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        //This event is emitted whenever a dispute has been successfully raised
        DisputeRaised {
            dispute_key: T::DisputeKey,
        },
        // This event is emitted whenever there has been a voting successfully happened for a given dispute by
        // the authorized jury member
        DisputeVotedOn {
            who: AccountIdOf<T>,
        },
        /// A dispute has been completed.
        DisputeCompleted,
        //This event is emitted when the dispute is being cancelled
        DisputeCancelled,
    }

    #[pallet::error]
    pub enum Error<T> {
        /// This error is thrown whenever the dispute key passed doesn't correspond to any dispute.
        DisputeDoesNotExist,
        /// Dispute already exists for this dispute id.
        DisputeAlreadyExists,
        // This account is not part of the specified jury.
        InvalidJuryAccount,
        /// There have been too many disputes on this block. Try next block.
        TooManyDisputesThisBlock,
        /// The total vote must equal on for a refund vote.
        TotalVoteMustEqualOne,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
            <Weight as Default>::default()
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        // FELIX: BENCHMARK
        #[pallet::weight(<T as Config>::WeightInfo::vote_on_dispute())]
        pub fn vote_on_dispute(
            origin: OriginFor<T>,
            dispute_key: T::DisputeKey,
            vote: Vote,
        ) -> DispatchResult {
            let whoz = ensure_signed(origin)?;
            // If refund vote then make sure each side adds to 100%
            if let Vote::Refund(refund_vote) = vote {
                ensure!(
                    refund.0 + refund.1 == <Percent as One>::one(),
                    Error::<T>::TotalVoteMustEqualOne
                );
            };

            Disputes::<T>::try_mutate(dispute_key, |dispute| {
                if let Some(d) = dispute {
                    ensure!(
                        d.jury.iter().any(|e| e == &who),
                        Error::<T>::InvalidJuryAccount
                    );
                    d.votes
                        .try_insert(&who, vote)
                        .map_err(|_| Error::<T>::TooManyDisputeVotes)?;
                    Ok::<(), DispatchError>(())
                } else {
                    Err(Error::<T>::DisputeDoesNotExist)
                }
            })?;

            Self::deposit_event(Event::<T>::DisputeVotedOn { who });
            Ok(().into())
        }

        #[pallet::call_index(1)]
        // FELIX REVIEW: Benchmarks
        #[pallet::weight(<T as Config>::WeightInfo::force_cancel_dispute())]
        pub fn force_cancel_dispute(
            origin: OriginFor<T>,
            dispute_key: T::DisputeKey,
            is_yay: bool,
        ) -> DispatchResult {
            //ensuring the cancelling authority
            <T as Config>::ForceOrigin::ensure_origin(origin)?;
            //calling the on_dispute cancel whenever the force cancel method is called
            let res = T::DisputeHooks::on_dispute_cancel(dispute_key);
            Ok(().into())
        }
    }

    #[derive(Encode, Decode, PartialEq, Eq, Clone, Debug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Dispute<T: Config> {
        /// Who this was raised by.
        pub raised_by: AccountIdOf<T>,
        /// The votes of each jury.
        pub votes: BoundedBTreeMap<AccountIdOf<T>, Vote, T::MaxJurySize>,
        /// The reason the dispute was raised.
        pub reason: BoundedVec<u8, <T as Config>::MaxReasonLength>,
        /// The party responsible for the vote.
        pub jury: BoundedVec<AccountIdOf<T>, <T as Config>::MaxJurySize>,
    }

    impl<T: Config> Dispute<T> {
        // Create a new dispute and setup state so that pallet will operate as intended.
        pub fn new(
            dispute_key: T::DisputeKey,
            raised_by: AccountIdOf<T>,
            reason: BoundedVec<u8, T::MaxReasonLength>,
            jury: BoundedVec<AccountIdOf<T>, T::MaxJurySize>,
        ) -> Result<(), DispatchError> {
            let dispute = Self {
                raised_by,
                reason,
                jury,
                votes: Default::default(),
            };
            let final_block = frame_system::Pallet::<T>::block_number();

            Disputes::<T>::insert(dispute_key, dispute);
            DisputesFinaliseOn::<T>::try_mutate(
                final_block.saturating_add(T::VotingTimeLimit::get()),
                |b_vec| {
                    b_vec
                        .try_push(dispute_key)
                        .map_err(|_| Error::<T>::TooManyDisputesThisBlock)?;

                    Ok::<(), DispatchError>(())
                },
            )?;

            crate::Pallet::<T>::deposit_event(Event::<T>::DisputeRaised { dispute_key });
            Ok(())
        }

        pub fn remove(key: T::DisputeKey) -> Result<(), DispatchError> {
            Disputes::<T>::insert(dispute_key, dispute);
            // Dispute,
            // DisputeFInaliseOm
            Ok(())
        }
    }

    #[derive(Encode, Decode, PartialEq, Eq, Clone, Debug, TypeInfo, MaxEncodedLen)]
    pub enum Vote {
        Refund(RefundVote),
        Continue,
        Abstain,
    }

    // A dispute vote contains what an account believes the outcome should be.
    #[derive(Encode, Decode, PartialEq, Eq, Clone, Copy, Debug, TypeInfo, MaxEncodedLen)]
    pub struct RefundVote {
        pub to_initiator: u32,
        pub to_refund: u32,
    }

    enum Outcome {
        Refund,
        Continue,
        Slash,
    }

    pub trait WeightInfoT {
        fn vote_on_dispute() -> Weight;
        fn force_cancel_dispute() -> Weight;
        fn raise_dispute() -> Weight;
        fn on_dispute_complete() -> Weight;
        fn on_dispute_cancel() -> Weight;
    }
}
