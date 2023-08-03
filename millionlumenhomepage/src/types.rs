use soroban_sdk::{contracterror, contracttype, Env};
#[contracttype]
pub enum MillionDataKey {
    TokenId,
    AssetAddress,
    Price,
}
impl storage::Storage for MillionDataKey {
    fn get<V: soroban_sdk::TryFromVal<Env, soroban_sdk::Val>>(&self, env: &Env) -> Option<V> {
        storage::Instance::get(env, self)
    }

    fn set<V: soroban_sdk::IntoVal<Env, soroban_sdk::Val>>(&self, env: &Env, val: &V) {
        storage::Instance::set(env, self, val)
    }

    fn has(&self, env: &Env) -> bool {
        storage::Instance::has(env, self)
    }

    fn bump(&self, env: &Env, expiration_ledger: u32) {
        storage::Instance::bump(env, expiration_ledger)
    }

    fn remove(&self, env: &Env) {
        storage::Instance::remove(env, self)
    }
}

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MillionError {
    Exhausted = 1,
}
