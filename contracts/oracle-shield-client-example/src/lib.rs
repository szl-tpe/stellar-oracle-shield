//! stellar-oracle-shield client example illustrating the use of stellar-oracle-shield standalone crate
#![no_std]

use soroban_sdk::{Address, Env, contract, contractimpl, contractmeta, contracttype};

contractmeta!(
    key = "Description",
    val = "sunzu lab oracle shield client example"
);

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
enum DataKey {
    OracleShieldContract,
    Base,
    Quote,
}

/// oracle shield main contract
#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    /// initialize contract
    /// freeze monitored pair and stellar contract
    /// `oracle_shield_contract` - oracle shield contract address
    /// `base` - SAC address of an asset
    /// `quote` - SAC address of an asset
    pub fn __constructor(env: Env, oracle_shield_contract: Address, base: Address, quote: Address) {
        env.storage()
            .instance()
            .set(&DataKey::OracleShieldContract, &oracle_shield_contract);
        env.storage().instance().set(&DataKey::Base, &base);
        env.storage().instance().set(&DataKey::Quote, &quote);
    }

    /// get health status of the monitored pair
    /// see `stellar-oracle-shield-client::ContractClient::get_status` for possible errors
    pub fn get_status(
        env: Env,
    ) -> Result<stellar_oracle_shield_client::Status, stellar_oracle_shield_client::Error> {
        let instance = env.storage().instance();
        let oracle_shield_contract = instance
            .get(&DataKey::OracleShieldContract)
            .expect("oracle shield contract is unset");
        let base = instance.get(&DataKey::Base).expect("base asset is unset");
        let quote = instance.get(&DataKey::Quote).expect("quote asset is unset");

        let client =
            stellar_oracle_shield_client::ContractClient::new(&env, &oracle_shield_contract);
        match client.try_get_status(&base, &quote) {
            Ok(r) => match r {
                Ok(contract_status) => Ok(contract_status),
                Err(conversion_error) => panic!("{conversion_error:#?}"),
            },
            Err(r) => match r {
                Ok(contract_error) => Err(contract_error),
                Err(invoke_error) => panic!("{invoke_error:#?}"),
            },
        }
    }
}

#[cfg(test)]
mod tests;
