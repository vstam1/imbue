use super::test_utils::*;


#[test]
fn test_calculate_winner_success_on_half() {
    new_test_ext().execute_with(|| {
        let dispute_key = 0;
        let jury = get_jury::<Test>(vec![*ALICE, *BOB]);
        let specifics = get_specifics::<Test>(vec![0, 1, 2]);
        assert_ok!(Dispute::<Test>::new(
            dispute_key,
            *CHARLIE,
            jury,
            specifics
        ));
        let mut dispute = Disputes::<Test>::get(dispute_key).expect("just inserted, should exist.");
        assert_ok!(dispute.try_add_vote(*ALICE, true));
        assert_eq!(dispute.calculate_winner(), DisputeResult::Success);
    })
}

#[test]
fn test_calculate_winner_success() {
    new_test_ext().execute_with(|| {
        let dispute_key = 0;
        let jury = get_jury::<Test>(vec![*ALICE, *BOB]);
        let specifics = get_specifics::<Test>(vec![0, 1, 2]);
        assert_ok!(Dispute::<Test>::new(
            dispute_key,
            *CHARLIE,
            jury,
            specifics
        ));
        let mut dispute = Disputes::<Test>::get(dispute_key).expect("just inserted, should exist.");
        assert_ok!(dispute.try_add_vote(*ALICE, true));
        assert_ok!(dispute.try_add_vote(*BOB, true));
        assert_eq!(dispute.calculate_winner(), DisputeResult::Success);
    })
}

#[test]
fn test_calculate_winner_failure() {
    new_test_ext().execute_with(|| {
        let dispute_key = 0;
        let jury = get_jury::<Test>(vec![*ALICE, *BOB]);
        let specifics = get_specifics::<Test>(vec![0, 1, 2]);
        assert_ok!(Dispute::<Test>::new(
            dispute_key,
            *CHARLIE,
            jury,
            specifics
        ));
        let mut dispute = Disputes::<Test>::get(dispute_key).expect("just inserted, should exist.");
        assert_ok!(dispute.try_add_vote(*ALICE, false));
        assert_ok!(dispute.try_add_vote(*BOB, false));
        assert_eq!(dispute.calculate_winner(), DisputeResult::Failure);
    })
}