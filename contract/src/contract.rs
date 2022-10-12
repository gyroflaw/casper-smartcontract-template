use alloc::{collections::BTreeMap, string::String, vec, vec::Vec};
use casper_contract::{
    contract_api::{runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::ToBytes, runtime_args, CLTyped, ContractHash, ContractPackageHash, RuntimeArgs,
    URef, U256, U512,
};
use contract_utils::{set_key, ContractContext, ContractStorage};

use crate::{
    data::{self, DepositPurse},
    Address, ContractEvent, Error,
};
pub trait Contract<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&mut self, fee_wallet: Address) {
        DepositPurse::init();
        self.set_fee_wallet(fee_wallet);
    }

    fn update_purse_balance(&mut self) {
        let new_purse_balance = system::get_purse_balance(self.purse()).unwrap_or_default();
        DepositPurse::update_purse_balance(new_purse_balance);
    }

    fn purse(&self) -> URef {
        DepositPurse::purse()
    }

    fn stored_purse_balance(&self) -> U512 {
        DepositPurse::purse_balance()
    }

    fn assert_valid_cspr_transfer(&mut self, amount: U512) {
        let new_purse_balance = system::get_purse_balance(self.purse()).unwrap_or_default();
        let old_purse_balance = self.stored_purse_balance();

        if !old_purse_balance
            .checked_add(amount)
            .unwrap_or_default()
            .eq(&new_purse_balance)
        {
            // entrypoint is called directly
            self.revert(Error::PermissionDenied);
        }
        self.update_purse_balance();
    }
    fn set_fee_wallet(&mut self, wallet: Address) {
        data::set_fee_wallet(wallet);
    }

    fn fee_wallet(&self) -> Address {
        data::get_fee_wallet()
    }

    fn revert(&self, error: Error) {
        runtime::revert(error);
    }

    fn contract_package_hash(&self) -> ContractPackageHash {
        let hash_addr = self.self_addr().into_hash().unwrap();
        ContractPackageHash::from(hash_addr)
    }

    fn emit(&mut self, event: ContractEvent) {
        data::emit(&event, self.contract_package_hash());
    }
}
