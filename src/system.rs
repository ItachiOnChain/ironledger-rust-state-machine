use std::collections::BTreeMap;
/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
pub struct Pallet {
	block_number: u32,
    nonce: BTreeMap<String,u32>,
}

impl Pallet {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self{
            block_number:0,
            nonce: BTreeMap::new()
        }
	}

    /// Get the current block number.
	pub fn block_number(&self) -> u32 {
		self.block_number

	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
        //crashes if overflows
		self.block_number = self.block_number.checked_add(1).unwrap();

	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &String) {
		let nonce = self.nonce.get(who).cloned().unwrap_or(0);
		self.nonce.insert(who.clone(), nonce.checked_add(1).unwrap());
	}

	// Get the nonce of an account.
	pub fn nonce(&self, who: &String) -> u32 {
		*self.nonce.get(who).unwrap_or(&0)
	}
}

#[cfg(test)]
mod test {
    use crate::system;

	#[test]
	fn init_system() {
		let mut system = super::Pallet::new();
        assert_eq!(system.block_number(), 0);
	}

    #[test]
	fn inc_block_number() {
		let mut system = super::Pallet::new();
        system.inc_block_number();
        assert_eq!(system.block_number(),1);
	}

	#[test]
	fn inc_nonce() {
		let alice = String::from("alice");
		let mut system = super::Pallet::new();
		system.inc_nonce(&alice.clone());
		assert_eq!(system.nonce(&alice), 1);
	}
}