use alloc::{
    collections::BTreeMap,
    format,
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::{
    contract_api::{runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::ToBytes, CLTyped, ContractHash, ContractPackageHash, Key, URef, U512,
};
use contract_utils::{get_key, key_and_value_to_str, key_to_str, set_key, Dict};

use crate::{Address, ContractEvent};

const PURSE_KEY_NAME: &str = "deposit_purse";
const PURSE_BALANCE_KEY_NAME: &str = "purse_balance";

#[derive(Default)]
pub struct DepositPurse {}

impl DepositPurse {
    pub fn init() {
        if runtime::get_key(PURSE_KEY_NAME).is_none() {
            let purse = system::create_purse();
            runtime::put_key(PURSE_KEY_NAME, Key::from(purse));
            set_key(PURSE_BALANCE_KEY_NAME, U512::zero());
        }
    }

    pub fn purse() -> URef {
        *runtime::get_key(PURSE_KEY_NAME).unwrap().as_uref().unwrap()
    }

    pub fn purse_balance() -> U512 {
        get_key(PURSE_BALANCE_KEY_NAME).unwrap_or_revert()
    }

    pub fn update_purse_balance(balance: U512) {
        set_key(PURSE_BALANCE_KEY_NAME, balance);
    }
}

const FEE_WALLET_KEY: &str = "fee_wallet";

pub fn set_fee_wallet(wallet: Address) {
    set_key(FEE_WALLET_KEY, wallet);
}

pub fn get_fee_wallet() -> Address {
    get_key(FEE_WALLET_KEY).unwrap_or_revert()
}

pub fn emit(event: &ContractEvent, contract_package_hash: ContractPackageHash) {
    let mut events: Vec<BTreeMap<&str, String>> = Vec::new();
    // TODO: Emit events
    for param in events {
        let _: URef = storage::new_uref(param);
    }
}
