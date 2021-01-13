#![cfg_attr(not(feature = "std"), no_std)]


use frame_support::{decl_module, decl_storage, decl_event,
					ensure, decl_error, dispatch, traits::{Get}};
use frame_system::ensure_signed;
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {	
	trait Store for Module<T: Trait> as TemplateModule {
		pub TotalSupply get(fn total_supply): u64 = 21000000;
		pub BalanceOf get(fn balance_of): map hasher(blake2_128_concat) T::AccountId => u64;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		ClaimCreated(AccountId, Vec<u8>),
		ClaimRevoked(AccountId, Vec<u8>),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		NotEnoughBalance,
		BalanceCalcOverflow,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		#[weight = 0]
		pub fn init(origin) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			<BalanceOf<T>>::insert(sender, Self::total_supply());
			Ok(())
		}

		#[weight = 0]
		pub fn transfer(origin, to: T::AccountId, value: u64) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			let sender_balance = Self::balance_of(sender.clone());

			ensure!(sender_balance >= value, Error::<T>::NotEnoughBalance);

			let updated_from_balance = sender_balance.checked_sub(value).ok_or(Error::<T>::BalanceCalcOverflow);
			let receiver_balance = Self::balance_of(to.clone());
			let updated_to_balance = receiver_balance.checked_add(value).ok_or(Error::<T>::BalanceCalcOverflow);

			<BalanceOf<T>>::insert(sender.clone(), updated_from_balance);
			<BalanceOf<T>>::insert(to.clone(), updated_to_balance);

			Ok(())
		}
	}
}
