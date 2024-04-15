use std::collections::BTreeMap;

pub struct Pallet{
    balances:BTreeMap<String,u128>
}

impl Pallet{
    pub fn new()->Self{
        Self { balances: BTreeMap::new() }
    }

    pub fn set_balance(&mut self,who:&String,amount:u128){
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self,who:&String)->u128{
        match self.balances.get(&who.to_string()){
            Some(val)=>val.to_owned(),
            None=>{
                println!("Nothing is Found");
                0
            }       
        }
    }
}


#[cfg(test)]
mod tests{
    #[test]
    fn init_balances(){
        let mut balances=super::Pallet::new();
        let alice=String::from("Alice");
        let bob=String::from("BOB");
        assert_eq!(balances.balance(&alice.clone()),0);
        balances.set_balance(&alice.clone(), 100);
        assert_eq!(balances.balance(&alice.clone()),100);
        assert_eq!(balances.balance(&bob.clone()),0);

    }
}