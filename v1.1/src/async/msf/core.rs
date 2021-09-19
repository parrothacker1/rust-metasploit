//! A module which is used to handle msfcore
#![allow(non_snake_case)]
#[path="../../structs/mod.rs"] mod structs;
#[path="../../connect.rs"] mod connect;
#[path="../../error.rs"] mod error;
use error::MsfError;
use crate::client::Client;
use std::collections::HashMap;
use connect::connect;
use serde::{Serialize,Deserialize};
use rmp_serde::{Serializer,Deserializer,decode::{Error as derror,from_read}};
use structs::{request as req,response as res};

/// To add a new module by path
///
/// ## Example
/// ```
/// core::add_module(client.clone(),"path").await.unwrap(); // response::core::addmodpath {}
/// ```
pub async fn add_module(client:Client,pathstr:&str) -> Result<res::core::addmodpath,MsfError> {
    let path:String=pathstr.to_string();
    let mut test:Result<res::core::addmodpath,MsfError>=Ok(res::core::addmodpath {
        exploits:0,
        auxiliary:0,
        post:0,
        encoders:0,
        nops:0,
        payloads:0,
    });
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut serializer=Serializer::new(&mut body);
    let byte=req::core::addmodpath("core.add_module_path".to_string(),client.token.unwrap(),path);
    byte.serialize(&mut serializer).unwrap();
    let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
        Ok(_) => {
            let de_ret:Result<res::core::addmodpath,derror>=Deserialize::deserialize(&mut de);
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
/// To get the status of modules loaded
///
/// ## Example
/// ```
/// core::module_status(client.clone()).await.unwrap(); // response::core::modulestat
/// ```
pub async fn module_status(client:Client) -> Result<res::core::modulestat,MsfError> {
    let mut test:Result<res::core::modulestat,MsfError>=Ok(res::core::modulestat{
        exploits:0,
        auxiliary:0,
        post:0,
        encoders:0,
        nops:0,
        payloads:0,
    });
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::core::modulestat("core.module_stats".to_string(),client.token.unwrap());
    byte.serialize(&mut se).unwrap();
    let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
        Ok(_) => {
            let de_ret:Result<res::core::modulestat,derror>=Deserialize::deserialize(&mut de);
            if let Err(_) = de_ret {
                let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
                test=Err(de_ret);
            };
            if let Ok(ref val) = de_ret {
                let new=val.clone();
                test=Ok(new);
            };
        },
        Err(_) => {
            panic!("Connection closed unexpectedly");
        },
    }
    test
}
/// To reload all the modules
///
/// ## Example
/// ```
/// core::reload_modules(client.clone()).await.unwrap(); // response::core::reloadmod {}
/// ```
pub async fn reload_module(client:Client) -> Result<res::core::reloadmod,MsfError> {
    let mut test:Result<res::core::reloadmod,MsfError>=Ok(res::core::reloadmod {
        exploits:0,
        auxiliary:0,
        post:0,
        encoders:0,
        nops:0,
        payloads:0,
    });
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::core::reloadmod("core.reload_modules".to_string(),client.token.unwrap());
    byte.serialize(&mut se).unwrap();
    let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
        Ok(_) => { 
            let de_ret:Result<res::core::reloadmod,derror>=Deserialize::deserialize(&mut de);
            if let Err(_) = de_ret { 
                let de_ret:MsfError=from_read(new_buf.as_slice()).unwrap();
                test=Err(de_ret);
            };
            if let Ok(ref val) = de_ret {
                let new=val.clone();
                test=Ok(new);
            };
        },
        Err(_) => {
            panic!("Connection closed unexpectedly");
        },
    }
    test
}
/// To save in the core
///
/// ## Example
/// ```
/// core::save(client.clone()).await.unwrap(); // true
/// ```
pub async fn save(client:Client) -> Result<bool,MsfError> {
    let mut test:Result<bool,MsfError>=Ok(false);
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::core::save("core.save".to_string(),client.token.unwrap());
    byte.serialize(&mut se).unwrap();
    let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
        Err(_) => {
            panic!("Connection closed unexpectedly");
        },
        Ok(_) => {
            let de_ret:Result<res::core::save,derror>=Deserialize::deserialize(&mut de);
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
        }
    }
    test
}
/// To set setg with key value pair
///
/// ## Example
/// ```
/// core::setg(client.clone(),"name","value").await.unwrap(); //true
/// ```
pub async fn setg(client:Client,namestr:&str,valuestr:&str) -> Result<bool,MsfError> {
    let name:String=namestr.to_string();
    let value:String=valuestr.to_string();
    let mut test:Result<bool,MsfError>=Ok(false);
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::core::setg("core.setg".to_string(),client.token.unwrap(),name,value);
    byte.serialize(&mut se).unwrap();
    let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
		Ok(_) => {
			let de_ret:Result<res::core::setg,derror>=Deserialize::deserialize(&mut de);
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
/// To remove setg with key name
///
/// ## Example
/// ```
/// core::unsetg(client.clone(),"name").await.unwrap(); // true
/// ```
pub async fn unsetg(client:Client,namestr:&str) -> Result<bool,MsfError> {
    let name:String=namestr.to_string();
    let mut test:Result<bool,MsfError>=Ok(true);
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::core::unsetg("core.unsetg".to_string(),client.token.unwrap(),name);
    byte.serialize(&mut se).unwrap();
    let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
		Err(_) => {
			panic!("Connection closed unexpectedly");
		},
		Ok(_) => {
			let de_ret:Result<res::core::unsetg,derror>=Deserialize::deserialize(&mut de);
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
	}
    test
}
/// To list all the threads
///
/// ## Example
/// ```
/// core::list_thread(client.clone()).await.unwrap(); // {1,response::core::threadlist {}}
/// ```
pub async fn list_thread(client:Client) -> Result<HashMap<i32,res::core::threadlist>,MsfError> {
	let mut test:Result<HashMap<i32,res::core::threadlist>,MsfError>=Ok(HashMap::new());
	let mut body=Vec::new();
	let mut buf=vec![];
	let mut se=Serializer::new(&mut body);
	let byte=req::core::threadlist("core.thread_list".to_string(),client.token.unwrap());
	byte.serialize(&mut se).unwrap();
	let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
	match con {
		Err(_) => {
			panic!("Connection closed unexpectedly");
		},
		Ok(_) => {
			let de_ret:Result<HashMap<i32,res::core::threadlist>,derror>=Deserialize::deserialize(&mut de);
			if let Err(_) = de_ret {
				let de_ret:MsfError=Deserialize::deserialize(&mut de).unwrap();
				test=Err(de_ret);
			};
			if let Ok(ref val) = de_ret {
				test=Ok(val.clone());
			};
		},
	}
	test
}
/// To kill a thread
///
/// ## Example
/// ```
/// core::kill_thread(client.clone(),1).await.unwrap(); // true
/// ```
pub async fn kill_thread(client:Client,threadID:i32) -> Result<bool,MsfError> {
    let mut test:Result<bool,MsfError>=Ok(false);
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::core::threadkill("core.thread_kill".to_string(),client.token.unwrap(),threadID);
    byte.serialize(&mut se).unwrap();
    let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
		Err(_) => {
			panic!("Connection closed unexpectedly");
		},
		Ok(_) => {
			let de_ret:Result<res::core::threadkill,derror>=Deserialize::deserialize(&mut de);
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
	}
    test
}
/// To get the version
///
/// ## Example
/// ```
/// core::version(client.clone()).await.unwrap(); //true
/// ```
pub async fn version(client:Client) -> Result<res::core::version,MsfError> {
    let mut test:Result<res::core::version,MsfError>=Ok(res::core::version {
        version:String::new(),
        api:String::new(),
        ruby:String::new(),
    });
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::core::version("core.version".to_string(),client.token.unwrap());
    byte.serialize(&mut se).unwrap();
    let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
        Ok(_) => {
            let de_ret:Result<res::core::version,derror>=Deserialize::deserialize(&mut de);
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
/// To stop the core
///
/// ## Example
/// ```
/// core::stop(client.clone()).await.unwrap(); // true
/// ```
pub async fn stop(client:Client) -> Result<bool,MsfError> {
    let mut test:Result<bool,MsfError>=Ok(false);
    let mut body=Vec::new();
    let mut buf=vec![];
    let mut se=Serializer::new(&mut body);
    let byte=req::core::stop("core.stop".to_string(),client.token.unwrap());
    byte.serialize(&mut se).unwrap();
    let con=connect(client.url,body,&mut buf);
    let new_buf=buf.clone();
    let mut de=Deserializer::new(new_buf.as_slice());
    match con {
        Ok(_) => {
            let de_ret:Result<res::core::stop,derror>=Deserialize::deserialize(&mut de);
            if let Err(_) = de_ret {
                let de_ret:MsfError=Deserialize::deserialize(&mut de).unwrap();
                test=Err(de_ret);
            };
            if let Ok(val) = de_ret {
                if val.result=="success".to_string() {
                    test=Ok(true);
                } else {
                    test=Ok(false);
                }
            };
        },
        Err(_) => {
            panic!("Connection closed unexpectedly");
        }
    }
    test
}