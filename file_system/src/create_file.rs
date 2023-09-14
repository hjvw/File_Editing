use crate::file_system::FileDatas;
use std::error::Error;
use std::fs::OpenOptions;

impl FileDatas{
    pub fn create(&self)->Result<(),Box<dyn Error>>{
        OpenOptions::new().append(true).create(true).open(format!("{}/{}",self.file_name,self.text))?;
        Ok(())
    }
}