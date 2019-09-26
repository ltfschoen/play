use support::{decl_storage, decl_module, decl_event, dispatch::Result, StorageMap};
use system::{ensure_signed};

use parity_codec::{Encode, Decode};

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Org {
    id: u64,
    fee_paid: bool,
}

pub trait Trait: system::Trait + balances::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as ManagerStorage {
        pub Orgs get(orgs): map T::AccountId  => Option<Org>;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event<T>() = default;
        
        pub fn create_org(origin, id: u64) -> Result {
            let sender = ensure_signed(origin)?;
            let org = Org {
                id,
                fee_paid: false,
            };

            <Orgs<T>>::insert(&sender, &org);
            Self::deposit_event(RawEvent::CreatedOrg(org.id, sender));
            Ok(())
        }
    }
}

decl_event!(
    pub enum Event<T>
    where <T as system::Trait>::AccountId {
		CreatedOrg(u64, AccountId),
	}
);

#[cfg(test)]
mod tests {
	use super::*;

	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher};
	use support::{impl_outer_origin, assert_ok};
	use runtime_primitives::{
		BuildStorage,
		traits::{BlakeTwo256, IdentityLookup},
		testing::{Digest, DigestItem, Header}
	};

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	impl system::Trait for Test {
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
	impl Trait for Test {
		type Event = ();
	}
	type TemplateModule = Module<Test>;

	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		system::GenesisConfig::<Test>::default().build_storage().unwrap().0.into()
	}

	#[test]
	fn it_creates_org() {
		with_externalities(&mut new_test_ext(), || {
			assert_ok!(ManagerModule::create_org(Origin::signed(1), 123));
			assert_eq!(ManagerModule::orgs(123), Some(123));
		});
	}
}
