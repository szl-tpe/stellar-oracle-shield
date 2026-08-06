# Stellar Oracle Shield Client

Stellar Oracle Shield is an oracle risk monitoring and circuit-breaker infrastructure layer for Stellar Smart Contract DeFi. It does not replace price feeds. Instead, it helps protocols decide whether current market conditions are safe enough to trust and act on oracle prices.
Stellar Oracle Shield Client provides the sdk (Client) to call Stellar Oracle Shield contract from another contract (on-chain).

## Example

```
/// oracle shield client contract
#[contract]
pub struct ClientContract;

#[contractimpl]
impl ClientContract {
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
    pub fn get_status(env: Env) -> Result<Status, Error> {
        let instance = env.storage().instance();
        let oracle_shield_contract = instance
            .get(&DataKey::OracleShieldContract)
            .expect("oracle shield contract is unset");
        let base = instance.get(&DataKey::Base).expect("base asset is unset");
        let quote = instance.get(&DataKey::Quote).expect("quote asset is unset");

        let client = Client::new(&env, &oracle_shield_contract);
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
```
