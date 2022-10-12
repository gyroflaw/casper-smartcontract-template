use casper_types::{ContractHash, U256};

use crate::Address;

pub enum ContractEvent {
    FeeWalletChanged { fee_wallet: Address },
}
