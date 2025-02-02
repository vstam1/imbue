use super::*;

// Saves a bit of typing.
type DCIdOf<Test> = <Test as Config>::DepositCurrencyId;

#[test]
fn freelancer_to_vetter_works() {
    new_test_ext().execute_with(|| {
        FellowToVetter::<Test>::insert(ALICE, BOB);
        let v = <Fellowship as MaybeConvert<&AccountIdOf<Test>, VetterIdOf<Test>>>::maybe_convert(
            &ALICE,
        )
        .expect("we just inserted so should be there.");
        assert_eq!(v, BOB);
        assert!(
            <Fellowship as MaybeConvert<&AccountIdOf<Test>, VetterIdOf<Test>>>::maybe_convert(&BOB)
                .is_none()
        );
    });
}

#[test]
fn force_add_fellowship_only_force_permitted() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Fellowship::force_add_fellowship(
                RuntimeOrigin::signed(ALICE),
                BOB,
                Role::Freelancer,
                10
            ),
            BadOrigin
        );
    });
}

#[test]
fn force_add_fellowship_ok_event_assert() {
    new_test_ext().execute_with(|| {
        assert_ok!(Fellowship::force_add_fellowship(
            RuntimeOrigin::root(),
            BOB,
            Role::Freelancer,
            10
        ));
        System::assert_last_event(
            Event::<Test>::FellowshipAdded {
                who: BOB,
                role: Role::Freelancer,
            }
            .into(),
        );
    });
}

#[test]
fn leave_fellowship_not_fellow() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Fellowship::leave_fellowship(RuntimeOrigin::signed(ALICE)),
            Error::<Test>::NotAFellow
        );
    });
}

#[test]
fn force_add_fellowship_then_leave_fellowship_maintains_fellow_reserve() {
    new_test_ext().execute_with(|| {
        let alice_reserved_before =
            <Test as Config>::MultiCurrency::reserved_balance(DCIdOf::<Test>::get(), &ALICE);
        Fellowship::force_add_fellowship(RuntimeOrigin::root(), ALICE, Role::Freelancer, 10)
            .expect("qed");
        assert_ok!(Fellowship::leave_fellowship(RuntimeOrigin::signed(ALICE)));
        let alice_reserved_after =
            <Test as Config>::MultiCurrency::reserved_balance(DCIdOf::<Test>::get(), &ALICE);
        assert_eq!(alice_reserved_before, alice_reserved_after);
    });
}

#[test]
fn leave_fellowship_assert_event() {
    new_test_ext().execute_with(|| {
        Fellowship::force_add_fellowship(RuntimeOrigin::root(), ALICE, Role::Freelancer, 10)
            .expect("qed");
        assert_ok!(Fellowship::leave_fellowship(RuntimeOrigin::signed(ALICE)));
        System::assert_last_event(Event::<Test>::FellowshipRemoved { who: ALICE }.into());
    });
}

#[test]
fn add_to_fellowship_takes_deposit_if_avaliable() {
    new_test_ext().execute_with(|| {
        let alice_reserved_before =
            <Test as Config>::MultiCurrency::reserved_balance(DCIdOf::<Test>::get(), &ALICE);
        assert!(add_to_fellowship_take_deposit(&ALICE, Role::Freelancer, 10, None).is_ok());
        let alice_reserved_after =
            <Test as Config>::MultiCurrency::reserved_balance(DCIdOf::<Test>::get(), &ALICE);
        assert_eq!(
            alice_reserved_after - alice_reserved_before,
            <Test as Config>::MembershipDeposit::get()
        );
    });
}

#[test]
fn add_to_fellowship_adds_to_pending_fellows_where_deposit_fails() {
    new_test_ext().execute_with(|| {
        let free = <Test as Config>::MultiCurrency::free_balance(DCIdOf::<Test>::get(), &ALICE);
        let minimum = <Test as Config>::MultiCurrency::minimum_balance(DCIdOf::<Test>::get());
        assert_ok!(<Test as Config>::MultiCurrency::withdraw(
            DCIdOf::<Test>::get(),
            &ALICE,
            free - minimum + minimum
        ));
        assert!(add_to_fellowship_take_deposit(&ALICE, Role::Freelancer, 10, None).is_ok());
        assert_eq!(
            PendingFellows::<Test>::get(ALICE)
                .expect("Pending fellows should have the account inserted."),
            (Role::Freelancer, 10)
        );
    });
}

#[test]
fn add_to_fellowship_adds_to_pending_fellows_assert_event() {
    new_test_ext().execute_with(|| {
        let free = <Test as Config>::MultiCurrency::free_balance(DCIdOf::<Test>::get(), &ALICE);
        let minimum = <Test as Config>::MultiCurrency::minimum_balance(DCIdOf::<Test>::get());
        <Test as Config>::MultiCurrency::withdraw(
            DCIdOf::<Test>::get(),
            &ALICE,
            free - minimum + minimum,
        )
        .unwrap();
        assert!(add_to_fellowship_take_deposit(&ALICE, Role::Freelancer, 10, None).is_ok());
        System::assert_last_event(Event::<Test>::MemberAddedToPendingFellows { who: ALICE }.into());
    });
}

#[test]
fn add_to_fellowship_adds_vetter_if_exists() {
    new_test_ext().execute_with(|| {
        assert!(add_to_fellowship_take_deposit(&ALICE, Role::Freelancer, 10, Some(&BOB)).is_ok());
        assert_eq!(FellowToVetter::<Test>::get(ALICE).unwrap(), BOB);
    });
}

#[test]
fn add_to_fellowship_edits_role_if_exists_already() {
    new_test_ext().execute_with(|| {
        assert!(add_to_fellowship_take_deposit(&ALICE, Role::Freelancer, 10, Some(&BOB)).is_ok());
        assert_eq!(Roles::<Test>::get(ALICE).unwrap(), (Role::Freelancer, 10));
        assert!(add_to_fellowship_take_deposit(&ALICE, Role::Vetter, 5, Some(&BOB)).is_ok());
        assert_eq!(Roles::<Test>::get(ALICE).unwrap(), (Role::Vetter, 5));
    });
}

#[test]
fn add_to_fellowship_maintains_vetter_if_exists_already() {
    new_test_ext().execute_with(|| {
        assert!(add_to_fellowship_take_deposit(&ALICE, Role::Freelancer, 10, Some(&BOB)).is_ok());
        assert!(add_to_fellowship_take_deposit(&ALICE, Role::Vetter, 5, Some(&CHARLIE)).is_ok());
        assert_eq!(FellowToVetter::<Test>::get(ALICE).unwrap(), BOB);
    });
}

#[test]
fn revoke_fellowship_not_a_fellow() {
    new_test_ext().execute_with(|| {
        assert_noop!(revoke_fellowship(&ALICE, true), Error::<Test>::NotAFellow);
        assert_noop!(revoke_fellowship(&ALICE, false), Error::<Test>::NotAFellow);
    });
}

#[test]
fn revoke_fellowship_unreserves_if_deposit_taken_no_slash() {
    new_test_ext().execute_with(|| {
        let alice_reserved_before =
            <Test as Config>::MultiCurrency::reserved_balance(DCIdOf::<Test>::get(), &ALICE);
        assert!(add_to_fellowship_take_deposit(&ALICE, Role::Vetter, 5, Some(&CHARLIE)).is_ok());
        assert_ok!(revoke_fellowship(&ALICE, false));
        let alice_reserved_after =
            <Test as Config>::MultiCurrency::reserved_balance(DCIdOf::<Test>::get(), &ALICE);
        assert_eq!(
            alice_reserved_before, alice_reserved_after,
            "deposit should be returned if no slash has occurred."
        )
    });
}

#[test]
fn revoke_fellowship_slashes_if_deposit_taken() {
    new_test_ext().execute_with(|| {
        let alice_reserved_before =
            <Test as Config>::MultiCurrency::reserved_balance(DCIdOf::<Test>::get(), &ALICE);
        assert!(add_to_fellowship_take_deposit(&ALICE, Role::Vetter, 5, Some(&CHARLIE)).is_ok());
        assert_ok!(revoke_fellowship(&ALICE, true));
        let alice_reserved_after =
            <Test as Config>::MultiCurrency::reserved_balance(DCIdOf::<Test>::get(), &ALICE);
        assert_eq!(
            alice_reserved_before,
            alice_reserved_after.saturating_sub(<Test as Config>::MembershipDeposit::get()),
            "deposit should have been taken since slash has occurred"
        );
    });
}

#[test]
fn revoke_fellowship_with_slash_goes_to_slash_account() {
    new_test_ext().execute_with(|| {
        let slash_before = <Test as Config>::MultiCurrency::free_balance(
            DCIdOf::<Test>::get(),
            &<Test as Config>::SlashAccount::get(),
        );
        assert!(add_to_fellowship_take_deposit(&ALICE, Role::Vetter, 5, Some(&CHARLIE)).is_ok());
        assert_ok!(revoke_fellowship(&ALICE, true));
        let slash_after = <Test as Config>::MultiCurrency::free_balance(
            DCIdOf::<Test>::get(),
            &<Test as Config>::SlashAccount::get(),
        );
        assert_eq!(
            slash_after - slash_before,
            <Test as Config>::MembershipDeposit::get(),
            "slash account should have increased by membership deposit.",
        )
    });
}

#[test]
fn add_candidate_to_shortlist_not_a_vetter() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Fellowship::add_candidate_to_shortlist(
                RuntimeOrigin::signed(ALICE),
                BOB,
                Role::Freelancer,
                10
            ),
            Error::<Test>::NotAFellow
        );
    });
}

#[test]
fn add_candidate_to_shortlist_already_fellow() {
    new_test_ext().execute_with(|| {
        assert_ok!(add_to_fellowship_take_deposit(
            &ALICE,
            Role::Vetter,
            5,
            Some(&CHARLIE)
        ));
        assert_ok!(add_to_fellowship_take_deposit(
            &BOB,
            Role::Freelancer,
            5,
            Some(&CHARLIE)
        ));
        assert_noop!(
            Fellowship::add_candidate_to_shortlist(
                RuntimeOrigin::signed(ALICE),
                BOB,
                Role::Freelancer,
                10
            ),
            Error::<Test>::AlreadyAFellow
        );
    });
}

#[test]
fn add_candidate_to_shortlist_candidate_lacks_deposit_fails() {
    new_test_ext().execute_with(|| {
        assert_ok!(add_to_fellowship_take_deposit(&BOB, Role::Vetter, 5, None));
        let free = <Test as Config>::MultiCurrency::free_balance(DCIdOf::<Test>::get(), &ALICE);
        let minimum = <Test as Config>::MultiCurrency::minimum_balance(DCIdOf::<Test>::get());
        <Test as Config>::MultiCurrency::withdraw(
            DCIdOf::<Test>::get(),
            &ALICE,
            free - minimum + minimum,
        )
        .unwrap();
        assert_noop!(
            Fellowship::add_candidate_to_shortlist(
                RuntimeOrigin::signed(BOB),
                ALICE,
                Role::Freelancer,
                10
            ),
            Error::<Test>::CandidateDepositRequired
        );
    });
}

#[test]
fn add_candidate_to_shortlist_candidate_already_on_shortlist() {
    new_test_ext().execute_with(|| {
        assert_ok!(add_to_fellowship_take_deposit(&BOB, Role::Vetter, 5, None));
        assert_ok!(Fellowship::add_candidate_to_shortlist(
            RuntimeOrigin::signed(BOB),
            ALICE,
            Role::Freelancer,
            10
        ));
        assert_noop!(
            Fellowship::add_candidate_to_shortlist(
                RuntimeOrigin::signed(BOB),
                ALICE,
                Role::Freelancer,
                10
            ),
            Error::<Test>::CandidateAlreadyOnShortlist
        );
    });
}

#[test]
fn add_candidate_to_shortlist_too_many_candidates() {
    new_test_ext().execute_with(|| {
        assert_ok!(add_to_fellowship_take_deposit(
            &CHARLIE,
            Role::Vetter,
            5,
            None
        ));
        let mut shortlist: BoundedShortlistPlaces<Test> = BoundedBTreeMap::new();
        (0..<Test as Config>::MaxCandidatesPerShortlist::get()).for_each(|i| {
            shortlist
                .try_insert(i as u128, ((Role::Vetter, 10), None))
                .unwrap();
        });
        CandidateShortlist::<Test>::mutate(ShortlistRound::<Test>::get(), |m_shortlist| {
            *m_shortlist = shortlist
        });
        assert_noop!(
            Fellowship::add_candidate_to_shortlist(
                RuntimeOrigin::signed(CHARLIE),
                BOB,
                Role::Freelancer,
                10
            ),
            Error::<Test>::TooManyCandidates
        );
    })
}

#[test]
fn add_candidate_to_shortlist_works_assert_event() {
    new_test_ext().execute_with(|| {
        assert_ok!(add_to_fellowship_take_deposit(&BOB, Role::Vetter, 5, None));
        assert_ok!(Fellowship::add_candidate_to_shortlist(
            RuntimeOrigin::signed(BOB),
            ALICE,
            Role::Freelancer,
            10
        ));
        System::assert_last_event(Event::<Test>::CandidateAddedToShortlist { who: ALICE }.into());
    });
}

#[test]
fn remove_candidate_from_shortlist_not_a_vetter() {
    new_test_ext().execute_with(|| {
        assert_ok!(add_to_fellowship_take_deposit(&BOB, Role::Vetter, 5, None));
        assert_ok!(Fellowship::add_candidate_to_shortlist(
            RuntimeOrigin::signed(BOB),
            ALICE,
            Role::Freelancer,
            10
        ));

        assert_noop!(
            Fellowship::remove_candidate_from_shortlist(RuntimeOrigin::signed(CHARLIE), ALICE),
            Error::<Test>::NotAFellow
        );
    });
}

#[test]
fn remove_candidate_from_shortlist_works_assert_event() {
    new_test_ext().execute_with(|| {
        assert_ok!(add_to_fellowship_take_deposit(&BOB, Role::Vetter, 5, None));
        assert_ok!(Fellowship::add_candidate_to_shortlist(
            RuntimeOrigin::signed(BOB),
            ALICE,
            Role::Freelancer,
            10
        ));
        assert_ok!(Fellowship::remove_candidate_from_shortlist(
            RuntimeOrigin::signed(BOB),
            ALICE
        ));
        assert!(
            CandidateShortlist::<Test>::get(ShortlistRound::<Test>::get())
                .get(&ALICE)
                .is_none()
        );
        System::assert_last_event(
            Event::<Test>::CandidateRemovedFromShortlist { who: ALICE }.into(),
        );
    });
}

#[test]
fn pay_deposit_and_remove_pending_status_not_pending() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Fellowship::pay_deposit_to_remove_pending_status(RuntimeOrigin::signed(ALICE)),
            Error::<Test>::NotAFellow
        );
    });
}

#[test]
fn pay_deposit_and_remove_pending_status_not_enough_funds_to_reserve() {
    new_test_ext().execute_with(|| {
        let minimum = <Test as Config>::MultiCurrency::minimum_balance(DCIdOf::<Test>::get());
        let free = <Test as Config>::MultiCurrency::free_balance(DCIdOf::<Test>::get(), &ALICE);
        <Test as Config>::MultiCurrency::withdraw(
            DCIdOf::<Test>::get(),
            &ALICE,
            free - minimum + minimum,
        )
        .unwrap();
        assert_ok!(add_to_fellowship_take_deposit(
            &ALICE,
            Role::Freelancer,
            5,
            None
        ));
        assert_noop!(
            Fellowship::pay_deposit_to_remove_pending_status(RuntimeOrigin::signed(ALICE)),
            TokensError::<Test>::BalanceTooLow
        );
    });
}

#[test]
fn pay_deposit_and_remove_pending_status_works_assert_event() {
    new_test_ext().execute_with(|| {
        let minimum = <Test as Config>::MultiCurrency::minimum_balance(DCIdOf::<Test>::get());
        let free = <Test as Config>::MultiCurrency::free_balance(DCIdOf::<Test>::get(), &ALICE);
        <Test as Config>::MultiCurrency::withdraw(
            DCIdOf::<Test>::get(),
            &ALICE,
            free - minimum + minimum,
        )
        .unwrap();
        assert_ok!(add_to_fellowship_take_deposit(
            &ALICE,
            Role::Freelancer,
            5,
            None
        ));
        <Test as Config>::MultiCurrency::deposit(
            DCIdOf::<Test>::get(),
            &ALICE,
            <Test as Config>::MembershipDeposit::get() + 100_000,
        )
        .unwrap();
        assert_ok!(Fellowship::pay_deposit_to_remove_pending_status(
            RuntimeOrigin::signed(ALICE)
        ));
        System::assert_last_event(
            Event::<Test>::FellowshipAdded {
                who: ALICE,
                role: Role::Freelancer,
            }
            .into(),
        );
    });
}

#[test]
fn on_initialize_adds_to_fellowship_from_shortlist() {
    new_test_ext().execute_with(|| {
        assert_ok!(Fellowship::force_add_fellowship(
            RuntimeOrigin::root(),
            ALICE,
            Role::Freelancer,
            10
        ));
        assert_ok!(Fellowship::add_candidate_to_shortlist(
            RuntimeOrigin::signed(ALICE),
            CHARLIE,
            Role::Vetter,
            10
        ));
        run_to_block::<Test>(
            frame_system::Pallet::<Test>::block_number() + <Test as Config>::ShortlistPeriod::get(),
        );
        assert_eq!(Roles::<Test>::get(CHARLIE).unwrap(), (Role::Vetter, 10));
    });
}

#[test]
fn on_initialize_doesnt_add_removed_shortlist_members() {
    new_test_ext().execute_with(|| {
        assert_ok!(Fellowship::force_add_fellowship(
            RuntimeOrigin::root(),
            ALICE,
            Role::Freelancer,
            10
        ));
        assert_ok!(Fellowship::add_candidate_to_shortlist(
            RuntimeOrigin::signed(ALICE),
            CHARLIE,
            Role::Vetter,
            10
        ));
        assert_ok!(Fellowship::remove_candidate_from_shortlist(
            RuntimeOrigin::signed(ALICE),
            CHARLIE,
        ));
        run_to_block::<Test>(
            frame_system::Pallet::<Test>::block_number() + <Test as Config>::ShortlistPeriod::get(),
        );
        assert!(Roles::<Test>::get(CHARLIE).is_none());
    });
}

#[test]
fn on_initialize_cleans_storage_for_next_round() {
    new_test_ext().execute_with(|| {
        assert_ok!(Fellowship::force_add_fellowship(
            RuntimeOrigin::root(),
            ALICE,
            Role::Freelancer,
            10
        ));
        assert_ok!(Fellowship::add_candidate_to_shortlist(
            RuntimeOrigin::signed(ALICE),
            CHARLIE,
            Role::Vetter,
            10
        ));
        let pre_shortlist_round_key = ShortlistRound::<Test>::get();
        assert!(
            CandidateShortlist::<Test>::get(pre_shortlist_round_key)
                .iter()
                .len()
                == 1
        );
        run_to_block::<Test>(
            frame_system::Pallet::<Test>::block_number() + <Test as Config>::ShortlistPeriod::get(),
        );

        let post_shortlist_round_key = ShortlistRound::<Test>::get();
        assert_eq!(post_shortlist_round_key, pre_shortlist_round_key + 1);
        assert!(
            CandidateShortlist::<Test>::get(post_shortlist_round_key)
                .iter()
                .len()
                == 0
        );
    });
}

#[test]
fn e2e() {
    new_test_ext().execute_with(|| {
        // force add some vetters to for clarity of state
        assert_ok!(Fellowship::force_add_fellowship(
            RuntimeOrigin::root(),
            ALICE,
            Role::Freelancer,
            10
        ));
        assert_ok!(Fellowship::force_add_fellowship(
            RuntimeOrigin::root(),
            BOB,
            Role::Vetter,
            10
        ));

        // Add multiple people to the shortlist using multiple vetters/freelancers
        assert_ok!(Fellowship::add_candidate_to_shortlist(
            RuntimeOrigin::signed(ALICE),
            CHARLIE,
            Role::Vetter,
            10
        ));

        // Bypass the usual requirement of deposit so we can test the e2e for PendingFellows
        assert_ok!(<Test as Config>::MultiCurrency::deposit(
            CurrencyId::Native,
            &EMPTY,
            <Test as Config>::MembershipDeposit::get() * 2
        ));
        assert_ok!(Fellowship::add_candidate_to_shortlist(
            RuntimeOrigin::signed(BOB),
            EMPTY,
            Role::Freelancer,
            10
        ));
        assert_ok!(<Test as Config>::MultiCurrency::withdraw(
            CurrencyId::Native,
            &EMPTY,
            <Test as Config>::MembershipDeposit::get() + 100
        ));

        // wait for blocks to pass
        run_to_block::<Test>(
            frame_system::Pallet::<Test>::block_number() + <Test as Config>::ShortlistPeriod::get(),
        );

        // ensure they are part of the fellowships or if without funds the pending fellows.
        assert_eq!(
            PendingFellows::<Test>::get(EMPTY).unwrap(),
            (Role::Freelancer, 10)
        );
        assert_eq!(Roles::<Test>::get(CHARLIE).unwrap(), (Role::Vetter, 10));

        // Deposit the required funds and pay.
        assert_ok!(<Test as Config>::MultiCurrency::deposit(
            CurrencyId::Native,
            &EMPTY,
            <Test as Config>::MembershipDeposit::get() * 2
        ));
        assert_ok!(Fellowship::pay_deposit_to_remove_pending_status(
            RuntimeOrigin::signed(EMPTY)
        ));
        assert_eq!(Roles::<Test>::get(EMPTY).unwrap(), (Role::Freelancer, 10));
    });
}
