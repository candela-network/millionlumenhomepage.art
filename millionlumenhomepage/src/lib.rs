#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String};

#[contract]
pub struct Million;

#[contractimpl]
impl Million {
    pub fn initialize(env: Env, admin: Address) {
        //
        let name = String::from_slice(&env, "Pixel");
        let sym = String::from_slice(&env, "PIX");
        erc721::ERC721Contract::initialize(env, admin, name, sym);
    }
}
