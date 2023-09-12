use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;

pub struct FileDatas{
    text:String,
    file_name:String
}

impl FileDatas{
    pub fn new(text:String,file_name:String)->Self{
        FileDatas{text,file_name}
    }
    pub fn to_file(&self)->Result<(),Box<dyn Error>>{
        
        let mut file=OpenOptions::new()
        .append(true)
        .write(true)
        .open(self.file_name.to_string())
        .unwrap();
        
        writeln!(file,"{}",self.text)?;
    //    file.write_all(self.text.as_bytes())?;
        
        file.flush()?;
        

        Ok(())
    }
}

pub fn args_to_string(args:&[String])->String{
    let mut string:String=String::new();
    for x in &args[3..]{
        string+=x;
        string+=" ";
    }
    string
}