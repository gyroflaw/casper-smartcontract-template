#![no_main]
#![no_std]

#[macro_use]
extern crate alloc;

use alloc::{
    boxed::Box,
    collections::{BTreeMap, BTreeSet},
    string::String,
    vec::Vec,
};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::NamedKeys, runtime_args, CLType, CLValue, ContractHash, ContractPackageHash,
    EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs,
    URef, U256, U512,
};
use contract_utils::{AdminControl, ContractContext, OnChainContractStorage, ReentrancyGuard};

use hello_contract::{Address, Contract};

#[derive(Default)]
struct HelloContract(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for HelloContract {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl Contract<OnChainContractStorage> for HelloContract {}
impl ReentrancyGuard<OnChainContractStorage> for HelloContract {}
impl AdminControl<OnChainContractStorage> for HelloContract {}

impl HelloContract {
    fn constructor(&mut self, fee_wallet: Address) {
        Contract::init(self, fee_wallet);
        ReentrancyGuard::init(self);
        AdminControl::init(self);
    }
}

#[no_mangle]
pub extern "C" fn constructor() {
    let fee_wallet: Address = runtime::get_named_arg("fee_wallet");
    HelloContract::default().constructor(fee_wallet);
    let default_admin = runtime::get_caller();
    HelloContract::default().add_admin_without_checked(Key::from(default_admin));
}

#[no_mangle]
pub extern "C" fn get_deposit_purse() {
    let purse = HelloContract::default().purse();

    // https://github.com/Jiuhong-casperlabs/restrict-access-right/blob/main/contract/src/contract.rs#L25
    runtime::ret(CLValue::from_t(purse.into_add()).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn set_fee_wallet() {
    let fee_wallet: Address = runtime::get_named_arg("fee_wallet");
    HelloContract::default().assert_caller_is_admin();
    HelloContract::default().set_fee_wallet(fee_wallet);
}

#[no_mangle]
pub extern "C" fn call() {
    let contract_name: String = runtime::get_named_arg("contract_name");

    let fee_wallet: Address = runtime::get_named_arg("fee_wallet");
    let exist_contract_package_hash: Option<ContractPackageHash> = {
        let contract_package_hash_str: Option<String> =
            runtime::get_named_arg("contract_package_hash");
        contract_package_hash_str.map(|str| ContractPackageHash::from_formatted_str(&str).unwrap())
    };
    let (contract_hash, _) = match exist_contract_package_hash {
        Some(contract_package_hash) => {
            let named_keys = NamedKeys::new();

            storage::add_contract_version(contract_package_hash, get_entry_points(), named_keys)
        }
        None => storage::new_contract(
            get_entry_points(),
            None,
            Some(format!("{}_contract_package_hash", contract_name)),
            Some(format!("{}_contract_access_token", contract_name)),
        ),
    };

    let package_hash: ContractPackageHash = ContractPackageHash::new(
        runtime::get_key(&format!("{}_contract_package_hash", contract_name))
            .unwrap_or_revert()
            .into_hash()
            .unwrap_or_revert(),
    );

    let constructor_access: URef = match exist_contract_package_hash {
        Some(contract_package_hash) => {
            storage::provision_contract_user_group_uref(contract_package_hash, "constructor")
                .unwrap()
        }
        None => {
            storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
                .unwrap_or_revert()
                .pop()
                .unwrap_or_revert()
        }
    };
    let constructor_args = runtime_args! {
        "fee_wallet" => fee_wallet
    };
    let _: () = runtime::call_contract(contract_hash, "constructor", constructor_args);

    let mut urefs = BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
        .unwrap_or_revert();

    runtime::put_key(
        &format!("{}_contract_hash", contract_name),
        contract_hash.into(),
    );
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new(
                "acceptable_tokens",
                CLType::Map {
                    key: Box::new(CLType::String),
                    value: Box::new(CLType::U32),
                },
            ),
            Parameter::new("fee_wallet", CLType::Key),
        ],
        CLType::Unit,
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_fee_wallet",
        vec![Parameter::new("fee_wallet", CLType::Key)],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_deposit_purse",
        vec![],
        CLType::URef,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points
}
