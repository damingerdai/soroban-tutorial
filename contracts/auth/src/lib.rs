#![no_std]
use soroban_sdk::{contract,contracttype,contractimpl, Address, Env};

#[contracttype]
pub enum DataKey {
    Counter(Address),
}

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
   pub fn increment(env: Env, user: Address, value: u32) -> u32 {
        user.require_auth();

        let key = DataKey::Counter(user.clone());

        let mut count: u32 = env.storage().persistent().get(&key).unwrap_or_default();

        count += value;

        env.storage().persistent().set(&key, &count);

        count
    } 
}

#[cfg(test)]
mod test;
