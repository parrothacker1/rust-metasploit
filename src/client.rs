#[path="./connect.rs"] mod connect;
#[path="./error.rs"] mod error;
use error::conerr;
use rmp_serde::Serializer;
use serde::Serialize;
pub struct Client {
    pub url:String,
    pub token:Option<String>,
}
#[derive(Serialize)]
struct authlogin(String,String,String);
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
        let mut body=Vec::new();
        let mut serializer=Serializer::new(&mut body);
        let byte=authlogin("auth.login".to_string(),new_user,new_pass);
        byte.serialize(&mut serializer).unwrap();
        let data=connect::connect(url.clone(),body);
        match data {
			Ok(val) => {
				if val.get("result").unwrap().as_str().unwrap()=="success" {
					test=Client {
						url:url.clone(),
						token:Some(val.get("token").unwrap().as_str().unwrap().to_string()),
					}
				} else {
                    let ret=conerr::Authfail { user:new_user };
					panic!(ret);
				}
			},
			Err(_e) => {
                let sock=String::from(format!("{}:{}",host,port));
                let ret=conerr::ConnectionNotPossible { socket:sock };
				panic!(ret);
			},
		}
        test
    }
}
