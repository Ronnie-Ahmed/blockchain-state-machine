use std::collections::BTreeMap;
pub struct Pallet{
    block_number:u32,
    nonce:BTreeMap<String,u32>
}

impl Pallet{
    pub fn new()->Self{
        Self { block_number: 0, nonce: BTreeMap::new() }
    }

    pub fn block_number(&self)->u32{
        self.block_number
    }

    pub fn inc_block_number(&mut self)->Result<(),&'static str>{
        self.block_number.checked_add(1).ok_or("Error");
        Ok(())
    }
    pub fn inc_nonce(&mut self,who:&String){
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
        assert_eq!(system.block_number(),0);
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