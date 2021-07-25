#[path="./connect.rs"] mod connect;
pub struct Client {
    pub url:String,
    pub token:Option<String>,
}
impl Client {
    pub fn new(host:&str,user:&str,password:&str,port:i32,ssl:bool) -> Self {
        let new_user=String::from(user);
        let url:String;
        let new_pass=String::from(password);
        if ssl {
            url=format!("https://{}:{}/api",host,port).to_string()
        } else {
            url=format!("http://{}:{}/api",host,port).to_string()
        };
        connect::connect(url);
        Client {
            url:url,
            token:None,
        }
    }
    pub fn print(self) {
        println!("{:?}",self.token)
    }
}
