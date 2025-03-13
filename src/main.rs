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
#[macros::runtime]
// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
pub struct Runtime {
    system: system::Pallet<Runtime>,
    balances: balances::Pallet<Runtime>,
    proof_of_existence: proof_of_existence::Pallet<Runtime>,
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
                call: RuntimeCall::balances(balances::Call::transfer {
                    to: dayitva.clone(),
                    amount: 30,
                }),
            },
            support::Extrinsic {
                caller: rajkumar.clone(),
                call: RuntimeCall::balances(balances::Call::transfer {
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
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: "UNISWAP DOCS",
                }),
            },
            support::Extrinsic {
                caller: rajkumar,
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
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
