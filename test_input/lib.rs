#![no_std]
use soroban_sdk::{contractimpl, symbol, vec, Env, Symbol, Vec};

pub struct HelloContract;

fn test_s(e: &Env) {
    e.storage().set(symbol!("test"), 1)
}

fn test_s_loop(e: &Env) {
    for _ in 0..10 {
        test_s(e)
    }
}

#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        for _ in 0..10 {
            test_s(&env)
        }

        vec![&env, symbol!("Hello"), to]
    }

    pub fn test(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol!("Hello"), to]
    }
}

mod test;
