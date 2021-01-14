use crate::{mock::*, Error, Event};
use crate::*;
use frame_support::{assert_noop, assert_ok};
use frame_system::EventRecord;
use frame_system::Phase;


#[test]
fn owned_kitties_can_append_value() {
    new_test_ext().execute_with(|| {
        run_to_block(5);
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_eq!(
            System::events()[1].event,
            TestEvent::kitties_event(Event::<Test>::Created(1u64,0))
        );
    })
}

#[test]
fn owner_kitties_failed_when_balance_not_enough() {
    new_test_ext().execute_with(|| {
        run_to_block(5);
        assert_noop!(KittiesModule::create(Origin::signed(9)), Error::<Test>::BalanceNotEnough);
    })
}