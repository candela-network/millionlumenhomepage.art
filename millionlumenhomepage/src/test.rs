#[cfg(test)]
extern crate std;
use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};
#[test]
fn init() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Million);
    let client = MillionClient::new(&env, &contract_id);

    let admin = Address::random(&env);
    client.initialize(&admin);

    assert_eq!(client.name(), String::from_slice(&env, "Pixel"));
    assert_eq!(client.symbol(), String::from_slice(&env, "PIX"));
}
