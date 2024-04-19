use core::fmt::Debug;
use std::collections::BTreeMap;

use crate::support::DispatchResult;

pub trait Config: crate::system::Config {
	
	type Content: Debug + Ord;
}


#[derive(Debug)]
pub struct Pallet<T: Config> {
	
    claims:BTreeMap<T::Content,T::AccountId>,
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the Proof of Existence Module.
	pub fn new() -> Self {
        Self { claims: BTreeMap::new() }
	}


 /// Get the owner (if any) of a claim.
    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
 
    self.claims.get(&claim)

    }

    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        /* TODO: Check that a `claim` does not already exist. If so, return an error. */
        /* TODO: `insert` the claim on behalf of `caller`. */
        // match self.get_claim(&claim){
        //     Some(_)=>Err("Already Exist"),
        //     None=>{
        //         self.claims.insert(claim, caller);
                
        //     }
        // }

        if self.claims.contains_key(&claim){
            return Err(&"this content is already claimed")
        }
        self.claims.insert(claim, caller);
    
        Ok(())
    }

    pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {

        let owner=self.get_claim(&claim).ok_or("Not Found")?;

        if caller!=*owner{
            return  Err(&"Don't have access");
        }

        self.claims.remove(&claim);
   
        Ok(())
    }
}
#[cfg(test)]
mod tests{

    struct TestConfig;

    impl super::Config for TestConfig {
		type Content = &'static str;
	}

    impl crate::system::Config for TestConfig{
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
	fn basic_proof_of_existence() {

        let mut peo=super::Pallet::<TestConfig>::new();
        assert_eq!(peo.get_claim(&"Hello World"),None);
        assert_eq!(peo.create_claim("Alice".to_string(), &"Hello World"),Ok(()));
        let alice=String::from("Alice");
        assert_eq!(peo.get_claim(&"Hello World"),Some(&alice));
        assert_eq!(peo.revoke_claim("Alice".to_string(), &"Hello World"),Ok(()));
        assert_eq!(peo.get_claim(&"Hello World"),None);

	}
}