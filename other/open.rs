use std::io;
use std::error::Error;
// use std::io::Bufread;


fn main(){
	let mut s = String::new();
	let stdin = io::stdin();
	// let input = stdin.lock();
	
	match stdin.read_line(& mut s){
		Err(e) => panic!("Error occured!!\nERROR: {}", e.description()),
		Ok(_) =>{println!("You Entered: {}", s)}  
	};
	
	



}
