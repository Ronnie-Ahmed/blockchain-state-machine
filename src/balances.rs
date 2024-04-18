use std::{backtrace::Backtrace, collections::BTreeMap};
use num::traits::{CheckedAdd, CheckedSub, Zero};

use crate::system;


pub trait  Config :system::Config{
    type Balance: Zero + CheckedSub + CheckedAdd + Copy;
   
}


#[warn(dead_code)]
#[derive(Debug)]
pub struct Pallet <T:Config>{
    balances:BTreeMap<T::AccountId,T::Balance>
}



#[warn(dead_code)]
impl <T:Config> Pallet<T>

{
    pub fn new()->Self{
        Self { balances: BTreeMap::new() }
    }

    pub fn set_balance(&mut self,who:&T::AccountId,amount:T::Balance){
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self,who:&T::AccountId)->T::Balance{
        match self.balances.get(&who){
            Some(val)=>val.to_owned(),
            None=>{
                println!("Nothing is Found");
                T::Balance::zero()
            }       
        }
    }

    pub fn transfer(&mut self,sender:&T::AccountId,receiver:&T::AccountId,amount:T::Balance)->crate::support::DispatchResult{

        let sender_balance=self.balance(&sender.clone());
        let receiver_balance=self.balance(&receiver.clone());
        let new_sender_balance=sender_balance.checked_sub(&amount).ok_or("Not Enough Funds")?;
        let new_receiver_balance=receiver_balance.checked_add(&amount).ok_or("overflow funds")?;
        self.balances.insert(sender.clone(), new_sender_balance);
        self.balances.insert(receiver.clone(), new_receiver_balance);
        Ok(())
    }
}


#[cfg(test)]
mod tests{
    use crate::system;


    struct TestConfig;

    impl system::Config for TestConfig{
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    impl super::Config for TestConfig{
        type Balance = u128;
    }

    #[test]
    fn init_balances(){
        let mut balances=super::Pallet::<TestConfig>::new();
        let alice=String::from("Alice");
        let bob=String::from("BOB");
        assert_eq!(balances.balance(&alice.clone()),0);
        balances.set_balance(&alice.clone(), 100);
        assert_eq!(balances.balance(&alice.clone()),100);
        assert_eq!(balances.balance(&bob.clone()),0);

    }

    #[test]
    fn transfer_balance(){
        let mut balances=super::Pallet::<TestConfig>::new();

        let amount=100;
        let alice=String::from("Alice");
        let bob=String::from("BOB");
        assert_eq!(balances.transfer(&alice.clone(), &bob.clone(), amount),Err("Not Enough Funds"));
        balances.set_balance(&alice.clone(), amount);
        assert_eq!(balances.transfer(&alice.clone(), &bob.clone(), amount),Ok(()));
        assert_eq!(balances.balance(&alice.clone()),0);
        assert_eq!(balances.balance(&bob.clone()),100);


    }
}