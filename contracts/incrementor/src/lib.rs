#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, log};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct IncrementorContract;

#[contractimpl]
impl IncrementorContract {
   pub fn increment(env: Env) ->  u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);

        count += 1;
        log!(&env, "count: {}", count);

        env.storage().instance().set(&COUNTER, &count);

        env.storage().instance().extend_ttl(100,100);

        count
    } 
}

#[cfg(test)]
mod test;
