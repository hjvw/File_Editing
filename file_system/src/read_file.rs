use std::{fs::OpenOptions, error::Error, io::Read};
use crate::file_system::FileDatas;


impl FileDatas{
pub fn read(&self)->Result<(),Box<dyn Error>>{
    let mut content=String::new();
    OpenOptions::new().read(true).open(self.file_name.to_string())?.read_to_string(&mut content)?;
    println!("{}",content);
    Ok(())
}
}