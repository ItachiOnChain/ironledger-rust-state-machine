use std::{collections::BTreeMap, ops::AddAssign};
use num::traits::{CheckedAdd, CheckedSub, Zero, One};

pub trait Config {
	type AccountId: Ord + Clone;
	type BlockNumber: Zero + One + AddAssign + Copy;
	type Nonce: Zero + One + Copy;
}

#[derive(Debug)]

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
pub struct Pallet<T:Config> 
	where{
	block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountId,T::Nonce>,
}

impl <T:Config>Pallet<T> 
	where
	T::AccountId: Ord + Clone,
	T::BlockNumber: Zero + One + Copy + AddAssign,
	T::Nonce: Zero + One + Copy,
	{
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self{
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new()
        }
	}

    /// Get the current block number.
	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number

	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
        //crashes if overflows
		self.block_number += T::BlockNumber::one();
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &T::AccountId) {
		let nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
		self.nonce.insert(who.clone(), nonce + T::Nonce::one());
	}

	// Get the nonce of an account.
	pub fn get_nonce(&self, who: &T::AccountId) -> T::Nonce {
		*self.nonce.get(who).unwrap_or(&T::Nonce::zero())
	}
}

#[cfg(test)]
mod test {

	struct TestConfig;

	impl super::Config for TestConfig{
		type AccountId = String;
        type BlockNumber = u32;
   		type Nonce = u32;
	}

	#[test]
	fn init_system() {
		let mut system: super::Pallet<TestConfig> = super::Pallet::new();
        assert_eq!(system.block_number(), 0);
	}

    #[test]
	fn inc_block_number() {
		let mut system: super::Pallet<TestConfig> = super::Pallet::new();
        system.inc_block_number();
        assert_eq!(system.block_number(),1);
	}

	#[test]
	fn inc_nonce() {
		let alice = String::from("alice");
		let mut system: super::Pallet<TestConfig> = super::Pallet::new();
		system.inc_nonce(&alice.clone());
		assert_eq!(system.get_nonce(&alice), 1);
	}
}