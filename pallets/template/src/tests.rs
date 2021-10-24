use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use super::*;
use sp_runtime::traits::Hash;


#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::something(), Some(42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(TemplateModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
	});
}

#[test]
fn test_simple_transaction() {
	new_test_ext().execute_with( || {
    //get alice's pubkey
	//create a transaction from genesis utxo to a 50 value utxo also owned by alice
	//sign the input with the complete tx wo sigscripts 
	//determine the new utxo hash after 

    	let alice_pub_key = sp_io::crypto::sr25519_public_keys(SR25519)[0];

		let mut transaction = Transaction {
			inputs: vec![TransactionInput {
				outpoint: H256::from(GENESIS_UTXO),
				sigscript: H512::zero(),
			}],
			outputs: vec![TransactionOutput {
				value: 50,
				pubkey: H256::from(alice_pub_key),
			}],
		};
		let alice_signature = sp_io::crypto::sr25519_sign(SR25519, &alice_pub_key, &transaction.encode()).unwrap();
		transaction.inputs[0].sigscript = H512::from(alice_signature);
		let new_utxo_hash = BlakeTwo256::hash_of(&(&transaction.encode(), 0 as u64));
	
		//the test 
	
		//1 Spend will be ok
		assert_ok!(TemplateModule::spend(Origin::signed(0), transaction));
		//2 old utxo is gone
		assert!( ! UtxoStore::<Test>::contains_key(H256::from(GENESIS_UTXO)));
		//3 new utxo is exists with value 50
		assert!(  UtxoStore::<Test>::contains_key(new_utxo_hash));
		// assert_eq!(UtxoStore::<Test>::get(new_utxo_hash),50);
	});

	


}
