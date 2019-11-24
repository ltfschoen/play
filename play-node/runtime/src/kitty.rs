use support::{decl_storage, decl_module, StorageValue, StorageMap,
    dispatch::Result, ensure, decl_event};
use system::ensure_signed;
use runtime_primitives::traits::{As, Hash};
#[cfg(feature = "std")]
use serde::{Serialize, Deserialize};
use parity_codec::{Encode, Decode};

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Kitty<Hash, Balance> {
    id: Hash,
    dna: Hash,
    price: Balance,
    gen: u64,
}

pub trait Trait: balances::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_event!(
    pub enum Event<T>
    where
        <T as system::Trait>::AccountId,
        <T as system::Trait>::Hash,
        <T as balances::Trait>::Balance
    {
        Created(AccountId, Hash),
        PriceSet(AccountId, Hash, Balance),
        Transferred(AccountId, AccountId, Hash),
    }
);

decl_storage! {
    trait Store for Module<T: Trait> as Kitty {
        Kitties get(kitty) config(): map T::Hash => Kitty<T::Hash, T::Balance>;
        KittyOwner get(owner_of): map T::Hash => Option<T::AccountId>;

        AllKittiesArray get(kitty_by_index): map u64 => T::Hash;
        AllKittiesCount get(all_kitties_count): u64;
        AllKittiesIndex: map T::Hash => u64;

        OwnedKittiesArray get(kitty_of_owner_by_index): map (T::AccountId, u64) => T::Hash;
        OwnedKittiesCount get(owned_kitty_count): map T::AccountId => u64;
        OwnedKittiesIndex: map T::Hash => u64;

        Nonce: u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn deposit_event<T>() = default;

        fn create_kitty(origin) -> Result {
            let sender = ensure_signed(origin)?;
            let nonce = <Nonce<T>>::get();
            let random_hash = (<system::Module<T>>::random_seed(), &sender, nonce)
                .using_encoded(<T as system::Trait>::Hashing::hash);

            let new_kitty = Kitty {
                id: random_hash,
                dna: random_hash,
                price: <T::Balance as As<u64>>::sa(0),
                gen: 0,
            };

            Self::_mint(sender, random_hash, new_kitty)?;

            <Nonce<T>>::mutate(|n| *n += 1);

            Ok(())
        }

        fn set_price(origin, kitty_id: T::Hash, new_price: T::Balance) -> Result {
            let sender = ensure_signed(origin)?;

            ensure!(<Kitties<T>>::exists(kitty_id), "This cat does not exist");

            let owner = Self::owner_of(kitty_id).ok_or("No owner for this kitty")?;
            ensure!(owner == sender, "You do not own this cat");

            let mut kitty = Self::kitty(kitty_id);
            kitty.price = new_price;

            <Kitties<T>>::insert(kitty_id, kitty);

            Self::deposit_event(RawEvent::PriceSet(sender, kitty_id, new_price));

            Ok(())
        }

        fn transfer(origin, to: T::AccountId, kitty_id: T::Hash) -> Result {
            let sender = ensure_signed(origin)?;

            let owner = Self::owner_of(kitty_id).ok_or("No owner for this kitty")?;
            ensure!(owner == sender, "You do not own this kitty");

            Self::_transfer_from(sender, to, kitty_id)?;

            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    fn _mint(to: T::AccountId, kitty_id: T::Hash, new_kitty: Kitty<T::Hash, T::Balance>) -> Result {
        ensure!(!<KittyOwner<T>>::exists(kitty_id), "Kitty already exists");

        let owned_kitty_count = Self::owned_kitty_count(&to);

        let new_owned_kitty_count = owned_kitty_count.checked_add(1)
            .ok_or("Overflow adding a new kitty to account balance")?;

        let all_kitties_count = Self::all_kitties_count();

        let new_all_kitties_count = all_kitties_count.checked_add(1)
            .ok_or("Overflow adding a new kitty to total supply")?;

        <Kitties<T>>::insert(kitty_id, new_kitty);
        <KittyOwner<T>>::insert(kitty_id, &to);

        <AllKittiesArray<T>>::insert(all_kitties_count, kitty_id);
        <AllKittiesCount<T>>::put(new_all_kitties_count);
        <AllKittiesIndex<T>>::insert(kitty_id, all_kitties_count);

        <OwnedKittiesArray<T>>::insert((to.clone(), owned_kitty_count), kitty_id);
        <OwnedKittiesCount<T>>::insert(&to, new_owned_kitty_count);
        <OwnedKittiesIndex<T>>::insert(kitty_id, owned_kitty_count);

        Self::deposit_event(RawEvent::Created(to, kitty_id));

        Ok(())
    }

    fn _transfer_from(from: T::AccountId, to: T::AccountId, kitty_id: T::Hash) -> Result {
        ensure!(<Kitties<T>>::exists(kitty_id), "This cat does not exist");
        let owner = Self::owner_of(kitty_id).ok_or("No owner for this kitty")?;

        ensure!(owner == from, "'from' account does not own this kitty");

        let owned_kitty_count_from = Self::owned_kitty_count(&from);
        let owned_kitty_count_to = Self::owned_kitty_count(&to);

        let new_owned_kitty_count_to = owned_kitty_count_to.checked_add(1)
            .ok_or("Transfer causes overflow of 'to' kitty balance")?;

        let new_owned_kitty_count_from = owned_kitty_count_from.checked_sub(1)
            .ok_or("Transfer causes underflow of 'from' kitty balance")?;

        // "Swap and pop"
        let kitty_index = <OwnedKittiesIndex<T>>::get(kitty_id);
        if kitty_index != new_owned_kitty_count_from {
            let last_kitty_id = <OwnedKittiesArray<T>>::get((from.clone(), new_owned_kitty_count_from));
            <OwnedKittiesArray<T>>::insert((from.clone(), kitty_index), last_kitty_id);
            <OwnedKittiesIndex<T>>::insert(last_kitty_id, kitty_index);
        }

        <KittyOwner<T>>::insert(&kitty_id, &to);
        <OwnedKittiesIndex<T>>::insert(kitty_id, owned_kitty_count_to);

        <OwnedKittiesArray<T>>::remove((from.clone(), new_owned_kitty_count_from));
        <OwnedKittiesArray<T>>::insert((to.clone(), owned_kitty_count_to), kitty_id);

        <OwnedKittiesCount<T>>::insert(&from, new_owned_kitty_count_from);
        <OwnedKittiesCount<T>>::insert(&to, new_owned_kitty_count_to);

        Self::deposit_event(RawEvent::Transferred(from, to, kitty_id));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	// Import test module dependencies
	use support::{impl_outer_origin, assert_ok, assert_noop};
	use runtime_io::{
		with_externalities, TestExternalities
	};
	use primitives::{H256, Blake2Hasher};
	use runtime_primitives::{
		BuildStorage, traits::{BlakeTwo256, IdentityLookup},
		testing::{Digest, DigestItem, Header}
	};

	impl_outer_origin! {
		pub enum Origin for KittyTest {}
	}

	#[derive(Clone, Eq, PartialEq)]
	pub struct KittyTest;

	// Implement the system module traits
	impl system::Trait for KittyTest {
		type Origin = Origin;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type Digest = Digest;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = ();
    type Log = DigestItem;
	}

	// Implement the balances module traits
	impl balances::Trait for KittyTest {
		type Balance = u64;
		type OnFreeBalanceZero = ();
		type OnNewAccount = ();
		type Event = ();
		type TransactionPayment = ();
		type TransferPayment = ();
		type DustRemoval = ();
	}

	// Implement the trait for our own module, `super::Trait`
	impl super::Trait for KittyTest {
    type Event = ();
	}

	// Implement type alias for Kitty module to easily access its methods
	// since the `Module` struct wraps all functions attached to it
	type Kitty = super::Module<KittyTest>;

	// Use a wrapper function to create `TestExternalities`.
	// `build_ext` wrapper function will be used to construct mocks for each unit test.
	// and mostly just build a genesis storage key/value store according to our desired mockup.
	fn build_ext() -> TestExternalities<Blake2Hasher> {
		let mut t = system::GenesisConfig::<KittyTest>::default().build_storage().unwrap().0;
		t.extend(balances::GenesisConfig::<KittyTest>::default().build_storage().unwrap().0);
		// Seed the chain with Kitty
		t.extend(GenesisConfig::<KittyTest> {
				kitty: vec![ (0, H256::random(), H256::random(), 0) ],
		}.build_storage().unwrap().0);

		t.into()
	}

	#[test]
	fn should_build_genesis_kitties() {
		with_externalities(&mut build_ext(), || {
			// Check that kitties exist at genesis
			let kitty0 = Kitty::kitty_by_index(0);
			let kitty1 = Kitty::network_by_index(1);

			// Check we have 2 kitties, as specified
			assert_eq!(Kitty::all_kitties_count(), 2);

			// Check that they are owned correctly
			assert_eq!(Kitty::owner_of_kitty(kitty0), Some(0));
			assert_eq!(Kitty::owner_of_kitty(kitty1), Some(1));

			// Check owners own the correct amount of kitties
			assert_eq!(Kitty::owned_kitty_count(0), 1);
			assert_eq!(Kitty::owned_kitty_count(2), 0);
		})
	}
}
