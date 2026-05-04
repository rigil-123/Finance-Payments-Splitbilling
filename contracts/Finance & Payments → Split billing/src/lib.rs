#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, Address, Vec, Map};

#[contract]
pub struct StreetSplitContract;

// Storage keys
#[derive(Clone)]
pub enum DataKey {
    Split(u64), // split_id
}

#[derive(Clone)]
pub struct Split {
    creator: Address,
    total: i128,
    participants: Vec<Address>,
    paid: Map<Address, bool>,
}

#[contractimpl]
impl StreetSplitContract {

    // Create a new split bill
    pub fn create_split(env: Env, id: u64, creator: Address, total: i128, participants: Vec<Address>) {
        let mut paid_map = Map::new(&env);

        for p in participants.iter() {
            paid_map.set(p.clone(), false);
        }

        let split = Split {
            creator,
            total,
            participants,
            paid: paid_map,
        };

        env.storage().instance().set(&DataKey::Split(id), &split);
    }

    // Pay share
    pub fn pay_share(env: Env, id: u64, user: Address) {
        let mut split: Split = env.storage().instance().get(&DataKey::Split(id)).unwrap();

        let already_paid = split.paid.get(user.clone()).unwrap_or(false);
        if already_paid {
            panic!("Already paid");
        }

        split.paid.set(user, true);
        env.storage().instance().set(&DataKey::Split(id), &split);
    }

    // Check if fully paid
    pub fn is_complete(env: Env, id: u64) -> bool {
        let split: Split = env.storage().instance().get(&DataKey::Split(id)).unwrap();

        for p in split.participants.iter() {
            if !split.paid.get(p.clone()).unwrap() {
                return false;
            }
        }
        true
    }
}