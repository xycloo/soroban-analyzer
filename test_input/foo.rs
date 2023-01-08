#![no_std]
use soroban_sdk::{contractimpl, symbol, vec, Env, Symbol, Vec};

pub struct HelloContract;

fn test_s(e: &Env) {
    e.storage().set(symbol!("test"), 1)
}

fn get_test(e: &Env) -> i32 {
    e.storage().get(symbol!("test")).unwrap().unwrap()
}

fn test_s_loop(e: &Env) {
    for _ in 0..10 {
        test_s(e)
    }
}

fn add(e: &Env, lhs: i32, rhs: i32) -> i32 {
    lhs + rhs
}

#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        for _ in 0..10 {
            1
        }

        for _ in 0..10 {
            test_s(&env);
        }
        test_s(&env);

        vec![&env, symbol!("Hello"), to]
    }

    pub fn test(env: Env, to: Symbol) -> Vec<Symbol> {
        add(get_test(), get_test());

        vec![&env, symbol!("Hello"), to]
    }
}
