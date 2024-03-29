#![no_std]
use soroban_sdk::{contract ,contractimpl, Env};

extern crate alloc;

#[contract]
pub struct AllocContract;

#[contractimpl]
impl AllocContract {
    pub fn sum(_env: Env, count: u32) -> u32 {
        let mut v1 = alloc::vec![];
        (0..count).for_each(|i| v1.push(i));

        let mut sum = 0;
        for i in v1 {
            sum += i;
        }

        sum
    }
}

#[cfg(test)]
mod test;
