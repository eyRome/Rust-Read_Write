use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;

// Enum Defination for  file read and write
enum GetFile{
    ForRead,
    ForWrite,
}

// Adding methods to the enum
impl GetFile{
  
    // A method for getting file content
    fn file_content(&self) -> String{

        // Buffer string for the saving the file contents
        let mut buff = String::new();

        // Match Self for the enum type
        match self {
            GetFile::ForRead => {
                // Path to the file
                let fy = Path::new("/etc/passwd");
                
                // Open file using the above path and save the file in a temporary file
                let mut tmp_f =  match File::open(fy){
                    // Panic when file open fails
                    Err(e)=> panic!("ERROR: {}",e.description()),
                    // When files open is success , return the file
                    Ok(file)=> file
                };

                // Read the contents of the file into the buffer
                match tmp_f.read_to_string(& mut buff){
                    // Panic wheb reading the content fails
                    Err(e)=> panic!("Could not read from buffer\nERROR: {}",
                        e.description()),
                    Ok(_)=>{ }
                }
            },
            GetFile::ForWrite =>{
            // Call the WRITE_TO_FILE function if the enum type is ForWrite
            GetFile::write_to_file(&self,String::from(buff.clone()));
            }
        }
    // Return the Buffer
    return buff
    }

    // A function for writing to an output file
    fn write_to_file(&self, s: String){

        // Create and open the file for write
        let mut outfile = match File::create(Path::new("./outfile") ){
            // Panic when file creation fails
            Err(e) => panic!("Erro creating file!!!\n{}", e.description()),
            // Return the Empty file with write only permissions
            Ok(f)  => f
        };

        // Write to File in bytes
        match outfile.write_all(s.as_bytes()){
            // Panic when writes fails
            Err(_) => panic!("Could not write to file"),
            // Print Success message when the write is complete
            Ok(_)  => println!("Write Success!!!")
        }
    }
}
 

fn main(){

    let file  = GetFile::ForRead;
    let file2 = GetFile::ForWrite;

    let content = file.file_content();

    file2. write_to_file(content);

}
