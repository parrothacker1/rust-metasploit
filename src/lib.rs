use reqwest::Client;
use rmp_serde;

pub struct test {
    host:String,
    test:String,
}

impl test {
    pub fn test1(host: String,test:String) -> Self {
        test { 
            host:host,
            test:test,
        } 
    }
    pub fn print(&self) {
        println!("{}",self.test);
    }
}
