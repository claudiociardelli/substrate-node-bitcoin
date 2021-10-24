use crate as pallet_template;
use super::*;
use crate::mock::sp_api_hidden_includes_construct_runtime::hidden_include::traits::GenesisBuild;
use hex_literal::hex;
pub use codec::{Decode, Encode};

use frame_support::parameter_types;
use frame_system as system;
use sp_keystore::testing::KeyStore;
use sp_keystore::KeystoreExt;
use sp_keystore::SyncCryptoStore;
use sp_std::sync::Arc;
pub use sp_core::{
	H256,
	H512,
	testing::SR25519,
};
pub use sp_runtime::{
	BuildStorage,
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		TemplateModule: pallet_template::{Pallet, Call, Storage, Event<T>, Config},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
}

impl pallet_template::Config for Test {
	type Event = Event;
}

// Build genesis storage according to the mock runtime.
const ALICE_PHRASE: &str = "news slush supreme milk chapter athlete soup sausage put clutch what kitten";
pub const GENESIS_UTXO: [u8; 32] = hex!("79eabcbd5ef6e958c6a7851b36da07691c19bda1835a08f875aa286911800999");
pub fn new_test_ext() -> sp_io::TestExternalities {
	// Create keys ofr a test user Alice
	// Store a seed  (100, Alice owned) in genesis storage
    // Store Alices keys storage


	// Create keys for a test user Alice
	let keystore = KeyStore::new(); // a key storage to store new key pairs during testing

	//there was a write here in the workshop. Does not exist in doc?
	let alice_pub_key = keystore.sr25519_generate_new(SR25519, Some(ALICE_PHRASE)).unwrap();

	// Store a seed  (100, Alice owned) in genesis storage
 	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	
	let extension = pallet_template::GenesisConfig  {
		genesis_utxos: vec![
			TransactionOutput {
				value: 100,
				pubkey: H256::from(alice_pub_key)
			}

		],
		..Default::default()
	};

	let extension = extension
	.build_storage()
	.unwrap();

	t.top.extend(
		extension.top,
	);
	
	
	// t.top.extend(
	// 	pallet_template::GenesisConfig  {
	// 		genesis_utxos: vec![
	// 			TransactionOutput {
	// 				value: 100,
	// 				pubkey: H256::from(alice_pub_key)
	// 			}

	// 		],
	// 		..Default::default()
	// 	}
	// 	.build_storage()
	// 	.unwrap()
	// 	.top,
	// );
	let mut ext = sp_io::TestExternalities::from(t);
	
   // Store Alices keys storage
   ext.register_extension(KeystoreExt(Arc::new(keystore)));
   ext
}
