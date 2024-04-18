use std::collections::BTreeMap;
use core::ops::AddAssign;
use num::traits::{One, Zero};

use crate::types::BlockNumber;


// type AccountId=String;
// type BlockNumber=u32;
// type Nonce=u32;

#[derive(Debug)]
pub struct Pallet<AccountId,BlockNumber,Nonce>{
    block_number:BlockNumber,
    nonce:BTreeMap<AccountId,Nonce>
}

impl<AccountId,BlockNumber,Nonce> Pallet <AccountId,BlockNumber,Nonce>

where
    AccountId: Ord + Clone,
    BlockNumber: Zero + One + AddAssign + Copy,
    Nonce: Zero + One + Copy,
{





    pub fn new()->Self{
        Self { block_number: BlockNumber::zero(), nonce: BTreeMap::new() }
    }

    pub fn block_number(&self)->BlockNumber{
        self.block_number
    }

    pub fn inc_block_number(&mut self)->Result<(),&'static str>{
        self.block_number+=BlockNumber::one();
        Ok(())
    }
    pub fn inc_nonce(&mut self,who:&AccountId){
        let nonce = *self.nonce.get(&who).unwrap_or(&Nonce::zero());
		let new_nonce = nonce + Nonce::one();
		self.nonce.insert(who.clone(), new_nonce);
    }
}

#[cfg(test)]
mod test{

    #[test]
    pub fn init_system(){
        let mut system=super::Pallet::<String,u32,u32>::new();
        system.inc_block_number();
        let alice="Alice".to_string();
        assert_eq!(system.block_number(),1);
        let mut nonce=match system.nonce.get(&alice){
            Some(val)=>val.to_owned(),
            None=>{
                println!("None");
                0

            }
        }; 
        assert_eq!(nonce,0);
    }
}