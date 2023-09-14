use crate::file_system::FileDatas;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;

impl FileDatas{
    pub fn to_file(&self)->Result<(),Box<dyn Error>>{
        
        let mut file=OpenOptions::new()
        .append(true)
        .write(true)
        .open(self.file_name.to_string())
        .unwrap();
        
        writeln!(file,"{}",self.text)?;
   
        
        file.flush()?;
        

        Ok(())
    }
}