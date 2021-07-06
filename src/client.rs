#[path="./connect.rs"] mod connect;

pub struct Client {
    pub url:String,
    pub token:Option<String>,
}

impl Client {
    pub fn new(host:&str,user:&str,password:&str,port:i32,ssl:bool) -> Self {
        let new_host=String::from(host);
        let new_user=String::from(user);
        let url:String;
        let new_pass=String::from(password);
        if ssl {
            url=format!("https://{}:{}/api",new_host,port).to_string()
        } else {
            url=format!("http://{}:{}/api",new_host,port).to_string()
        };
        let token=connect::auth(url.clone(),new_user,new_pass);
        Client {
            url:url,
            token:None,
        }
    }
    pub fn print(&self) {
        println!("{}",self.url);
    }
}
