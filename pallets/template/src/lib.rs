#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    // use super::*;
	use frame_support::{
		dispatch::DispatchResult, 
		pallet_prelude::*,
		ensure,
	};
	use frame_system::pallet_prelude::*;
	use sp_core::{
		H256,
		H512,
		sr25519::{
			Public,
			Signature,
		},	
	};
	use sp_runtime::{
		traits::{
			BlakeTwo256,
			Hash,
		}
	};
	use sp_std::collections::btree_map::BTreeMap;
	// sc_consensus_aura::AuraApi;

	#[cfg(feature = "std")]
	use serde::{Deserialize, Serialize};

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);


	pub type Value = u128;

	#[cfg_attr(feature="std", derive(Serialize, Deserialize))]
	#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode, Hash, Debug)]
	pub struct TransactionInput {
		pub outpoint: H256,  // ref to a future utxo to be spent
		pub sigscript: H512, // proof 
	}



	#[cfg_attr(feature="std", derive(Serialize, Deserialize))]
	#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode, Hash, Debug)]
	pub struct TransactionOutput {
		pub value: Value, // value associated with this utxo
		pub pubkey: H256, // public key associated with this output, key of the utxo owner
	}

	#[cfg_attr(feature="std", derive(Serialize, Deserialize))]
	#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode, Hash, Debug)]
	pub struct Transaction {
		pub inputs: Vec<TransactionInput>,
		pub outputs: Vec<TransactionOutput>, 
	}

	#[pallet::storage]
	#[pallet::getter(fn utxo_store)]
	pub type UtxoStore<T> = StorageMap<_, Identity, H256, Option<TransactionOutput>>;

	#[pallet::storage]
	#[pallet::getter(fn reward_total)]
	pub type RewardTotal<T> = StorageValue<_, Value>;

	


	#[pallet::genesis_config]
	pub struct GenesisConfig {
		pub genesis_utxos: Vec<TransactionOutput>
	}
	impl Default for GenesisConfig {
		fn default() -> Self { 
			Self{
				genesis_utxos: Vec::<TransactionOutput>::new()
			}
		}
	}

	//TODO THis compiles but does probably nothing. 
	//Written as a manual migration of the FRAME v1 utxo workshop
	// I think the result of the iter below should be stored into the 
	// UtxoStore
	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {

			for utxo in self.genesis_utxos.iter() {
				let hash = BlakeTwo256::hash_of(&utxo);
				// let utxo_clone = utxo.clone();
				UtxoStore::<T>::insert(hash, Some(&utxo));
			};
			// self.genesis_utxos
			// .iter()
			// .cloned()
			// .map(|u| (BlakeTwo256::hash_of(&u), u))
			// .collect::<Vec<_>>();
		}
	}


	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),

		/// Transaction successfully executed [Transaction] 
		TransactionSuccess(Transaction),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}
//TODO Import sc-consensus-aura (had version conflicts)
    // Hooks
	// #[pallet::hooks]
	// impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {
	// 	fn on_finalize(n: T::BlockNumber) {
	// 		// let auth: Vec<_> = Aura::authorities()
	// 		// 	.iter()
	// 		// 	.map(|x| {
	// 		// 		let r: &Public = x.as_ref();
	// 		// 		r.0.into()
	// 		// 	})
	// 		// 	.collect();
	// 		// Self::disperse_reward(&auth);
	// 	}
	// }

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn spend(origin: OriginFor<T>, transaction: Transaction) -> DispatchResultWithPostInfo {
			// Check that the trx is valid
			let reward = Self::validate_transaction(&transaction)?;

			// Write to storage
			Self::update_storage(&transaction, reward)?;

			// Emit success event
			Self::deposit_event(Event::TransactionSuccess(transaction));

			Ok(().into())
		}

	


		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}



	// "Internal" helper functions, callable by code.
	impl<T: Config> Pallet<T> {
		fn get_simple_transaction(transaction: &Transaction) -> Vec<u8> {
			let mut trx = transaction.clone();
			for input in trx.inputs.iter_mut() {
				input.sigscript = H512::zero(); // 0x000...

			}
			trx.encode()
		}

		fn validate_transaction(transaction: &Transaction) -> Result<Value, &'static str> {
			// Check inputs not empty
			// Check outputs not empty
			// Check no duplicate input 
			// Check no duplicate output
			// Check the inputs are valid (each input sigscript must sign the whole transaction except the sigscripts)
			// Check no empty output utxo 
			// Check each output does not exist in the storage 
			// Check total output is not greater that total input

			ensure!( !transaction.inputs.is_empty(), "Transaction with no inputs");
			ensure!( !transaction.outputs.is_empty(), "Transaction with no outputs");

			{ // creates a btree map with unique keys, this deletes duplicates. Allowing to compare to original
				let input_set: BTreeMap<_, ()> = transaction.inputs.iter().map(|input| (input, ())).collect(); 
				ensure!( input_set.len() == transaction.inputs.len(), "each input must only be used once");				
			}

			{ // creates a btree map with unique keys, this deletes duplicates. Allowing to compare to original
				let output_set: BTreeMap<_, ()> = transaction.outputs.iter().map(|input| (input, ())).collect(); 
				ensure!( output_set.len() == transaction.outputs.len(), "each output must only be defined once");				
			}
			
			let simple_transaction = Self::get_simple_transaction(transaction);
			let mut total_input: Value = 0;
			let mut total_output: Value = 0;
			for input in  transaction.inputs.iter() {
				if let Some(input_utxo) = UtxoStore::<T>::get(&input.outpoint).unwrap() {
					ensure!( sp_io::crypto::sr25519_verify(
						&Signature::from_raw(*input.sigscript.as_fixed_bytes()),
						&simple_transaction,
						&Public::from_h256(input_utxo.pubkey)
					), "signature must be valid");
					total_input = total_input.checked_add(input_utxo.value).ok_or("input value overflow")?;
				} else {
					//TODO manage race condition or invalid utxo
				}
			}

			let mut output_index: u64 = 0;
			for output in transaction.outputs.iter() {
				ensure!( output.value > 0, "output value must be greter than 0");
				let hash = BlakeTwo256::hash_of(&(&transaction.encode(), output_index));
				output_index = output_index.checked_add(1).ok_or("output index overflow")?;
				ensure!(UtxoStore::<T>::contains_key(hash),"output already exists");
				total_output = total_output.checked_add(output.value).ok_or("output value overflow")?;
			}

			ensure!(total_output <= total_input, "output value cannot be greated that input value" );

			// returns the diff as reward
			total_input.checked_sub(total_output).ok_or("reward underflow")
		}
		fn update_storage(transaction: &Transaction, reward: Value) -> DispatchResultWithPostInfo {

			// Update total reward 
			let new_total = RewardTotal::<T>::get()
				.unwrap()
				.checked_add(reward)
				.ok_or("reward overvflow")?;
			RewardTotal::<T>::put(new_total);

			// Remove spent utxo
			for input in &transaction.inputs {
				UtxoStore::<T>::remove(input.outpoint);
			}
			// Create new utxo
			let mut index: u64 = 0;
			for output in &transaction.outputs {
				let hash = BlakeTwo256::hash_of( &(&transaction.encode(), index) );
				index = index.checked_add(1).ok_or("output index overflow")?;
				UtxoStore::<T>::insert(hash, Some(output));
			}

			Ok(().into())
		}

		fn disperse_reward(authorities: &[H256]) {
			// divide reward 
			let reward = RewardTotal::<T>::take().unwrap();
			let share_value = reward
				.checked_div(authorities.len() as Value)
				.ok_or("no validators")
				.unwrap();

			if share_value == 0 { return };

			let remainder = reward
				.checked_sub(share_value * authorities.len() as Value)
				.ok_or("sub enderflow")
				.unwrap();

			RewardTotal::<T>::put(remainder as Value);

			// create utxo per validator
			for authority in authorities {
				let utxo = TransactionOutput {
					value: share_value,
					pubkey: *authority,
				};

				let hash = BlakeTwo256::hash_of( &(&utxo,
					// <frame_system::Pallet<T>>::block_number().saturated_into::<u64>()
					 <frame_system::Pallet<T>>::block_number()
				));

				// write to utxo store
				if !UtxoStore::<T>::contains_key(hash) {
					UtxoStore::<T>::insert(hash, Some(utxo));
					sp_runtime::print("Transaction reward sent to");
					sp_runtime::print(hash.as_fixed_bytes() as &[u8]);
				} else {
					sp_runtime::print("Transaction reward wasted");
				}
			};
		}
	}

}
