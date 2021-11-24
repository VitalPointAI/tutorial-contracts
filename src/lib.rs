use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Incrementor {
    count: i8,
}

#[near_bindgen]
impl Incrementor {
    pub fn increment(&mut self) {
        //check to prevent overflow
        if self.count < 100 {
            self.count = self.count + 1;
        }
        else{
            self.count = 0; 
        }
    }

    pub fn decrement(&mut self) {
        if self.count > 0 {
        self.count = self.count - 1;
        } 
        else {
        self.count = 100; 
        }
    }

    pub fn get_count(&self) -> i8 {
        return self.count;
    }
}
