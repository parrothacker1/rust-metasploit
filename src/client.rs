#[path="./connect.rs"] mod connect;
#[path="./structs/mod.rs"] mod structs;
use structs::{request::auth::login,response::auth::login as reslogin};
use rmp_serde::{Serializer,Deserializer,decode::Error};
use serde::{Serialize,Deserialize};
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
        let mut body=Vec::new();
        let mut buf=vec![];
        let mut serializer=Serializer::new(&mut body);
        let byte=login("auth.login".to_string(),new_user.clone(),new_pass);
        byte.serialize(&mut serializer);
        let con=connect::connect(url.clone(),body,&mut buf);
        let mut de=Deserializer::new(buf.as_slice());
        let mut token=String::new();
        match con {
			Ok(_) => {
				let de_ret:Result<reslogin,Error>=Deserialize::deserialize(&mut de);
				if let Ok(ref val) = de_ret {
					if val.result=="success".to_string() {
						token=val.token.clone();
					} else {
						panic!("Not authorised.Username or password is wrong");
					}
				}
				if let Err(_) = de_ret {
					panic!("Not authorised.Username or password is wrong");
				}
			},
			Err(_) => {
				panic!("Couldn't connect to the metasploit RPC Server at {}:{}",host,port);
			},
		}
		Client {
			url:url.clone(),
			token:Some(token.clone()),
		}
    }
    pub fn print(self) {
        println!("{:?}",self.token)
    }
}
