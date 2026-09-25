#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, symbol_short};

#[contract]
pub struct Voting;
#[contractimpl]
impl Voting {
    pub fn initialize(env: Env, admin: Address) { admin.require_auth(); env.storage().instance().set(&symbol_short!("ADMIN"), &admin); }
    pub fn open(env: Env) { let _: Address = env.storage().instance().get(&symbol_short!("ADMIN")).unwrap(); env.storage().instance().set(&symbol_short!("OPEN"), &true); }
    pub fn cast(env: Env, voter: Address, option: Symbol) { voter.require_auth(); let open: bool = env.storage().instance().get(&symbol_short!("OPEN")).unwrap_or(false); if !open { panic!("closed") }; let key = (symbol_short!("V"), voter); if env.storage().persistent().has(&key) { panic!("duplicate") }; env.storage().persistent().set(&key, &option); env.events().publish((symbol_short!("VOTE"),), option); }
}
