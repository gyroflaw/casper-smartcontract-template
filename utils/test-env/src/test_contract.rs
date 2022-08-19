use std::time::SystemTime;

use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, CLTyped, ContractHash, ContractPackageHash,
    RuntimeArgs,
};

use crate::{utils::DeploySource, TestEnv};

pub struct TestContract {
    env: TestEnv,
    name: String,
    contract_owner: AccountHash,
}

impl TestContract {
    pub fn new(
        env: &TestEnv,
        wasm: &str,
        name: &str,
        sender: AccountHash,
        mut args: RuntimeArgs,
    ) -> TestContract {
        args.insert("contract_name", name).unwrap();
        env.depoy_contract(sender, wasm, args);
        TestContract {
            env: env.clone(),
            name: String::from(name),
            contract_owner: sender,
        }
    }

    pub fn query_dictionary<T: CLTyped + FromBytes>(
        &self,
        dict_name: &str,
        key: String,
    ) -> Option<T> {
        self.env
            .query_dictionary(self.contract_hash().value(), dict_name, key)
    }

    pub fn query_dictionary_old<T: CLTyped + FromBytes>(
        &self,
        dict_name: &str,
        key: String,
    ) -> Result<T, String> {
        self.env
            .query_dictionary_old(self.contract_hash().value(), dict_name, key)
    }

    pub fn query_named_key<T: CLTyped + FromBytes>(&self, key: String) -> T {
        let contract_name = format!("{}_contract_hash", self.name);
        self.env
            .query_account_named_key(self.contract_owner, &[contract_name, key])
    }

    pub fn contract_hash(&self) -> ContractHash {
        let key = format!("{}_contract_hash", self.name);
        self.env
            .get_account_named_key(self.contract_owner, key)
            .into_hash()
            .unwrap()
            .into()
    }

    pub fn contract_package_hash(&self) -> ContractPackageHash {
        let key = format!("{}_contract_package_hash", self.name);
        self.env
            .get_account_named_key(self.contract_owner, key)
            .into_hash()
            .unwrap()
            .into()
    }

    pub fn call_contract(&self, sender: AccountHash, entry_point: &str, session_args: RuntimeArgs) {
        let session_code = DeploySource::ByHash {
            hash: self.contract_hash(),
            method: entry_point.to_string(),
        };
        self.env.run(sender, session_code, session_args);
    }

    pub fn call_contract_with_condition(
        &self,
        sender: AccountHash,
        entry_point: &str,
        session_args: RuntimeArgs,
        success: bool,
    ) {
        let session_code = DeploySource::ByHash {
            hash: self.contract_hash(),
            method: entry_point.to_string(),
        };
        self.env
            .run_with_condition(sender, session_code, session_args, success);
    }

    pub fn call_contract_with_time(
        &self,
        sender: AccountHash,
        entry_point: &str,
        session_args: RuntimeArgs,
        time: SystemTime,
    ) {
        let session_code = DeploySource::ByHash {
            hash: self.contract_hash(),
            method: entry_point.to_string(),
        };
        self.env
            .run_with_time(sender, session_code, session_args, time);
    }
}
