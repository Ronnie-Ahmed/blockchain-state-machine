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

// A public enum which describes the calls we want to expose to the dispatcher.
// We should expect that the caller of each call will be provided by the dispatcher,
// and not included as a parameter of the call.
pub enum Call<T: Config> {

    CreateClaim{claim:T::Content},
    RevokeClaim{claim:T::Content},

	/*
		TODO:
		Create variants for:
		- `CreateClaim`
		- `RevokeClaim`

		Remember that you only need to pass in the `claim` data, as `caller` information is passed
		in through the `dispatch` logic.
	*/
}

/// Implementation of the dispatch logic, mapping from `POECall` to the appropriate underlying
/// function we want to execute.
impl<T: Config> crate::support::Dispatch for Pallet<T> {

    type Caller = T::AccountId;
    type Call = Call<T>;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
        match call {
            Call::CreateClaim {  claim }=>{
                self.create_claim(caller, claim)
            },
            Call::RevokeClaim {  claim }=>{
                self.revoke_claim(caller, claim)
            }
            
        }
    }
	/*
		TODO:
		Implement `crate::support::Dispatch` for `Pallet<T>`.

		In your `dispatch` logic, match on `call` and forward the `caller` and `claim` data to the
		appropriate function.
	*/
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