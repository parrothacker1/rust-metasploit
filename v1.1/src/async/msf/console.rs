//! A module to handle msfconsole.
#![allow(non_snake_case)]
#[path="../../structs/mod.rs"] mod structs;
#[path="../../error.rs"] mod error;
#[path="../../connect.rs"] mod connect;
use error::MsfError;
use connect::connect;
use structs::{request as req,response as res};
use crate::client::Client;
use serde::{Serialize,Deserialize};
use rmp_serde::{Serializer,Deserializer,decode::{Error as derror,from_read}};

/// To Create a new console shell
///
/// ## Example
/// ```
/// console::create(client.clone()).unwrap(); // response::console::create {}
/// ```
pub async fn create(client:Client) -> Result<res::console::create,MsfError> {
    let mut test:Result<res::console::create,MsfError>=Ok(res::console::create {
        id:String::new(),
        prompt:"".to_string(),
        busy:false,
    });
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::console::create("console.create".to_string(),client.token.as_ref().unwrap().to_string());
    byte.serialize(&mut serializer).unwrap();
    let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    match con {
		Ok(_) => {
			let de_ret:Result<res::console::create,derror>=from_read(new_buf.as_slice());
			if let Ok(ref val) = de_ret {
				test=Ok(val.clone());
			};
			if let Err(_) = de_ret {
				let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
				test=Err(de_ret);
			};
		},
		Err(_) => {
			panic!("Connection closed unexpectedly");
		},
	}
    test
}
/// To kill the existing specified console
///
/// ## Example
/// ```
/// console::destroy(client.clone(),"1").unwrap(); // true
/// ```
pub async fn destroy(client:Client,consoleID:&str) -> Result<bool,MsfError> {
    let consoleid:String=consoleID.to_string();
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
				let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
				test=Err(de_ret);
			};
		},
		Err(_) => {
			panic!("Connection closed unexpectedly");
		},
	}
    test
}
/// To get the list of all consoles
///
/// ## Example
/// ```
/// console::list(client.clone()).unwrap() // response::console::list {}
/// ```
pub fn list(client:Client) -> Result<res::console::list,MsfError> {
	let mut test:Result<res::console::list,MsfError>=Ok(res::console::list {
		consoles:Vec::new(),
	});
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
			let de_ret:Result<res::console::list,derror>=Deserialize::deserialize(&mut de);
			if let Ok(ref val) = de_ret {
                let new=val.clone();
				test=Ok(new);
			};
			if let Err(_) = de_ret {
				let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
				test=Err(de_ret);
			};
		},
		Err(_) => {
			panic!("Closed connection unexpectedly");
		},
	}
	test
}
/// To write a command into the shell.
///
/// It is recommended to add "\n" at the end of command.Or it may not execute
/// ## Example
/// ```
/// console::write(client.clone(),"1","help\n").unwrap() // 1
/// ```
pub fn write(client:Client,consoleID:&str,command:&str) -> Result<i32,MsfError> {
    let data:String=command.to_string();
    let consoleid:String=consoleID.to_string();
    let mut test:Result<i32,MsfError>=Ok(1);
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::console::write("console.write".to_string(),client.token.unwrap(),consoleid,data);
    byte.serialize(&mut serializer).unwrap();
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
				let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
				test=Err(de_ret);
			};
		},
		Err(_) => {
			panic!("Connection closed unexpectedly");
		},
	}
    test
}
/// To read the console
///
/// ## Example
/// ```
/// console::read(client.clone(),"1").unwrap(); // response::console::read {}
/// ```
pub fn read(client:Client,consoleID:&str) -> Result<res::console::read,MsfError> {
    let consoleid:String=consoleID.to_string();
    let mut test:Result<res::console::read,MsfError>=Ok(res::console::read {
        data:String::new(),
        prompt:String::new(),
        busy:true,
    });
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::console::read("console.read".to_string(),client.token.unwrap(),consoleid);
    byte.serialize(&mut serializer).unwrap();
    let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
		Ok(_) => {
			let de_ret:Result<res::console::read,derror>=Deserialize::deserialize(&mut de);
			if let Err(_) = de_ret {
				let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
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
/// To detach the session
///
/// ## Example
/// ```
/// console::detach_session(client.clone(),"1").unwrap(); // true
/// ```
pub fn detach_session(client:Client,consoleID:&str) -> Result<bool,MsfError> {
    let consoleid:String=consoleID.to_string();
    let mut test:Result<bool,MsfError>=Ok(false);
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::console::session_detach("console.session_detach".to_string(),client.token.unwrap(),consoleid);
    byte.serialize(&mut serializer).unwrap();
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
				let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
				test=Err(de_ret);
			};
		},
		Err(_) => {
			panic!("Connection closed unexpectedly");
		},
	}
    test
}
/// To kill the session
///
/// ## Example
/// ```
/// console::kill_session(client.clone(),"1").unwrap(); // true
/// ```
pub fn kill_session(client:Client,consoleID:&str) -> Result<bool,MsfError> {
    let consoleid:String=consoleID.to_string();
    let mut test:Result<bool,MsfError>=Ok(false);
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::console::session_kill("console.session_kill".to_string(),client.token.unwrap(),consoleid);
    byte.serialize(&mut serializer).unwrap();
    let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
		Ok(_) => {
			let de_ret:Result<res::console::session_kill,derror>=Deserialize::deserialize(&mut de);
			if let Err(_) = de_ret {
				let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
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
/// To list all the possible commands which starts with a specific keyword
///
/// ## Example
/// ```
/// console::tabs(client.clone(),"1","hel").unwrap(); // ["help"]
/// ```
pub fn tabs(client:Client,consoleID:&str,inputlinestr:&str) -> Result<Vec<String>,MsfError> {
    let consoleid:String=consoleID.to_string();
    let inputline:String=inputlinestr.to_string();
    let mut test:Result<Vec<String>,MsfError>=Ok(Vec::new());
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::console::tabs("console.tabs".to_string(),client.token.unwrap(),consoleid,inputline);
    byte.serialize(&mut serializer).unwrap();
    let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
		Ok(_) => {
			let de_ret:Result<res::console::tabs,derror>=Deserialize::deserialize(&mut de);
			if let Err(_) = de_ret {
				let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
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
