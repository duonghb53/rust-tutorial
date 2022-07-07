use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{assert_one_yocto, env, near_bindgen, AccountId, PanicOnDefault};
use zkp_stark::{*, primefield::*};


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub name: UnorderedMap<AccountId, String>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            name: UnorderedMap::new(b"name".to_vec()),
        }
    }

    #[payable]
    pub fn set_name(&mut self, name: String) {
        assert_one_yocto();
        let account_id = env::predecessor_account_id();
        self.name.insert(&account_id, &name);
    }

    pub fn get_name(&self, account_id: AccountId) -> String {
        self.name.get(&account_id).unwrap_or_else(|| format!(""))
    }
}
