mod balances;
mod system;


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
	println!("Hello, world!");
}
