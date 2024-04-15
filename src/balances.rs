use std::collections::BTreeMap;


#[warn(dead_code)]
pub struct Pallet{
    balances:BTreeMap<String,u128>
}

#[warn(dead_code)]
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

    pub fn transfer(&mut self,sender:&String,receiver:&String,amount:u128)->Result<(),&'static str>{

        let sender_balance=self.balance(&sender.clone());
        let receiver_balance=self.balance(&receiver.clone());
        let new_sender_balance=sender_balance.checked_sub(amount).ok_or("Not Enough Funds")?;
        let new_receiver_balance=receiver_balance.checked_add(amount).ok_or("overflow funds")?;
        self.balances.insert(sender.clone(), new_sender_balance);
        self.balances.insert(receiver.clone(), new_receiver_balance);
        Ok(())
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

    #[test]
    fn transfer_balance(){
        let mut balances=super::Pallet::new();

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