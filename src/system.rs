use std::{collections::BTreeMap, ops::AddAssign};
use num::traits::{CheckedAdd, CheckedSub, Zero, One};

#[derive(Debug)]

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
pub struct Pallet<AccountId, BlockNumber, Nonce> {
	block_number: BlockNumber,
    nonce: BTreeMap<AccountId,Nonce>,
}

impl <AccountId, BlockNumber, Nonce>Pallet<AccountId, BlockNumber, Nonce> 
	where
	AccountId: Ord + Clone,
	BlockNumber: Zero + One + CheckedSub + CheckedAdd + Copy + AddAssign,
	Nonce: Zero + One + Ord + Clone + Copy,
	{
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self{
            block_number: BlockNumber::zero(),
            nonce: BTreeMap::new()
        }
	}

    /// Get the current block number.
	pub fn block_number(&self) -> BlockNumber {
		self.block_number

	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
        //crashes if overflows
		self.block_number += BlockNumber::one();
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &AccountId) {
		let nonce = *self.nonce.get(who).unwrap_or(&Nonce::zero());
		self.nonce.insert(who.clone(), nonce + Nonce::one());
	}

	// Get the nonce of an account.
	pub fn get_nonce(&self, who: &AccountId) -> Nonce {
		*self.nonce.get(who).unwrap_or(&Nonce::zero())
	}
}

#[cfg(test)]
mod test {

	#[test]
	fn init_system() {
		let mut system: super::Pallet<String, u32, u32> = super::Pallet::new();
        assert_eq!(system.block_number(), 0);
	}

    #[test]
	fn inc_block_number() {
		let mut system: super::Pallet<String, u32, u32> = super::Pallet::new();
        system.inc_block_number();
        assert_eq!(system.block_number(),1);
	}

	#[test]
	fn inc_nonce() {
		let alice = String::from("alice");
		let mut system: super::Pallet<String, u32, u32> = super::Pallet::new();
		system.inc_nonce(&alice.clone());
		assert_eq!(system.get_nonce(&alice), 1);
	}
}