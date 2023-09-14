use crate::file_system::FileDatas;
use std::error::Error;
use std::fs;
impl FileDatas{
    pub fn rename(&self)-> Result<(),Box<dyn Error>>{
        fs::rename(self.file_name.to_string(), self.text.to_string())?;
        Ok(())
    }
}