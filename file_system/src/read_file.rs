use std::{fs::OpenOptions, error::Error, io::Read};

pub fn read(file_name:String)->Result<String,Box<dyn Error>>{
    let mut file=OpenOptions::new().read(true).open(file_name)?;
    let mut content=String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}