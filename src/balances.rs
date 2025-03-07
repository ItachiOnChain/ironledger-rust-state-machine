use std::collections::BTreeMap; // we are using this rust collection to store balances, in real blockchain balances are stored in a database
use num::traits::{CheckedAdd, CheckedSub, Zero};

pub trait Config {
	type AccountId: Ord + Clone;
	type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}

#[derive(Debug)]
//Here we want to store balance of each user
pub struct Pallet<T:Config> {
 	balances: BTreeMap<T::AccountId, T::Balance>,
}


impl <T:Config> Pallet<T> where
	T::AccountId: Ord + Clone,
	T::Balance: Zero + CheckedSub + CheckedAdd + Copy,{
	/// Create a new instance of the balances module.
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	/// Set the balance of an account `who` to some `amount`.
	pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
	}

	/// Get the balance of an account `who`.
	/// If the account has no stored balance, we return zero.
	pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
	}

    /// Transfer `amount` from one account to another.
	/// This function verifies that `from` has at least `amount` balance to transfer,
	/// and that no mathematical overflows occur.
	pub fn transfer(
		&mut self,
		caller: T::AccountId,
		to: T::AccountId,
		amount: T::Balance,
	) -> Result<(), &'static str> {
        let caller_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        let new_caller_balance = caller_balance.checked_sub(&amount)
            .ok_or("Insufficient balance")?;

        let new_to_balance = to_balance.checked_add(&amount)
            .ok_or("Overflow in transfer")?;

        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_balance);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
    
    struct TestConfig;
    impl super::Config for TestConfig {
        type AccountId = String;
        type Balance = u128;
    }

    #[test]

    fn init_balances() {
	let mut balances: super::Pallet<TestConfig> = super::Pallet::new();

	assert_eq!(balances.balance(&"alice".to_string()), 0);
	balances.set_balance(&"alice".to_string(), 100);
	assert_eq!(balances.balance(&"alice".to_string()), 100);
	assert_eq!(balances.balance(&"bob".to_string()), 0);
}

    #[test]
	fn transfer_balance() {
        let alice = "alice".to_string();
        let bob = "bob".to_string();

        let mut balances: super::Pallet<TestConfig> = super::Pallet::new();

        balances.set_balance(&alice.to_string(), 100);
        let _ = balances.transfer(alice.clone(), bob.clone(), 90);

        assert_eq!(balances.balance(&alice), 10);
        assert_eq!(balances.balance(&bob), 90);
	}

    #[test]
    fn transfer_balance_insufficient() {
        let alice = "alice".to_string();
        let bob = "bob".to_string();

        let mut balances: super::Pallet<TestConfig> = super::Pallet::new();

        balances.set_balance(&alice.to_string(), 100);
        let result = balances.transfer(alice.clone(), bob.clone(), 110);

        assert!(result.is_err());
        assert_eq!(balances.balance(&alice), 100);
        assert_eq!(balances.balance(&bob), 0);
    }

    #[test]
    fn transfer_balance_overflow() {
        let alice = "alice".to_string();
        let bob = "bob".to_string();
        let mut balances: super::Pallet<TestConfig> = super::Pallet::new();

        balances.set_balance(&alice.to_string(), 100);
        balances.set_balance(&bob.to_string(), u128::MAX);

        let result = balances.transfer(alice.clone(), bob.clone(), 1);

        assert_eq!(result, Err("Overflow in transfer"));
        assert_eq!(balances.balance(&alice), 100);
        assert_eq!(balances.balance(&bob), u128::MAX);
    }

}