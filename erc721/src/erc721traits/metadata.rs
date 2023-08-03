use soroban_sdk::{Env, String};
pub trait ERC721Metadata {
    fn name(env: Env) -> String;
    fn symbol(env: Env) -> String;
    fn token_uri(env: Env, token_id: u32) -> String;
}
