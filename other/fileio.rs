use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::path::Path;

fn get_file(p: String)-> String{

	let mut buff = String::new();

	let mut f = match File::open(  Path::new(& String::from(p)) ){
		Err(e) => panic!("Error Occred\nERROR: {}", e),
		Ok(file)=> file
	};

	match f.read_to_string(& mut buff){
		Err(e) => panic!("Error reading from file\nERROR: {}",
			e.description()),
		Ok(_)=>{}
	};
	return buff;
}

fn  main(){
	// let fl = Path::new("./greet");
	let f = get_file(String::from("./greet"));
	println!("{:?}",f);
    
    

}
