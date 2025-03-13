use crate::support::Dispatch;

mod balances;
mod proof_of_existence;
mod support;
mod system;

mod types {
    use crate::support;

    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Extrinsic = support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = support::Header<BlockNumber>;
    pub type Block = support::Block<Header, Extrinsic>;
    pub type Content = &'static str; // For simplicity, we use String as the content type.
}

pub enum RuntimeCall {
    Balances(balances::Call<Runtime>),
    ProofOfExistence(proof_of_existence::Call<Runtime>),
}

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

impl proof_of_existence::Config for Runtime {
    type Content = types::Content; // For simplicity, we use String as the content type.
}

#[derive(Debug)]

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
pub struct Runtime {
    system: system::Pallet<Runtime>,
    balances: balances::Pallet<Runtime>,
    proof_of_existence: proof_of_existence::Pallet<Runtime>,
}

impl Runtime {
    // Create a new instance of the main Runtime, by creating a new instance of each pallet.
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
            proof_of_existence: proof_of_existence::Pallet::new(),
        }
    }

    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        self.system.inc_block_number();

        if (self.system.block_number() != block.header.block_number) {
            return Err("Block number mismatch");
        }

        for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
            // Increment the nonce for the caller
            self.system.inc_nonce(&caller);

            // Dispatch the call
            let _ = self.dispatch(caller, call).map_err(|e| {
                eprintln!(
                    "Extrensic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
                    block.header.block_number, i, e
                );
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
        match runtime_call {
            RuntimeCall::Balances(call) => {
                self.balances.dispatch(caller, call)?;
            }
            RuntimeCall::ProofOfExistence(call) => {
                self.proof_of_existence.dispatch(caller, call)?;
            }
        }
        Ok(())
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

    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: rajkumar.clone(),
                call: RuntimeCall::Balances(balances::Call::transfer {
                    to: dayitva.clone(),
                    amount: 30,
                }),
            },
            support::Extrinsic {
                caller: rajkumar.clone(),
                call: RuntimeCall::Balances(balances::Call::transfer {
                    to: aditya.clone(),
                    amount: 20,
                }),
            },
        ],
    };

    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: dayitva.clone(),
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::create_claim {
                    claim: "UNISWAP DOCS",
                }),
            },
            support::Extrinsic {
                caller: rajkumar,
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::create_claim {
                    claim: "UNISWAP V3 DOCS",
                }),
            },
        ],
    };

    runtime
        .execute_block(block_1)
        .expect("Block execution failed");
    runtime
        .execute_block(block_2)
        .expect("Block execution failed");

    println!("{:?}", runtime);
}
