use std::env;
mod read_file;
mod file_system;
mod write_file;
mod create_file;
mod rename_file;

fn main() {

    let args:Vec<String>=env::args().collect();

    let contents=file_system::args_to_string(&args);
    

    let current_file=file_system::FileDatas::new(contents,args[2].clone());

    match args[1].as_str(){
        "write"=>if let Err(e)= current_file.to_file(){
            panic!("Error: {}",e);
         },
        "read"=> if let Err(e)= current_file.read(){
           panic!("Error: {}",e);
        }
        "create"=> if let Err(e)=current_file.create(){
            panic!("Error: {}",e)
        },
        "rename"=> if let Err(e)=current_file.rename(){
            panic!("Error: {}",e)
        }
        _=>println!("xd"),
    
    }

   

    
    


    
}


