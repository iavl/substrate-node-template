use crate::{mock::*, Error, Event};
use crate::*;
use frame_support::{assert_noop, assert_ok};
use frame_system::EventRecord;
use frame_system::Phase;

#[test]
fn demo_x() {
    new_test_ext().execute_with(|| {
        assert_eq!(1,1);
    })
}

#[test]
fn owned_kitties_can_append_value() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        assert_ok!(Kitties::create(Origin::signed(1)));
        assert_eq!(
            System::events()[1].event,
            TestEvent::kitties_event(Event::<Test>::Created(1u64,0))
        );
    })
}

#[test]
fn owner_kitties_failed_when_no_enought_money() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        assert_noop!(KittiesModule::create(Origin::signed(9)), Error::<Test>::MoneyNotEnough);
    })
}