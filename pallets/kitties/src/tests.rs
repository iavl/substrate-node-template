use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use frame_system::EventRecord;
use frame_system::Phase;
use balances::RawEvent;

// create kitty
#[test]
fn owned_kitties_can_append_value() {
    new_test_ext().execute_with(|| {
        run_to_block(5);
        // test create
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        // check event
        assert_eq!(
            System::events()[1].event,
            TestEvent::kitties_event(Event::<Test>::Created(1u64,0))
        );
    })
}

// transfer kitty
#[test]
fn owner_kitties_transfer() {
    new_test_ext().execute_with(|| {
        run_to_block(5);
        let _ = KittiesModule::create(Origin::signed(1));
        // test transfer
        assert_ok!(KittiesModule::transfer(Origin::signed(1), 2, 0));
        // check event
        assert_eq!(
            System::events()[2].event,
            TestEvent::kitties_event(Event::<Test>::Transferred(1u64, 2, 0)),
        )
    })
}

// test NotKittyOwner
#[test]
fn kitties_transfer_failed_when_not_owner() {
    new_test_ext().execute_with(|| {
        run_to_block(5);
        let _ = KittiesModule::create(Origin::signed(1));

        // transfer
        assert_noop!(
            KittiesModule::transfer(Origin::signed(2), 3, 0),
            Error::<Test>::NotKittyOwner
        );
    })
}

// test TransferToSelf
#[test]
fn kitties_transfer_failed_when_transfer_to_self() {
    new_test_ext().execute_with(|| {
        run_to_block(5);
        let _ = KittiesModule::create(Origin::signed(1));

        // transfer
        assert_noop!(
            KittiesModule::transfer(Origin::signed(1), 1, 0),
            Error::<Test>::TransferToSelf
        );
    })
}

// test BalanceNotEnough
#[test]
fn owner_kitties_failed_when_balance_not_enough() {
    new_test_ext().execute_with(|| {
        run_to_block(5);
        assert_noop!(KittiesModule::create(Origin::signed(9)), Error::<Test>::BalanceNotEnough);
    })
}

// test KittyNotExists
#[test]
fn kitties_transfer_failed_when_not_exist() {
    new_test_ext().execute_with(|| {
        // test transfer
        assert_noop!(
            KittiesModule::transfer(Origin::signed(1), 2, 0),
            Error::<Test>::KittyNotExists
        );
    })
}

// breed kitty
#[test]
fn breed_kitty_work() {
    new_test_ext().execute_with(|| {
        run_to_block(5);
        let _ = KittiesModule::create(Origin::signed(1));
        let _ = KittiesModule::create(Origin::signed(1));

        assert_ok!(KittiesModule::breed(Origin::signed(1), 0, 1));
        assert_eq!(
            System::events(),
            vec![
                EventRecord {
                    phase: Phase::Initialization, 
                    event: TestEvent::balances(RawEvent::Reserved(1u64, 5000)), 
                    topics: vec![],
                },
                EventRecord {
                    phase: Phase::Initialization,
                    event: TestEvent::kitties_event(Event::<Test>::Created(1u64, 0)),
                    topics: vec![],
                },
                EventRecord {
                    phase: Phase::Initialization, 
                    event: TestEvent::balances(RawEvent::Reserved(1u64, 5000)), 
                    topics: vec![],
                },
                EventRecord {
                    phase: Phase::Initialization,
                    event: TestEvent::kitties_event(Event::<Test>::Created(1u64, 1)),
                    topics: vec![],
                },
                EventRecord {
                    phase: Phase::Initialization, 
                    event: TestEvent::balances(RawEvent::Reserved(1u64, 5000)), 
                    topics: vec![],
                },
                EventRecord {
                    phase: Phase::Initialization,
                    event: TestEvent::kitties_event(Event::<Test>::Created(1u64, 2)),
                    topics: vec![],
                },
            ]
        );
    })
}

// breed kitty when not exist
#[test]
fn kitties_breed_failed_when_not_exist() {
    new_test_ext().execute_with(|| {
        run_to_block(5);

        assert_noop!(
            KittiesModule::breed(Origin::signed(1), 0, 1),
            Error::<Test>::KittyNotExists
        );
    })
}

// test RequireDifferentParent
#[test]
fn kitties_breed_failed_when_same_parent() {
    new_test_ext().execute_with(|| {
        run_to_block(5);
        let _ = KittiesModule::create(Origin::signed(1));

        assert_noop!(
            KittiesModule::breed(Origin::signed(1), 0, 0),
            Error::<Test>::RequireDifferentParent
        );
    })
}

// breed kitty when not owner
#[test]
fn kitties_breed_failed_when_not_owner() {
    new_test_ext().execute_with(|| {
        run_to_block(5);
        let _ = KittiesModule::create(Origin::signed(1));
        let _ = KittiesModule::create(Origin::signed(1));

        assert_noop!(
            KittiesModule::breed(Origin::signed(2), 0, 1),
            Error::<Test>::NotKittyOwner
        );
    })
}