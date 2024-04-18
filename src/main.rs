use support::Dispatch;
use types::AccountId;


mod balances;
mod system;
mod support;




mod types{

	pub type AccountId=String;
	pub type Balance=u128;
	pub type BlockNumber=u32;
	pub type Nonce=u32;
	pub type Extrinsic=crate::support::Extrinsic<AccountId,crate::RuntimeCall>;
	pub type Header=crate::support::Header<BlockNumber>;
	pub type Block=crate::support::Block<Header,Extrinsic>;

}

pub enum RuntimeCall{

}




#[derive(Debug)]
pub struct Runtime{
	system:system::Pallet<Self>,
	balance:balances::Pallet<Self>
}

impl system::Config for Runtime{
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl balances::Config for Runtime{
	type Balance = types::Balance;
}


impl Runtime{
	fn new()->Self{
		Self { system: system::Pallet::new(), balance: balances::Pallet::new() }

	}

	// Execute a block of extrinsics. Increments the block number.
	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
		
		self.system.inc_block_number();
		if block.header.block_number!=self.system.block_number(){
			return Err(&"block number does not match what is expected")
		}

		for (i, support::Extrinsic{caller,call}) in block.extrinsics.into_iter().enumerate(){
			self.system.inc_nonce(&caller);
			let _res=self.dispatch(caller, call).map_err(|e| 
				eprintln!(
					"Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
					block.header.block_number, i, e
				)
			);
		}
		/* TODO:
			- Increment the system's block number.
			- Check that the block number of the incoming block matches the current block number,
			  or return an error.
			- Iterate over the extrinsics in the block...
				- Increment the nonce of the caller.
				- Dispatch the extrinsic using the `caller` and the `call` contained in the extrinsic.
				- Handle errors from `dispatch` same as we did for individual calls: printing any
				  error and capturing the result.
				- You can extend the error message to include information like the block number and
				  extrinsic number.
		*/
		Ok(())
	}
} 

impl crate::support::Dispatch for Runtime{
	type Caller = <Runtime as system::Config>::AccountId;
	type Call = RuntimeCall;
	fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> support::DispatchResult {
		unimplemented!()
	}
}

fn main() {
	let mut runtime=Runtime::new();
	let alice=String::from("Alice");
	let bob=String::from("Bob");
	let charlie=String::from("charlie");
	runtime.balance.set_balance(&alice.clone(), 100);

	// start emulating a block
	runtime.system.inc_block_number();
	assert_eq!(runtime.system.block_number(),1);

	// first transaction

	runtime.system.inc_nonce(&alice.clone());
	let _res=runtime.balance.transfer(&alice.clone(), &bob.clone(), 30).map_err(|e| eprintln!("{}",e));

	runtime.system.inc_nonce(&alice.clone());

	let _res=runtime.balance.transfer(&alice.clone(), &charlie.clone(), 20).map_err(|e| eprintln!("{}",e));

	println!("{:#?}",runtime);
}
