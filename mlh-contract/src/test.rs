#[cfg(test)]
extern crate std;

use std::println;

use super::*;
//use crate::MillionError;
use soroban_sdk::{testutils::Address as _, Address, Env};
#[test]
fn init() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Million);
    let client = MillionClient::new(&env, &contract_id);

    let asset_admin = Address::random(&env);
    let native_addr = env.register_stellar_asset_contract(asset_admin);

    let admin = Address::random(&env);
    client.initialize(&admin, &native_addr, &100);

    assert_eq!(client.name(), String::from_slice(&env, "Pixel"));
    assert_eq!(client.symbol(), String::from_slice(&env, "PIX"));
}

#[test]
fn mint() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Million);
    let client = MillionClient::new(&env, &contract_id);

    let asset_admin = Address::random(&env);
    let native_addr = env.register_stellar_asset_contract(asset_admin.clone());
    let asset_client_admin = token::AdminClient::new(&env, &native_addr);

    let admin = Address::random(&env);
    client.initialize(&admin, &native_addr, &2_560_000_000);

    assert_eq!(client.name(), String::from_slice(&env, "Pixel"));
    assert_eq!(client.symbol(), String::from_slice(&env, "PIX"));

    let user1 = Address::random(&env);
    asset_client_admin
        .mock_all_auths()
        .mint(&user1, &2_560_000_000);
    client.mock_all_auths().mint(&0, &1, &user1);
    let auths = env.auths();
    for a in auths.into_iter() {
        std::println!("{:?}", a.1);
    }

    //std::println!("{:?}", r1);
    let user2 = Address::random(&env);
    asset_client_admin
        .mock_all_auths()
        .mint(&user2, &2_560_000_000);
    let _ = client.mock_all_auths().try_mint(&0, &0, &user2);

    assert_eq!(client.balance_of(&user1), 1);
    assert_eq!(client.balance_of(&user2), 1);

    let turi = client.token_uri(&0);
    let mut uri = [0u8; 67];
    let (sl, _) = uri.split_at_mut(turi.len() as usize);
    turi.copy_into_slice(sl);
    //println!("{:?}", std::str::from_utf8(uri.as_slice()));
    assert_eq!(sl, "http://localhost:3000/test/0x000.json".as_bytes());
    client.token_uri(&1).copy_into_slice(sl);
    assert_eq!(sl, "http://localhost:3000/test/0x001.json".as_bytes());
}
#[test]
fn mint_all() {
    let max = crate::MAX_SUPPLY + 1;
    let env = Env::default();
    let contract_id = env.register_contract(None, Million);
    let client = MillionClient::new(&env, &contract_id);

    let asset_admin = Address::random(&env);
    let native_addr = env.register_stellar_asset_contract(asset_admin.clone());
    let asset_client_admin = token::AdminClient::new(&env, &native_addr);

    let admin = Address::random(&env);
    client.initialize(&admin, &native_addr, &2_560_000_000);

    env.budget().reset_unlimited();
    for i in 0..max {
        let user1 = Address::random(&env);
        asset_client_admin
            .mock_all_auths()
            .mint(&user1, &2_560_000_000);
        let x = i % (MAX_XY.0 + 1);
        let y = i / (MAX_XY.0 + 1);
        println!("x: {x}, y: {y}");
        let r = client.mock_all_auths().try_mint(&x, &y, &user1);
        match r {
            Ok(Ok(_)) => {}
            _ => {
                panic!("FAIL");
            }
        }
    }

    assert_eq!(client.total_supply(), max);
    let user1 = Address::random(&env);
    asset_client_admin
        .mock_all_auths()
        .mint(&user1, &2_560_000_000);
    let result = client.mock_all_auths().try_mint(&0, &0, &user1);

    //println!("{:?}", result);
    match result {
        Err(Err(_)) => {
            // Ok
        }
        _ => panic!("Expect an error"),
    }
    //assert_eq!(result, Err(Ok(MillionError::Exhausted)));
}
