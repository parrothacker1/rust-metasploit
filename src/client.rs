#[path="./connect.rs"] mod connect;
#[path="./error.rs"] mod error;
use error::conerr;
use connect::Parse_Type as PType;
pub struct Client {
    pub url:String,
    pub token:Option<String>,
}
impl Client {
    pub fn new(host:&str,user:&str,password:&str,port:i32,ssl:bool) -> Self {
		let test:Client;
        let new_user=String::from(user);
        let url:String;
        let new_pass=String::from(password);
        if ssl {
            url=format!("https://{}:{}/api",host,port).to_string()
        } else {
            url=format!("http://{}:{}/api",host,port).to_string()
        };
        let body=vec![PType::String("auth.login".to_string()),PType::String(new_user),PType::String(new_pass)];
        let data=connect::connect(url.clone(),body);
        match data {
			Ok(val) => {
				if val.get("result").unwrap().as_str().unwrap()=="success" {
					test=Client {
						url:url.clone(),
						token:val.get("token").unwrap().as_str().unwrap(),
					}
				} else {
					panic!("Auth failed");
				}
			},
			Err(_e) => {
				panic!("Not possible");
			},
		}
        test
    }
    pub fn print(self) {
        println!("{:?}",self.token)
    }
}
