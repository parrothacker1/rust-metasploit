use serde::Deserialize as des;
#[derive(des,Debug)]
pub struct consolecreate {
    pub id:u32,
    pub prompt:String,
    pub busy:bool,
}
#[derive(des,Debug)]
pub struct consoledestroy {
    pub result:String,
}
#[derive(des,Debug)]
pub struct consolewrite {
    pub wrote:u32,
}
#[derive(des,Debug)]
pub struct consoleread {
    pub data:String,
    pub prompt:String,
    pub busy:bool,
}
#[derive(des,Debug)]
pub struct consoledetach {
    pub result:String,
}
#[derive(des,Debug)]
pub struct consolekill {
    pub result:String,
}
#[derive(des,Debug)]
pub struct consoletabs {
    pub tabs:Vec<String>,
}
