use soroban_sdk::{Address, Env};

pub trait ERC721Burnable {
    fn burn(env: Env, caller: Address, token_id: u32);
}
