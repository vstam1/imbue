//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as CrowdFunding;
use common_types::CurrencyId;
use frame_benchmarking::v2::*;
use frame_support::assert_ok;
use frame_system::{EventRecord, RawOrigin};
use orml_traits::MultiCurrency;
use pallet_proposals::ProposedMilestone;
use sp_arithmetic::per_things::Percent;
use sp_core::{Get, H256};
use sp_std::collections::btree_map::BTreeMap;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn create_crowdfund() {
        let caller: T::AccountId = whitelisted_caller();
        let milestones = get_max_milestones::<T>();
        let required_funds = u32::MAX;
        let _currency_id = CurrencyId::Native;
        let agg_hash = H256::from([10u8; 32]);
        let crowdfund_key = 0;
        // (Origin, agg_hash, ProposedMilestones, RequiredFunds, CurrencyId)
        #[extrinsic_call]
        create_crowdfund(
            RawOrigin::Signed(whitelisted_caller()),
            agg_hash,
            milestones,
            required_funds.into(),
            CurrencyId::Native,
        );
        assert_last_event::<T>(
            Event::<T>::CrowdFundCreated(
                caller,
                agg_hash,
                crowdfund_key,
                required_funds.into(),
                CurrencyId::Native,
            )
            .into(),
        );
    }

    #[benchmark]
    fn update_crowdfund() {
        let milestones = get_max_milestones::<T>();
        let required_funds = u32::MAX;
        let caller = create_crowdfund_common::<T>(required_funds);
        let currency_id = CurrencyId::Native;
        let agg_hash = H256::from([2; 32]);

        // origin, crowdfund_key, proposed_milestones, required_funds, currency_id, agreement_hash
        #[extrinsic_call]
        update_crowdfund(
            RawOrigin::Signed(caller.clone()),
            0,
            Some(milestones),
            Some(required_funds.into()),
            Some(currency_id),
            Some(agg_hash),
        );
        assert_last_event::<T>(Event::CrowdFundUpdated(caller, 0).into());
    }

    #[benchmark]
    fn add_crowdfund_whitelist() {
        let required_funds = u32::MAX;
        let caller = create_crowdfund_common::<T>(required_funds);
        let mut bbt: BoundedWhitelistSpots<T> = BTreeMap::new().try_into().unwrap();

        for _i in 0..<T as Config>::MaxWhitelistPerCrowdFund::get() {
            bbt.try_insert(whitelisted_caller(), 100u32.into()).unwrap();
        }
        // (Origin, CrowdFundKey, BoundedWhitelistSpots)
        #[extrinsic_call]
        add_crowdfund_whitelist(RawOrigin::Signed(caller), 0, bbt);
        assert_last_event::<T>(Event::<T>::WhitelistAdded(0, 1u32.into()).into());
    }

    #[benchmark]
    fn remove_crowdfund_whitelist() {
        let required_funds = u32::MAX;
        let caller = create_crowdfund_common::<T>(required_funds);
        let mut bbt: BoundedWhitelistSpots<T> = BTreeMap::new().try_into().unwrap();

        for _i in 0..<T as Config>::MaxWhitelistPerCrowdFund::get() {
            bbt.try_insert(whitelisted_caller(), 100u32.into()).unwrap();
        }
        let _ = CrowdFunding::<T>::add_crowdfund_whitelist(
            RawOrigin::Signed(caller.clone()).into(),
            0,
            bbt,
        );

        // (Origin, CrowdFundKey)
        #[extrinsic_call]
        remove_crowdfund_whitelist(RawOrigin::Signed(caller), 0u32);
        assert_last_event::<T>(Event::<T>::WhitelistRemoved(0).into());
    }

    #[benchmark]
    fn open_contributions() {
        create_crowdfund_common::<T>(u32::MAX);
        // (Origin, CrowdFundKey)
        #[extrinsic_call]
        open_contributions(RawOrigin::Root, 0);
        assert_last_event::<T>(Event::<T>::FundingRoundCreated(0).into());
    }

    #[benchmark]
    fn contribute() {
        let required_funds = u32::MAX;
        create_crowdfund_common::<T>(required_funds);
        let alice: T::AccountId = create_funded_user::<T>("candidate", 1, 100_000);
        let _caller: T::AccountId = whitelisted_caller();
        let _ = CrowdFunding::<T>::open_contributions(RawOrigin::Root.into(), 0);

        //(Origin, CrowdFundKey, Contribution)
        #[extrinsic_call]
        contribute(RawOrigin::Signed(alice.clone()), 0, 10_000u32.into());
        assert_last_event::<T>(Event::<T>::ContributeSucceeded(alice, 0, 10_000u32.into()).into());
    }

    #[benchmark]
    fn approve_crowdfund_for_milestone_submission() {
        let required_funds: u32 = 100_000u32;
        create_crowdfund_common::<T>(required_funds);
        let alice: T::AccountId = create_funded_user::<T>("candidate", 1, required_funds);
        let _ = CrowdFunding::<T>::open_contributions(RawOrigin::Root.into(), 0);
        let _ = CrowdFunding::<T>::contribute(
            RawOrigin::Signed(alice).into(),
            0u32,
            required_funds.into(),
        );

        //(Origin, CrowdFundKey)
        #[extrinsic_call]
        approve_crowdfund_for_milestone_submission(RawOrigin::Root, 0);
        assert_last_event::<T>(Event::<T>::CrowdFundApproved(0).into());
    }
    impl_benchmark_test_suite!(CrowdFunding, crate::mock::new_test_ext(), crate::mock::Test);
}

fn create_funded_user<T: Config>(
    string: &'static str,
    n: u32,
    balance_factor: u32,
) -> T::AccountId {
    let user = account(string, n, 99);
    let balance: BalanceOf<T> = balance_factor.into();
    let _ = <T::MultiCurrency as MultiCurrency<<T as frame_system::Config>::AccountId>>::deposit(
        CurrencyId::Native,
        &user,
        balance,
    );
    user
}

fn create_crowdfund_common<T: Config>(required_funds: u32) -> T::AccountId {
    let bob: T::AccountId = create_funded_user::<T>("initiator", 1, 100_000_000);
    let milestones = get_max_milestones::<T>();

    let agg_hash = H256::from([20; 32]);
    let currency_id = CurrencyId::Native;

    assert_ok!(CrowdFunding::<T>::create_crowdfund(
        RawOrigin::Signed(bob.clone()).into(),
        agg_hash,
        milestones,
        required_funds.into(),
        currency_id
    ));
    bob
}

fn get_milestones<T: Config>(mut n: u32) -> BoundedProposedMilestones<T> {
    let max = <T as Config>::MaxMilestonesPerCrowdFund::get();
    if n > max {
        n = max;
    }

    (0..n)
        .map(|_| ProposedMilestone {
            percentage_to_unlock: Percent::from_percent((100 / n) as u8),
        })
        .collect::<Vec<ProposedMilestone>>()
        .try_into()
        .expect("qed")
}

fn get_max_milestones<T: Config>() -> BoundedProposedMilestones<T> {
    let max_milestones: u32 = <T as Config>::MaxMilestonesPerCrowdFund::get();
    get_milestones::<T>(max_milestones)
}

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
    let events = frame_system::Pallet::<T>::events();
    let system_event: <T as frame_system::Config>::RuntimeEvent = generic_event.into();
    // compare to the last event record
    let EventRecord { event, .. } = &events[events.len() - 1];
    assert_eq!(event, &system_event);
}
