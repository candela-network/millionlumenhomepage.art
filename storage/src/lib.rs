#![no_std]
use soroban_sdk::{Env, IntoVal, TryFromVal, Val};

pub trait Storage {
    fn get<V: TryFromVal<Env, Val>>(&self, env: &Env) -> Option<V>;
    fn set<V: IntoVal<Env, Val>>(&self, env: &Env, val: &V);
    fn has(&self, env: &Env) -> bool;
    fn extend(&self, env: &Env, min_ledger_to_live: u32) -> &Self;
    fn remove(&self, env: &Env);
}

pub struct Instance;
impl Instance {
    pub fn get<K: IntoVal<Env, Val>, V: TryFromVal<Env, Val>>(env: &Env, key: &K) -> Option<V> {
        env.storage().instance().get::<K, V>(key)
    }
    pub fn set<K: IntoVal<Env, Val>, V: IntoVal<Env, Val>>(env: &Env, key: &K, val: &V) {
        env.storage().instance().set(key, val);
    }
    pub fn has<K: IntoVal<Env, Val>>(env: &Env, key: &K) -> bool {
        env.storage().instance().has(key)
    }
    pub fn extend(env: &Env, min_ledger_to_live: u32) {
        env.storage()
            .instance()
            .extend_ttl(min_ledger_to_live, min_ledger_to_live)
    }
    pub fn remove<K: IntoVal<Env, Val>>(env: &Env, key: &K) {
        env.storage().instance().remove(key);
    }
}

pub struct Persistent;
impl Persistent {
    pub fn get<K: IntoVal<Env, Val>, V: TryFromVal<Env, Val>>(env: &Env, key: &K) -> Option<V> {
        env.storage().persistent().get::<K, V>(key)
    }
    pub fn set<K: IntoVal<Env, Val>, V: IntoVal<Env, Val>>(env: &Env, key: &K, val: &V) {
        env.storage().persistent().set(key, val);
    }
    pub fn has<K: IntoVal<Env, Val>>(env: &Env, key: &K) -> bool {
        env.storage().persistent().has(key)
    }
    pub fn extend<K: IntoVal<Env, Val>>(env: &Env, key: &K, min_ledger_to_live: u32) {
        env.storage()
            .persistent()
            .extend_ttl(key, min_ledger_to_live, min_ledger_to_live)
    }
    pub fn remove<K: IntoVal<Env, Val>>(env: &Env, key: &K) {
        env.storage().persistent().remove(key);
    }
}

pub struct Temporary;
impl Temporary {
    pub fn get<K: IntoVal<Env, Val>, V: TryFromVal<Env, Val>>(env: &Env, key: &K) -> Option<V> {
        env.storage().temporary().get::<K, V>(key)
    }
    pub fn set<K: IntoVal<Env, Val>, V: IntoVal<Env, Val>>(env: &Env, key: &K, val: &V) {
        env.storage().temporary().set(key, val);
    }
    pub fn has<K: IntoVal<Env, Val>>(env: &Env, key: &K) -> bool {
        env.storage().temporary().has(key)
    }
    pub fn extend<K: IntoVal<Env, Val>>(env: &Env, key: &K, min_ledger_to_live: u32) {
        env.storage()
            .temporary()
            .extend_ttl(key, min_ledger_to_live, min_ledger_to_live)
    }
    pub fn remove<K: IntoVal<Env, Val>>(env: &Env, key: &K) {
        env.storage().temporary().remove(key);
    }
}
