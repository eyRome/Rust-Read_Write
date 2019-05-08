use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;

enum GetFile{
    ForRead,
}

fn file_content(f: GetFile) -> String{

    let mut buff = String::new();
    match f {
        GetFile::ForRead => {
            let fy = Path::new("/home/aggrey/.bashrc");
            
            
            let mut tmp_f =  match File::open(fy){
                Err(e)=> panic!("ERROR: {}",e.description()),
                Ok(file)=> file
            };

            match tmp_f.read_to_string(& mut buff){
                Err(e)=> panic!("Could not read from buffer\nERROR: {}",
                    e.description()),
                Ok(_)=>{ }
            }
        }
    }
    return buff
}


fn write_to_file(s: String){

    let mut outfile = match File::create(Path::new("./outfile") ){
        Err(e) => panic!("Erro creating file!!!\n{}", e.description()),
        Ok(f) => f
    };

    match outfile.write_all(s.as_bytes()){
        Err(_)=>panic!("Could not write to file"),
        Ok(_)=> println!("Write Success!!!")
    }
}


fn main(){

    let file:GetFile = GetFile::ForRead;
    write_to_file( file_content(file) );

//    write_to_file(c);

}
