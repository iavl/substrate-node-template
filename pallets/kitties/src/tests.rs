use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

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
