use soroban_sdk::{Address, Env};
pub trait ERC721Enumerable {
    fn total_supply(env: Env) -> u32;
    fn token_by_index(env: Env, index: u32) -> u32;
    fn token_of_owner_by_index(env: Env, owner: Address, index: u32) -> u32;
}
