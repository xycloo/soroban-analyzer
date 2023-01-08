#![no_std]
use soroban_sdk::{contractimpl, symbol, vec, Env, Symbol, Vec};

pub struct HelloContract;

fn test_s(e: &Env) {
    e.storage().set(symbol!("test"), 1)
}

fn test(e: &Env) {
    let s = symbol!("test");
    e.storage().get(s).unwrap()
}

#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol!("Hello"), to]
    }
}
