
pub struct FileDatas{
   pub text:String,
   pub file_name:String
}

impl FileDatas{
    pub fn new(text:String,file_name:String)->Self{
        FileDatas{text,file_name}
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

