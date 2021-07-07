use serde::Deserialize as des;
#[derive(des,Debug)]
pub struct authlogin {
    pub result:String,
    pub token:String,
}
#[derive(des,Debug)]
pub struct authlogout {
    pub result:String,
}
#[derive(des,Debug)]
pub struct authtokenadd {
    pub result:String,
}
#[derive(des,Debug)]
pub struct authtokengen {
    pub result:String,
    pub token:String,
}
#[derive(des,Debug)]
pub struct authtokenlist {
    pub tokens:Vec<String>,
}
#[derive(des,Debug)]
pub struct authtokenrem {
    pub result:String,
}
