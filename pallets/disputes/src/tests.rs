use crate::traits::*;
use crate::*;
use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, once_cell::sync::Lazy, BoundedBTreeMap, traits::Hooks};
use orml_traits::{MultiCurrency, MultiReservableCurrency};
use sp_runtime::{traits::BadOrigin, DispatchError, Saturating, BoundedVec};
use sp_arithmetic::traits::One;

pub fn run_to_block<T: Config>(n: T::BlockNumber)
    where T::BlockNumber: Into<u64>
{
    loop {
        let mut block: T::BlockNumber = frame_system::Pallet::<T>::block_number();
        if block >= n {
            break;
        }
        block = block.saturating_add(<T::BlockNumber as One>::one());
        frame_system::Pallet::<T>::set_block_number(block);
        frame_system::Pallet::<T>::on_initialize(block);
    }
}


///testing of the raised dispute is being inserted into the storage
#[test]
fn test_create_a_dispute_get_and_check_insertion_within_the_storage() {
    const DISPUTE_KEY: u32 = 10;
    new_test_ext().execute_with(|| {
        Dispute::<Test>::new(DISPUTE_KEY, **&ALICE, BoundedVec::default(), BoundedVec::default()).expect("TODO: panic message");
        assert!(PalletDisputes::disputes(DISPUTE_KEY).is_some());
        assert_eq!(1, PalletDisputes::disputes(DISPUTE_KEY).iter().count());
    });
}

///testing to insert the dispute with same dispute key twice and it should throw the error saying the dispute already exists
#[test]
fn test_trying_to_insert_create_a_dispute_with_existing_key() {
    new_test_ext().execute_with(|| {
        Dispute::<Test>::new(1, *ALICE, BoundedVec::default(), BoundedVec::default()).expect("Creation Failed");
        assert_noop!(
            Dispute::<Test>::new(1, *ALICE, BoundedVec::default(), BoundedVec::default()),
            Error::<Test>::DisputeAlreadyExists
        );
    });
}



// TODO Working on this upon the approval of the finalization
#[test]
fn test_voting_on_a_dispute() {
    let mut jury_vec: BoundedVec<AccountIdOf<Test>, <mock::Test as pallet::Config>::MaxJurySize> = BoundedVec::new();
    jury_vec.try_push(*BOB);
    jury_vec.try_push(*CHARLIE);
    new_test_ext().execute_with(|| {
        Dispute::<Test>::new(10, *ALICE,jury_vec , BoundedVec::default()).expect("Creation Failed");
        assert_eq!(1, PalletDisputes::disputes(10).iter().count());
        PalletDisputes::vote_on_dispute(RuntimeOrigin::signed(*CHARLIE),10,true);
        PalletDisputes::vote_on_dispute(RuntimeOrigin::signed(*BOB),10,false);


    });
}

///testing if the non jury account tries to vote it should throw the error saying its not a jury account
#[test]
fn test_voting_on_a_dispute_from_a_not_jury_account() {
    let mut jury_vec: BoundedVec<AccountIdOf<Test>, <mock::Test as pallet::Config>::MaxJurySize> = BoundedVec::new();
    jury_vec.try_push(*BOB);
   // jury_vec.try_push(*CHARLIE);
    new_test_ext().execute_with(|| {
        Dispute::<Test>::new(10, *ALICE,jury_vec , BoundedVec::default()).expect("Creation Failed");
        assert_noop!(PalletDisputes::vote_on_dispute(RuntimeOrigin::signed(*CHARLIE),10,true),Error::<Test>::NotAJuryAccount);
    });
}

///trying to vote on a dispute that doesn't exists which result in the error throwing dispute does not exists
#[test]
fn test_voting_on_a_dispute_which_does_not_exists() {
    let mut jury_vec: BoundedVec<AccountIdOf<Test>, <mock::Test as pallet::Config>::MaxJurySize> = BoundedVec::new();
    jury_vec.try_push(*BOB);
    new_test_ext().execute_with(|| {
        Dispute::<Test>::new(10, *ALICE,jury_vec , BoundedVec::default()).expect("Creation Failed");
        assert_noop!(PalletDisputes::vote_on_dispute(RuntimeOrigin::signed(*CHARLIE),1,true),Error::<Test>::DisputeDoesNotExist);
    });
}



///trying to extend the voting time  on a dispute that doesn't exists which result in the error throwing dispute does not exists
#[test]
fn test_extending_the_voting_time_on_a_dispute_that_does_not_exist() {
    let mut jury_vec: BoundedVec<AccountIdOf<Test>, <mock::Test as pallet::Config>::MaxJurySize> = BoundedVec::new();
    jury_vec.try_push(*BOB);
    new_test_ext().execute_with(|| {
        Dispute::<Test>::new(10, *ALICE,jury_vec , BoundedVec::default()).expect("Creation Failed");
        assert_noop!(PalletDisputes::extend_dispute(RuntimeOrigin::signed(*BOB),1),Error::<Test>::DisputeDoesNotExist);
    });
}

///testing to extend the time for voting from a not jury account
#[test]
fn test_extending_the_voting_from_a_non_jury_account() {
    let mut jury_vec: BoundedVec<AccountIdOf<Test>, <mock::Test as pallet::Config>::MaxJurySize> = BoundedVec::new();
    jury_vec.try_push(*BOB);
    new_test_ext().execute_with(|| {
        Dispute::<Test>::new(10, *ALICE,jury_vec , BoundedVec::default()).expect("Creation Failed");
        assert_noop!(PalletDisputes::extend_dispute(RuntimeOrigin::signed(*CHARLIE),10),Error::<Test>::NotAJuryAccount);
    });
}

