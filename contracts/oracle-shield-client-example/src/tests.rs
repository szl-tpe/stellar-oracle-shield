use {super::*, soroban_sdk::Env, soroban_sdk::testutils::Address as AddressTrait};

fn xlm_address(env: &Env) -> Address {
    Address::from_str(
        env,
        "CAS3J7GYLGXMF6TDJBBYYSE3HQ6BBSMLNUQ34T6TZMYMW2EVH34XOWMA",
    )
}

fn usdc_circle_address(env: &Env) -> Address {
    Address::from_str(
        env,
        "CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75",
    )
}

fn shield_contract_client(env: &Env) -> (Address, stellar_oracle_shield::ContractClient<'_>) {
    let admin_address = Address::generate(&env);
    let constructor_args = (&admin_address, 60_u64);
    let contract_id = env.register(stellar_oracle_shield::Contract, constructor_args.clone());

    (
        contract_id.clone(),
        stellar_oracle_shield::ContractClient::new(&env, &contract_id),
    )
}

fn proxy_contract_client(env: &Env, shield_contract_address: Address) -> ContractClient<'_> {
    let base = usdc_circle_address(&env);
    let quote = xlm_address(&env);
    let constructor_args = (&shield_contract_address, &base, &quote);
    let contract_id = env.register(Contract, constructor_args.clone());

    ContractClient::new(&env, &contract_id)
}

#[test]
fn test_get_status() {
    let env = Env::default();
    env.mock_all_auths();

    let (shield_contract_address, shield_client) = shield_contract_client(&env);

    let base = usdc_circle_address(&env);
    let quote = xlm_address(&env);

    shield_client.set_score(&base, &quote, &33_u32);

    let proxy_client = proxy_contract_client(&env, shield_contract_address);
    assert_eq!(
        proxy_client.get_status(),
        stellar_oracle_shield_client::Status::Degraded
    );
}
