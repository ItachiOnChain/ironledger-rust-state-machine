mod balances;
mod system;

mod types{
    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
}

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber; 
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type AccountId = types::AccountId;
    type Balance = types::Balance;    
}

#[derive(Debug)]

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
pub struct Runtime {
    system: system::Pallet<Runtime>,
    balances: balances::Pallet<Runtime>,
}

impl Runtime {
	// Create a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		Self { 
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
         }
	}
}


fn main() {
    let mut runtime = Runtime::new();

    //Genesis state
    let rajkumar = "rajkumar".to_string();
    let dayitva = "dayitva".to_string();
    let aditya = "aditya".to_string();

    // Set initial balances
    runtime.balances.set_balance(&rajkumar, 100);

    runtime.system.inc_block_number();

    assert_eq!(runtime.system.block_number(), 1);

    runtime.system.inc_nonce(&rajkumar);

    //First transaction
    let _ = runtime.balances.transfer(rajkumar.clone(), dayitva.clone(), 50)
    .map_err(|e| println!("Error: {:?}", e));

    runtime.system.inc_nonce( &rajkumar);

    // Second transaction
    let _ = runtime.balances.transfer(dayitva.clone(), aditya.clone(), 30)
    .map_err(|e| println!("Error: {:?}", e));

    println!("{:?}", runtime);
}