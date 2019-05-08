use std::error::Error;
use std::io;

enum Input{
	Keyboard,
}

impl Input {
	fn get_input(&self){
		
		match self {
			Input::Keyboard => {
				Input::check_err("Error Occured".to_string());
			}
		}
	}

	fn check_err(s: String){
		let mut buff = String::new();
		match io::stdin().read_line(& mut buff){
			Err(e) => panic!("{}\n{}",s.to_string(), e.description()),
			Ok(_)=>{ println!("{}", buff) }
		}
	}
}


fn main(){

	let value0 = Input::Keyboard;

	value0.get_input();
     	

}
