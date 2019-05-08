use std::io;
use std::error::Error;

fn main(){

	let mut name: String = String::new();

	match io::stdin().read_line(& mut name){
		Err(e) => panic!("ERROR Occured {}", e),
		Ok(_)=>{}
	}

	println!("You entered: {}", name);
	
}