#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, String};

#[test]
fn simpl_test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ERC721Contract);
    let client = ERC721ContractClient::new(&env, &contract_id);

    let admin = Address::random(&env);
    client.initialize(
        &admin,
        &String::from_slice(&env, "Cool NFT"),
        &String::from_slice(&env, "COOL"),
    );

    let user1 = Address::random(&env);
    client.mock_all_auths().mint(&user1, &1);
    assert_eq!(client.balance_of(&user1), 1);

    let user2 = Address::random(&env);
    client.mock_all_auths().mint(&user2, &2);
    assert_eq!(client.balance_of(&user2), 1);

    assert_eq!(client.total_supply(), 2);

    client.mock_all_auths().mint(&user1, &3);
    assert_eq!(client.balance_of(&user1), 2);

    assert_eq!(client.token_by_index(&0), 1);
    assert_eq!(client.token_by_index(&1), 2);
    assert_eq!(client.token_by_index(&2), 3);

    assert_eq!(client.token_of_owner_by_index(&user1, &0), 1);
    assert_eq!(client.token_of_owner_by_index(&user1, &1), 3);
    assert_eq!(client.token_of_owner_by_index(&user2, &0), 2);

    client.transfer_from(&user1, &user1, &user2, &3);

    assert_eq!(client.balance_of(&user1), 1);
    assert_eq!(client.balance_of(&user2), 2);

    assert_eq!(client.token_of_owner_by_index(&user1, &0), 1);
    assert_eq!(client.token_of_owner_by_index(&user2, &1), 3);
    assert_eq!(client.token_of_owner_by_index(&user2, &0), 2);

    client.burn(&user2, &2);

    assert_eq!(client.balance_of(&user1), 1);
    assert_eq!(client.balance_of(&user2), 1);

    assert_eq!(client.token_of_owner_by_index(&user1, &0), 1);
    assert_eq!(client.token_of_owner_by_index(&user2, &0), 3);
    assert_eq!(client.total_supply(), 2);
}
