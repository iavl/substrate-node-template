#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use frame_support::{decl_module, decl_storage, decl_event, dispatch,
                    ensure, decl_error, StorageValue, StorageMap, traits::Randomness, Parameter, traits::{Get},
};
use sp_io::hashing::blake2_128;
use frame_system::ensure_signed;
use sp_runtime::{
    traits::{AtLeast32BitUnsigned, Bounded},
    DispatchError,
};
use sp_std::prelude::*;
use sp_std::ops::Index;
use sp_std::vec;
use frame_support::traits::Currency;
use frame_support::traits::ReservableCurrency;


// import test file
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// DNA
#[derive(Encode, Decode, Clone, Copy)]
pub struct DNA(pub [u8; 16]);

impl DNA {
    pub fn new() -> Self {
        Self { 0: [0; 16] }
    }

    pub fn set(self, val: [u8; 16]) -> Self {
        Self { 0: val}
    }
    
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Index<usize> for DNA {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

/// Parent
#[derive(Encode, Decode)]
pub struct Parents {
    father: DNA,
    mother: DNA,
}

impl Parents {
    pub fn new() -> Self {
        Self {
            father: DNA::new(),
            mother: DNA::new(),
        }
    }

    pub fn set_father(self, father: DNA) -> Self {
        Self { father, ..self }
    }

    pub fn set_mother(self, mother: DNA) -> Self {
        Self { mother, ..self }
    }
}

/// Kitty

// #[derive(Encode, Decode)]
// pub struct Kitty(pub [u8; 16]);

#[derive(Encode, Decode)]
pub struct Kitty {
    parents_dna: Parents,
    brothers_dna: DNA,
    children_dna: DNA,
    partner_dna: DNA,
    self_dna: DNA,
}

impl Kitty {
    pub fn new() -> Self {
        Self {
            parents_dna: Parents {
                father: DNA::new(),
                mother: DNA::new(),
            },
            brothers_dna: DNA::new(),
            children_dna: DNA::new(),
            partner_dna: DNA::new(),
            self_dna: DNA::new(),
        }
    }

    pub fn set_self_dna(self, dna: DNA) -> Self {
        Self {
            self_dna: dna,
            ..self
        }
    }

    pub fn set_parents_dna(self, dna: Parents) -> Self {
        Self {
            parents_dna: dna,
            ..self
        }
    }

    pub fn set_partner_dna(self, dna: DNA) -> Self {
        Self {
            partner_dna: dna,
            ..self
        }
    }

    pub fn set_brother_dna(self, dna: DNA) -> Self {
        Self {
            brothers_dna: dna,
            ..self
        }
    }

    pub fn mutate_partner_dna(&mut self, partner_dna: DNA) {
        self.partner_dna = partner_dna;
    }

    pub fn mutate_children_dna(&mut self, children_dna: DNA) {
        self.children_dna = children_dna;
    }

    pub fn get_self_dna(&self) -> DNA {
        self.self_dna
    }
}

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

pub trait Trait: frame_system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type Randomness: Randomness<Self::Hash>;
    type KittyIndex: Parameter + Default + AtLeast32BitUnsigned + Copy + Bounded;
    type NewKittyReserve: Get<BalanceOf<Self>>;
    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
}

decl_storage! {
    trait Store for Module<T: Trait> as Kitties {
        // kitty id => kitty
        pub Kitties get(fn kitties): map hasher(blake2_128_concat) T::KittyIndex => Option<Kitty>;
        // kitty count
        pub KittiesCount get(fn kitties_count): T::KittyIndex;
        // kitty id => owner
        pub KittyOwners get(fn kitty_owner): map hasher(blake2_128_concat) T::KittyIndex => Option<T::AccountId>;
        pub KittyTotal get(fn kitty_total): map hasher(blake2_128_concat) T::AccountId => vec::Vec<T::KittyIndex>;
        pub KittyParents get(fn kitty_parents): map hasher(blake2_128_concat) T::KittyIndex => (T::KittyIndex, T::KittyIndex);
        pub KittyChildren get(fn kitty_children): double_map hasher(blake2_128_concat) T::KittyIndex, hasher(blake2_128_concat) T::KittyIndex => vec::Vec<T::KittyIndex>;
        pub KittyBrother get(fn kitty_brother): map hasher(blake2_128_concat) T::KittyIndex => vec::Vec<T::KittyIndex>;
        pub KittyPartner get(fn kitty_partner): map hasher(blake2_128_concat) T::KittyIndex => T::KittyIndex;
    }
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId, <T as Trait>::KittyIndex {
        Created(AccountId, KittyIndex),
        Transferred(AccountId, AccountId, KittyIndex),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
        KittiesCountOverflow,
        InvalidKittyId,
        RequireDifferentParent,
        BalanceNotEnough,
        KittyNotExists,
        NotKittyOwner,
        TransferToSelf,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
        fn deposit_event() = default;

        #[weight = 0]
        pub fn create(origin) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            let kitty_id = Self::next_kitty_id()?;
            let dna = Self::random_value(&sender);

            let kitty = Kitty::new().set_self_dna(dna);

            // stake token
            T::Currency::reserve(&sender, T::NewKittyReserve::get()).map_err(|_| Error::<T>::BalanceNotEnough)?;

            Self::insert_kitty(&sender, kitty_id, kitty);
            Self::deposit_event(RawEvent::Created(sender, kitty_id));

            Ok(())
        }

        #[weight = 0]
        pub fn transfer(origin, to: T::AccountId, kitty_id: T::KittyIndex) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            // kitty must exist
            let owner = Self::kitty_owner(kitty_id).ok_or(Error::<T>::KittyNotExists)?;
            // check kitty owner
            ensure!(owner == sender, Error::<T>::NotKittyOwner);
            // can't transfer to self
            ensure!(to != sender, Error::<T>::TransferToSelf);

            <KittyOwners<T>>::insert(kitty_id, &to);

            Self::deposit_event(RawEvent::Transferred(sender, to, kitty_id));

            Ok(())
        }

        #[weight = 0]
        pub fn breed(origin, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            let new_kitty_id = Self::do_breed(&sender, kitty_id_1, kitty_id_2)?;
            Self::deposit_event(RawEvent::Created(sender, new_kitty_id));

            Ok(())
        }
	}
}

fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8 {
    (selector & dna1) | (!selector & dna2)
}

impl<T: Trait> Module<T> {
    fn insert_kitty(owner: &T::AccountId, kitty_id: T::KittyIndex, kitty: Kitty) {
        <Kitties<T>>::insert(kitty_id, kitty);
        <KittiesCount<T>>::put(kitty_id + 1.into());
        <KittyOwners<T>>::insert(kitty_id, owner);

        if <KittyTotal<T>>::contains_key(&owner) {
            let _ = <KittyTotal<T>>::mutate(owner, |val| val.push(kitty_id));
        } else {
            <KittyTotal<T>>::insert(owner, vec![kitty_id]);
        }
    }

    fn update_kitty_parents(children: T::KittyIndex, father: T::KittyIndex, mother: T::KittyIndex) {
            <KittyParents<T>>::insert(children, (father, mother));
    }

    fn update_kitty_children(children: T::KittyIndex, father: T::KittyIndex, mother: T::KittyIndex) {
        if <KittyChildren<T>>::contains_key(father, mother) {
            let _ = <KittyChildren<T>>::mutate(father, mother, |val| val.push(children));
        } else {
            <KittyChildren<T>>::insert(father, mother, vec![children]);
        }
    }

    fn update_kitty_brother(kitty_id: T::KittyIndex) {
        let (father, mother) = <KittyParents<T>>::get(kitty_id);

        if <KittyChildren<T>>::contains_key(father, mother) {
            let val: vec::Vec<T::KittyIndex> = <KittyChildren<T>>::get(father, mother);
            let reserve_val: vec::Vec<T::KittyIndex> =
                val.into_iter().filter(|&val| val != kitty_id).collect();
            <KittyBrother<T>>::insert(kitty_id, reserve_val);
        } else {
            <KittyBrother<T>>::insert(kitty_id, vec::Vec::<T::KittyIndex>::new());
        }
    }

    fn update_kitty_partner(partner1: T::KittyIndex, partner2: T::KittyIndex) {
        <KittyPartner<T>>::insert(partner1, partner2);
    }

    fn next_kitty_id() -> sp_std::result::Result<T::KittyIndex, DispatchError> {
        let kitty_id = Self::kitties_count();
        if kitty_id == <T as Trait>::KittyIndex::max_value() {
            return Err(Error::<T>::KittiesCountOverflow.into());
        }

        Ok(kitty_id)
    }

    fn random_value(sender: &T::AccountId) -> DNA {
        let payload = (
            T::Randomness::random_seed(),
            &sender,
            <frame_system::Module<T>>::extrinsic_index(),
        );
        DNA::new().set(payload.using_encoded(blake2_128))
    }

    fn do_breed(
        sender: &T::AccountId, 
        kitty_id_1: T::KittyIndex, 
        kitty_id_2: T::KittyIndex
    ) -> sp_std::result::Result<T::KittyIndex, DispatchError> {
        let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;
        let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;

        ensure!(kitty_id_1 != kitty_id_2, Error::<T>::RequireDifferentParent);

        let owner1 = Self::kitty_owner(kitty_id_1).ok_or(Error::<T>::KittyNotExists)?;
        let owner2 = Self::kitty_owner(kitty_id_2).ok_or(Error::<T>::KittyNotExists)?;

        ensure!(owner1 == *sender, Error::<T>::NotKittyOwner);
        ensure!(owner2 == *sender, Error::<T>::NotKittyOwner);

        let kitty_id = Self::next_kitty_id()?;

        // update kitty partner
        Self::update_kitty_partner(kitty_id_1, kitty_id_2);
        Self::update_kitty_partner(kitty_id_2, kitty_id_1);

        // update kitty parents
        Self::update_kitty_parents(kitty_id, kitty_id_1, kitty_id_2);

        // update kitty children
        Self::update_kitty_children(kitty_id, kitty_id_2, kitty_id_2);


        // update kitty brother
        Self::update_kitty_brother(kitty_id);

        let kitty1_dna = kitty1.get_self_dna();
        let kitty2_dna = kitty2.get_self_dna();
        let selector = Self::random_value(&sender);
        let mut new_dna = [0u8; 16];

        for i in 0..kitty1_dna.len() {
            new_dna[i] = combine_dna(kitty1_dna[i], kitty2_dna[i], selector[i]);
        }

        let new_dna = DNA::new().set(new_dna);
        let new_kitty = Kitty::new().set_self_dna(new_dna);

        // stake token
        T::Currency::reserve(&sender, T::NewKittyReserve::get()).map_err(|_| Error::<T>::BalanceNotEnough)?;

        Self::insert_kitty(sender, kitty_id, new_kitty);
        Ok(kitty_id)
    }
}
