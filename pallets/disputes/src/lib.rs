#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub mod traits;
pub mod weights;
pub use weights::*;
use sp_runtime::{DispatchError, traits::AtLeast32BitUnsigned};

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use codec::{FullEncode, FullCodec};
use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use traits::DisputeRaiser;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	pub(crate) type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
		//Felix teachings --> to keep an associated type into storage it needs to implement certain traits
		type DisputeKey: AtLeast32BitUnsigned + FullEncode + FullCodec + MaxEncodedLen + TypeInfo;
		type MaxReasonLength: Get<u32>;
		type MaxJurySize: Get<u32>;
		type DisputeHooks: traits::DisputeHooks;
		type TimeLimit: Get<<Self as frame_system::Config>::BlockNumber>;
		type CancellingAuthority: EnsureOrigin<Self::RuntimeOrigin>;
	}

	#[pallet::storage]
	#[pallet::getter(fn disputes)]
    pub type Disputes<T: Config> =
        StorageMap<_, Blake2_128Concat, T::DisputeKey, Dispute<T>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		//This event is emitted when a dispute is being raised
		DisputeRaised {who: AccountIdOf<T>},
		//This event is emitted from the vote_on_dispute extrinsics 
		//to notify what dispute is being voted on
		DisputeVotedOn {who: AccountIdOf<T>},
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
		//This error is thrown whenever the dispute key passed doesn't correspond to any dispute
		DisputeDoesNotExist,
		DisputeAlreadyExists,
	}


	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn vote_on_dispute(
			origin: OriginFor<T>,
			dispute_key: T::DisputeKey,
			is_yay: bool,
		) -> DispatchResult {
			// get dispute struct
			// ensure caller is part of the jury
			// mutate vote accordingly.
			
			// FELIX_REVIEW: use mutate instead and throw error if not found.
			// TODO: only the choosen jury can vote on the disputes?
			//Disputes::<T>::try_mutate(|dispute_key|{
			//	let

			// Ok::<(), DispatchError>();
			//})?;

			let mut dispute = Disputes::<T>::get(dispute_key).ok_or(Error::<T>::DisputeDoesNotExist)?;
			let mut vote = dispute.votes;
			if is_yay{
				vote.yay +=1;
			}
			else {
				vote.nay +=1;
			}

			// emit event
			//TODO will update the mutated votes into the dispute correct?
			Ok(().into())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn force_cancel_dispute(
			origin: OriginFor<T>,
			dispute_key: T::DisputeKey,
			is_yay: bool,
		) -> DispatchResult {
			// check origin is of authority origin using associated type
			// remove the dispute.
			// call DisputeHooks::on_dispute_cancelled()
			// emit event
			Ok(().into())
		}
	}

	impl<T: Config> DisputeRaiser<AccountIdOf<T>> for Pallet<T> {
		type DisputeKey = T::DisputeKey;
		
		fn raise_dispute(
			dispute_key: Self::DisputeKey,
			raised_by: AccountIdOf<T>,
			fund_account: AccountIdOf<T>,
			reason: Vec<u8>,
			project_id: u32,
			jury: Vec<AccountIdOf<T>>,
		) -> Result<(), DispatchError> {
			// creating the struct with the passed information and initializing vote as 0 initially
			let dispute:Dispute<T> = Dispute{
				raised_by,
				fund_account,
				votes: Vote{
					yay: 0,
					nay: 0,
				},
				reason,
				jury,
			};

			ensure!(!Disputes::<T>::contains_key(dispute_key), Error::<T>::DisputeAlreadyExists);

			//storing the raised dispute inside the disputes storage 
			Disputes::<T>::insert(dispute_key, dispute);

			// Raise Event 
			//TODO need to check if want to add more information while raising a dispute like returning the whole dispute struct?
			Self::deposit_event(Event::DisputeRaised(
				raised_by
            ));

			Ok(())
		}
	}

	#[derive(Encode, Decode, PartialEq, Eq, Clone, Debug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	struct Dispute<T: Config> {
		raised_by: AccountIdOf<T>,
		fund_account: AccountIdOf<T>,
		// TODO: Add balance type
		// currencyid: CurrencyId
		//fund_amount: BalanceOf<T>
		votes: Vote,
		reason: BoundedVec<u8, <T as Config>::MaxReasonLength>,
		jury: BoundedVec<AccountIdOf<T>, <T as Config>::MaxJurySize>
	}

	#[derive(Encode, Decode, PartialEq, Eq, Clone, Debug, TypeInfo, MaxEncodedLen)]
   pub struct Vote {
    yay: u32 ,
    nay: u32,
}

}