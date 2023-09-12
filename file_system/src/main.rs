use std::env;

mod file_system;
fn main() {

    let args:Vec<String>=env::args().collect();

    let contents=file_system::args_to_string(&args);
    //println!("{}",contents);

    let current_file=file_system::FileDatas::new(contents,args[2].clone());

    match args[1].as_str(){
        "write"=>match current_file.to_file(){
            Ok(_)=>(),
            Err(x)=>panic!("Error: {}",x)
        },
        _=>println!("xd"),
    
    }

   

    
    


    
}


