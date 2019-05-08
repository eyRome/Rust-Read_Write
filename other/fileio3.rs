use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;

enum GetFile{
    ForRead,
}

enum MakeFile{
    ForWrite,
}


impl GetFile{
  
    fn file_content(&self) -> String{

    let mut buff = String::new();
    match self {
        GetFile::ForRead => {
            let fy = Path::new("./greet");
            
            
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

}

impl MakeFile{
    
    fn write_to_file(&self, s: String){

    let mut outfile = match File::create(Path::new("./outfile") ){
        Err(e) => panic!("Erro creating file!!!\n{}", e.description()),
        Ok(f) => f
    };

    match outfile.write_all(s.as_bytes()){
        Err(_)=>panic!("Could not write to file"),
        Ok(_)=> println!("Write Success!!!")
    }
}
}


fn main(){

    let file:GetFile = GetFile::ForRead;
    let file2: MakeFile = MakeFile::ForWrite;

    let content = file.file_content();

    file2. write_to_file(content);

}
