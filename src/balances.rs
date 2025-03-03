use std::collections::BTreeMap; // we are using this rust collection to store balances

//Here we want to store balance of each user
pub struct Pallet{
    balances: BTreeMap<String, u128>,
} 


impl Pallet {
	pub fn new() -> Self {
		Self {
			balances: BTreeMap::new()
		}
	}
}