mod balances;
mod system;

#[derive(Debug)]
pub struct Runtime{
	system:system::Pallet,
	balance:balances::Pallet
}


impl Runtime{
	fn new()->Self{
		Self { system: system::Pallet::new(), balance: balances::Pallet::new() }

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
