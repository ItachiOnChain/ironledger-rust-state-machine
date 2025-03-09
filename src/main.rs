use crate::support::Dispatch;

mod balances;
mod system;
mod support;

mod types{
    use crate::support;

    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Extrinsic = support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = support::Header<BlockNumber>;
    pub type Block = support::Block<Header, Extrinsic>;
}

pub enum RuntimeCall {}

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber; 
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
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

    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
		self.system.inc_block_number();

        if (self.system.block_number()!= block.header.block_number) {
            return Err("Block number mismatch");
        }

        for (i ,support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
            // Increment the nonce for the caller
            self.system.inc_nonce(&caller);

            // Dispatch the call
            let _ = self.dispatch(caller, call).map_err(|e| {
                eprintln!("Extrensic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
                block.header.block_number, i, e);
            });
        }
		Ok(())
	}
}

//also ADD THIS CODE TO YOUR main.rs file:
impl crate::support::Dispatch for Runtime {
	type Caller = <Runtime as system::Config>::AccountId;
	type Call = RuntimeCall;
	// Dispatch a call on behalf of a caller. Increments the caller's nonce.
	//
	// Dispatch allows us to identify which underlying module call we want to execute.
	// Note that we extract the `caller` from the extrinsic, and use that information
	// to determine who we are executing the call on behalf of.
	fn dispatch(
		&mut self,
		caller: Self::Caller,
		runtime_call: Self::Call,
	) -> support::DispatchResult {
		unimplemented!();
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