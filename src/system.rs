use std::collections::BTreeMap;

type AccountId=String;
type BlockNumber=u32;
type Nonce=u32;

#[derive(Debug)]
pub struct Pallet{
    block_number:BlockNumber,
    nonce:BTreeMap<AccountId,BlockNumber>
}

impl Pallet{
    pub fn new()->Self{
        Self { block_number: 0, nonce: BTreeMap::new() }
    }

    pub fn block_number(&self)->BlockNumber{
        self.block_number
    }

    pub fn inc_block_number(&mut self)->Result<(),&'static str>{
        self.block_number+=1;
        Ok(())
    }
    pub fn inc_nonce(&mut self,who:&AccountId){
        let nonce=self.nonce.get(&who.clone()).unwrap_or(&0);
		let new_nonce = nonce + 1;
        self.nonce.insert(who.clone(), new_nonce);
    }
}

#[cfg(test)]
mod test{

    #[test]
    pub fn init_system(){
        let mut system=super::Pallet::new();
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