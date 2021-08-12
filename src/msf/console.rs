#[path="../structs/mod.rs"] mod structs;
#[path="../error.rs"] mod error;
#[path="../connect.rs"] mod connect;
use error::MsfError;
use connect::connect;
use structs::{request as req,response as res};
use crate::client::Client;
use std::collections::HashMap;
use serde::{Serialize,Deserialize};
use rmp_serde::{Serializer,Deserializer,decode::Error as derror};

pub fn create(client:Client) -> Result<res::console::create,MsfError> {
    let mut test:Result<res::console::create,MsfError>=Ok(res::console::create {
        id:1,
        prompt:"".to_string(),
        busy:false,
    });
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::console::create("console.create".to_string(),client.token.as_ref().unwrap().to_string());
    byte.serialize(&mut serializer);
    let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
		Ok(_) => {
			let de_ret:Result<res::console::create,derror>=Deserialize::deserialize(&mut de);
			if let Ok(ref val) = de_ret {
				test=Ok(val.clone());
			};
			if let Err(_) = de_ret {
				let de_ret:MsfError=Deserialize::deserialize(&mut de).unwrap();
				test=Err(de_ret);
			};
		},
		Err(_) => {
			panic!("Connection closed unexpectedly");
		},
	}
    test
}
pub fn destroy(client:Client,consoleid:String) -> Result<bool,MsfError> {
    let mut test:Result<bool,MsfError>=Ok(false);
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::console::destroy("console.destroy".to_string(),client.token.unwrap(),consoleid);
    byte.serialize(&mut serializer).unwrap();
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    let con=connect(client.url,body,&mut buf);
    match con {
		Ok(_) => {
			let de_ret:Result<res::console::destroy,derror>=Deserialize::deserialize(&mut de);
			if let Ok(ref val) = de_ret {
				if val.result=="success".to_string() {
					test=Ok(true);
				} else {
					test=Ok(false);
				}
			};
			if let Err(_) = de_ret {
				let de_ret:MsfError=Deserialize::deserialize(&mut de).unwrap();
				test=Err(de_ret);
			};
		},
		Err(_) => {
			panic!("Connection closed unexpectedly");
		},
	}
    test
}
pub fn list(client:Client) -> Result<HashMap<String,res::console::list>,MsfError> {
	let mut test:Result<HashMap<String,res::console::list>,MsfError>=Ok(HashMap::new());
	let mut body=Vec::new();
	let mut buf=vec![];
	let mut serializer=Serializer::new(&mut body);
	let byte=req::console::list("console.list".to_string(),client.token.unwrap());
	byte.serialize(&mut serializer).unwrap();
	let con=connect(client.url,body,&mut buf);
	let new_buf=buf.clone();
	let mut de=Deserializer::new(new_buf.as_slice());
	match con {
		Ok(_) => {
			let new_de:HashMap<String,res::console::list>=Deserialize::deserialize(&mut de).unwrap();
			let de_ret:Result<HashMap<String,res::console::list>,derror>=Deserialize::deserialize(&mut de);
			if let Ok(ref val) = de_ret {
                let new=val.clone();
				test=Ok(new);
			};
			if let Err(_) = de_ret {
				let de_ret:MsfError=Deserialize::deserialize(&mut de).unwrap();
				test=Err(de_ret);
			};
		},
		Err(_) => {
			panic!("Closed connection unexpectedly");
		},
	}
	test
}
pub fn write(client:Client,consoleid:String,data:String) -> Result<i32,MsfError> {
    let mut test:Result<i32,MsfError>=Ok(1);
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::console::write("console.write".to_string(),client.token.unwrap(),consoleid,data);
    byte.serialize(&mut serializer);
    let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
		Ok(_) => {
			let de_ret:Result<res::console::write,derror>=Deserialize::deserialize(&mut de);
			if let Ok(ref val) = de_ret {
				test=Ok(val.wrote);
			};
			if let Err(_) = de_ret {
				let de_ret:MsfError=Deserialize::deserialize(&mut de).unwrap();
				test=Err(de_ret);
			};
		},
		Err(_) => {
			panic!("Connection closed unexpectedly");
		},
	}
    test
}
pub fn read(client:Client,consoleid:String) -> Result<res::console::read,MsfError> {
    let mut test:Result<res::console::read,MsfError>=Ok(res::console::read {
        data:String::new(),
        prompt:String::new(),
        busy:true,
    });
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::console::read("console.read".to_string(),client.token.unwrap(),consoleid);
    byte.serialize(&mut serializer);
    let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
		Ok(_) => {
			let de_ret:Result<res::console::read,derror>=Deserialize::deserialize(&mut de);
			if let Err(_) = de_ret {
				let de_ret:MsfError=Deserialize::deserialize(&mut de).unwrap();
				test=Err(de_ret);
			};
			if let Ok(ref val) = de_ret {
				test=Ok(val.clone());
			};
		},
		Err(_) => {
			panic!("Connection closed unexpectedly");
		},
	}
    test
}
pub fn session_detach(client:Client,consoleid:String) -> Result<bool,MsfError> {
    let mut test:Result<bool,MsfError>=Ok(false);
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::console::session_detach("console.session_detach".to_string(),client.token.unwrap(),consoleid);
    byte.serialize(&mut serializer);
    let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
		Ok(_) => {
			let de_ret:Result<res::console::session_detach,derror>=Deserialize::deserialize(&mut de);
			if let Ok(ref val) = de_ret {
				if val.result=="success".to_string() {
					test=Ok(true);
				} else {
					test=Ok(false);
				}
			};
			if let Err(_) = de_ret {
				let de_ret:MsfError=Deserialize::deserialize(&mut de).unwrap();
				test=Err(de_ret);
			};
		},
		Err(_) => {
			panic!("Connection closed unexpectedly");
		},
	}
    test
}
pub fn session_kill(client:Client,consoleid:String) -> Result<bool,MsfError> {
    let mut test:Result<bool,MsfError>=Ok(false);
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::console::session_kill("console.session_kill".to_string(),client.token.unwrap(),consoleid);
    byte.serialize(&mut serializer);
    let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
		Ok(_) => {
			let de_ret:Result<res::console::session_kill,derror>=Deserialize::deserialize(&mut de);
			if let Err(_) = de_ret {
				let de_ret:MsfError=Deserialize::deserialize(&mut de).unwrap();
				test=Err(de_ret);
			};
			if let Ok(ref val) = de_ret {
				if val.result=="success".to_string() {
					test=Ok(true);
				} else {
					test=Ok(false);
				}
			};
		},
		Err(_) => {
			panic!("Connection closed unexpectedly");
		},
	}
    test
}
pub fn tabs(client:Client,consoleid:String,inputline:String) -> Result<Vec<String>,MsfError> {
    let mut test:Result<Vec<String>,MsfError>=Ok(Vec::new());
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::console::tabs("console.tabs".to_string(),client.token.unwrap(),consoleid,inputline);
    byte.serialize(&mut serializer);
    let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
		Ok(_) => {
			let de_ret:Result<res::console::tabs,derror>=Deserialize::deserialize(&mut de);
			if let Err(_) = de_ret {
				let de_ret:MsfError=Deserialize::deserialize(&mut de).unwrap();
				test=Err(de_ret);
			};
			if let Ok(ref val) = de_ret {
				test=Ok(val.tabs.clone());
			};
		},
		Err(_) => {
			panic!("Connection closed unexpectedly");
		},
	}
    test
}
