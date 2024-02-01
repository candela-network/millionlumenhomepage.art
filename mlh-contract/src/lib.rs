#![no_std]

use erc721::{DataKey, ERC721Metadata, Error, ERC721};
use soroban_sdk::{
    contract, contractimpl, panic_with_error, token, Address, Bytes, BytesN, Env, String,
};
use storage::Storage;
mod types;
use crate::types::*;

const MAX_BUMP: u32 = 100_000;

#[cfg(test)]
pub const MAX_SUPPLY: u32 = 0xff;
#[cfg(test)]
pub const MAX_XY: (u32, u32) = (0xf, 0xf);

#[cfg(not(test))]
pub const MAX_SUPPLY: u32 = 0xfff;
#[cfg(not(test))]
pub const MAX_XY: (u32, u32) = (0x7f, 0x1f);

#[contract]
pub struct Million;

#[cfg(feature = "init")]
#[contractimpl]
impl Million {
    pub fn initialize(env: Env, admin: Address, asset: Address, price: i128) {
        let name = String::from_str(&env, "Pixel");
        let sym = String::from_str(&env, "PIX");
        MillionDataKey::TokenId
            .extend(&env, MAX_BUMP)
            .set::<u32>(&env, &0);
        MillionDataKey::AssetAddress
            .extend(&env, MAX_BUMP)
            .set::<Address>(&env, &asset);
        MillionDataKey::Price
            .extend(&env, MAX_BUMP)
            .set::<i128>(&env, &price);
        erc721::ERC721Contract::initialize(env, admin, name, sym);
    }

    pub fn upgrade(env: Env, wasm_hash: BytesN<32>) {
        erc721::ERC721Contract::upgrade(env, wasm_hash)
    }
}

#[cfg(feature = "prod")]
#[contractimpl]
impl Million {
    #[cfg(not(feature = "init"))]
    pub fn upgrade(env: Env, wasm_hash: BytesN<32>) {
        erc721::ERC721Contract::upgrade(env, wasm_hash)
    }

    pub fn mint(env: Env, x: u32, y: u32, to: Address) -> Result<u32, Error> {
        // Check the destination approved the transaction
        to.require_auth();

        // Check the coordinates are free
        if Coords::Token(x, y).has(&env) {
            panic!("Coordinates already used");
        }

        // Check out of bound
        if x > MAX_XY.0 || y > MAX_XY.1 {
            panic!("X or Y too big")
        }

        // Pay the NFT
        let asset = MillionDataKey::AssetAddress.get::<Address>(&env).unwrap();
        let price = MillionDataKey::Price.get::<i128>(&env).unwrap();
        token::Client::new(&env, &asset).transfer(&to, &erc721::get_admin(&env), &price);

        // Retrieve the token id to mint
        let token_id: u32 = MillionDataKey::TokenId.get(&env).unwrap_or(0);

        // Check if we reached the max supply
        if token_id > MAX_SUPPLY {
            //return Err(MillionError::Exhausted);
            panic!("Exhausted")
        }

        // Compute and store the next token id
        MillionDataKey::TokenId.set::<u32>(&env, &(token_id + 1));
        Coords::Token(x, y).set(&env, &token_id);
        Coords::Xy(token_id).set(&env, &(x, y));

        // Mint
        erc721::ERC721Contract::mint(env.clone(), to.clone(), token_id);
        DataKey::Balance(to).extend(&env, MAX_BUMP);
        DataKey::TokenOwner(token_id).extend(&env, MAX_BUMP);
        Coords::Token(x, y).extend(&env, MAX_BUMP);
        Coords::Xy(token_id).extend(&env, MAX_BUMP);
        Ok(token_id)
    }

    pub fn balance_of(env: Env, owner: Address) -> u32 {
        erc721::ERC721Contract::balance_of(env, owner)
    }

    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, token_id: u32) {
        erc721::ERC721Contract::transfer_from(env, spender, from, to, token_id)
    }

    pub fn approve(
        env: Env,
        caller: Address,
        operator: Option<Address>,
        token_id: u32,
        expiration_ledger: u32,
    ) {
        erc721::ERC721Contract::approve(env, caller, operator, token_id, expiration_ledger)
    }

    pub fn set_approval_for_all(
        env: Env,
        caller: Address,
        owner: Address,
        operator: Address,
        approved: bool,
        expiration_ledger: u32,
    ) {
        erc721::ERC721Contract::set_approval_for_all(
            env,
            caller,
            owner,
            operator,
            approved,
            expiration_ledger,
        )
    }

    pub fn get_approved(env: Env, token_id: u32) -> Option<Address> {
        erc721::ERC721Contract::get_approved(env, token_id)
    }

    pub fn is_approval_for_all(env: Env, owner: Address, operator: Address) -> bool {
        erc721::ERC721Contract::is_approval_for_all(env, owner, operator)
    }

    pub fn name(env: Env) -> String {
        erc721::ERC721Contract::name(env)
    }

    pub fn symbol(env: Env) -> String {
        erc721::ERC721Contract::symbol(env)
    }

    pub fn token_uri(env: Env, token_id: u32) -> String {
        if token_id < MillionDataKey::TokenId.get(&env).unwrap_or(0) {
            const BASE: &str = "http://localhost:3000/test/";
            //const BASE: &str = "https://millionlumenhomepage.art/test/";
            let d = to_hex(token_id);

            // concat
            let mut uri = Bytes::new(&env);
            uri.extend_from_slice(BASE.as_bytes());
            uri.extend_from_slice(d.as_slice());
            uri.extend_from_slice(".json".as_bytes());

            // Bytes to &str
            let mut slice = [0; BASE.len() + 10];
            uri.copy_into_slice(&mut slice);
            let struri = core::str::from_utf8(slice.as_slice()).unwrap();

            String::from_str(&env, struri)
        } else {
            panic_with_error!(&env, Error::NotNFT);
        }
    }

    pub fn total_supply(env: Env) -> u32 {
        MillionDataKey::TokenId
            .extend(&env, 1000)
            .get(&env)
            .unwrap_or(0)
    }

    pub fn owner_of(env: Env, token_id: u32) -> Address {
        erc721::DataKey::TokenOwner(token_id)
            .get(&env)
            .unwrap_or_else(|| panic!("token_id does not exist"))
    }

    pub fn coords(env: Env, token_id: u32) -> Option<(u32, u32)> {
        Coords::Xy(token_id).get(&env)?
    }
}
fn to_hex(n: u32) -> [u8; 5] {
    let mut out = [0; 5];
    out[0] = b'0';
    out[1] = b'x';
    for i in (0..3).rev() {
        let x = ((n >> (i * 4)) & 0xf) as u8;
        let digit: u8 = match x {
            x @ 0..=9 => b'0' + x,
            x @ 10..=15 => b'a' + (x - 10),
            x => panic!("number not in the range 0..16: {}", x),
        };

        out[2 - i + 2] = digit;
    }

    out
}
#[cfg(test)]
mod test;
