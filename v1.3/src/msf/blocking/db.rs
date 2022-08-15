#[path="../../connect.rs"] mod connect;
#[path="../../structs/mod.rs"] mod structs;
use crate::error::{MsfError,Error as E};
use structs::request as req;
use crate::client::Client;
use std::fs::{self,File};
use std::collections::HashMap;
use serde::{Serialize,de::DeserializeOwned as DOwned};
use rmp_serde::{Serializer,decode::Error as derror,from_read};

pub fn hosts<T:DOwned>(client:Client) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::hosts("db.hosts".to_string(),client.token.as_ref().unwrap().to_string());
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body)

}
pub fn get_host<T:DOwned>(client:Client,workspace:Option<str>,host:str) -> Result<T,E>  {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let hash:HashMap<String,String>=HashMap::new();
    hash.insert("host".to_string,host.to_string());
    match workspace {
        Some(val) => {
            hash.insert("workspace".to_string(),val.to_string());
        },
    };
    let byte=req::db::grd_host("db.get_host".to_string(),client.token.as_ref.unwrap().to_string(),hash);
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}
pub fn report_host<T:DOwned>(client:Client,workspace:Option<str>,host:str) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let hash:HashMap<String,String>=HashMap::new();
    hash.insert("host".to_string,host.to_string());
    match workspace {
        Some(val) => {
            hash.insert("workspace".to_string(),val.to_string());
        },
    };
    let byte=req::db::grd_host("db.report_host".to_string(),client.token.as_ref.unwrap().to_string(),hash);
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}
pub fn del_host<T:DOwned>(client:Client,workspace:Option<str>,host:str) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let hash:HashMap<String,String>=HashMap::new();
    hash.insert("host".to_string,host.to_string());
    match workspace {
        Some(val) => {
            hash.insert("workspace".to_string(),val.to_string());
        },
    };
    let byte=req::db::grd_host("db.del_host".to_string(),client.token.as_ref.unwrap().to_string(),hash);
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}
pub fn services<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::nc("db.services".to_string(),client.token.as_ref().unwrap().to_string(),hash);
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}
pub fn report_service<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::grd_service("db.report_service".to_string(),client.token.as_ref().unwrap().to_string(),hash);
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}
pub fn get_service<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::grd_service("db.get_service".to_string(),client.token.as_ref().unwrap().to_string(),hash);
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}
pub fn del_service<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::grd_service("db.del_service".to_string(),client.token.as_ref().unwrap().to_string(),hash);
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}

pub fn vulns<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::nc("db.vulns".to_string(),client.token.as_ref().unwrap().to_string(),hash);
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}
pub fn del_vuln<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::grd_vuln("db.del_vuln".to_string(),client.token.as_ref().unwrap().to_string(),hash);
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}
pub fn report_vuln<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::grd_vuln("db.report_vuln".to_string(),client.token.as_ref().unwrap().to_string(),hash);
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}
pub fn get_vuln<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::grd_vuln("db.get_vuln".to_string(),client.token.as_ref().unwrap().to_string(),hash);
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}

pub fn workspaces<T:DOwned>(client:Client) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::workspaces("db.workspaces".to_string(),client.token.as_ref().unwrap().to_string());
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}
pub fn current_workspace<T:DOwned>(client:Client) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::current_workspacs("db.current_workspace".to_string(),client.token.as_ref().unwrap().to_string());
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}
pub fn get_workspace<T:DOwned>(client:Client,workspace:str) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::gsda_workspace("db.get_workspace".to_string(),client.token.as_ref().unwrap().to_string(),workspace.to_string());
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}
pub fn set_workspace<T:DOwned>(client:Client,workspace:str) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::gsda_workspace("db.set_workspace".to_string(),client.token.as_ref().unwrap().to_string(),workspace.to_string());
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}
pub fn del_workspace<T:DOwned>(client:Client,workspace:str) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::gsda_workspace("db.del_workspace".to_string(),client.token.as_ref().unwrap().to_string(),workspace.to_string());
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}
pub fn add_workspace<T:DOwned>(client:Client,workspace:str) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::gsda_workspace("db.add_workspace".to_string(),client.token.as_ref().unwrap().to_string(),workspace.to_string());
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}

pub fn get_note<T:DOwned>(client:Client,hash:HashMap<String,String> -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::grd_note("db.get_note".to_string(),client.token.as_ref().unwrap().to_string(),hash);
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}
pub fn report_note<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::grd_note("db.report_note".to_string(),client.token.as_ref().unwrap().to_string(),hash);
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}
pub fn notes<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::nc("db.notes".to_string(),client.token.as_ref().unwrap().to_string(),hash);
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}
pub fn del_note<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::grd_note("db.del_note".to_string(),client.token.as_ref().unwrap().to_string(),hash);
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}

pub fn get_client<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::grd_client("db.get_client".to_string(),client.token.as_ref().unwrap().to_string(),hash);
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}
pub fn clients<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::nc("db.clients".to_string(),client.token.as_ref().unwrap().to_string(),hash);
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}
pub fn del_client<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::grd_client("db.del_client".to_string(),client.token.as_ref().unwrap().to_string(),hash);
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}
pub fn report_client<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::grd_client("db.report_client".to_string(),client.token.as_ref().unwrap().to_string(),hash);
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}

pub fn get_ref<T:DOwned>(client:Client,ref_name:str) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::get_ref("db.get_ref".to_string(),client.token.as_ref().unwrap().to_string(),ref_name.to_string());
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}

pub fn events<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::nc("db.events".to_string(),client.token.as_ref().unwrap().to_string(),hash);
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}
pub fn report_event<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::report_event("db.report_event".to_string(),client.token.as_ref().unwrap().to_string(),hash);
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}

pub fn report_loot<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::report_loot("db.report_loot".to_string(),client.token.as_ref().unwrap().to_string(),hash);
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}
pub fn loots<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::nc("db.loots".to_string(),client.token.as_ref().unwrap().to_string(),hash);
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}

pub fn creds<T:DOwned>(client:Client,hash:HashMap<String,String>) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::nc("db.creds".to_string(),client.token.as_ref().unwrap().to_string(),hash);
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}

pub fn import_data<T:DOwned>(client:Client,data:str) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::import_db("db.import_data".to_string(),client.token.as_ref().unwrap().to_string(),data.to_string());
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}
pub fn import_file<T:DOwned>(client:Client,file:File) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let data=fs::read_to_string(file).unwrap();
    let byte=req::db::import_db("db.import_data".to_string(),client.token.as_ref().unwrap().to_string(),data);
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}
pub fn set_driver<T:DOwned>(client:Client,driver:str) -> Result<T,E> {
    let mut body=Vec::new();
    let hash:HashMap<String,String>=HashMap::new();
    let mut serializer=Serializer::new(&mut body);
    hash.insert("driver".to_string(),driver.to_string());
    let byte=req::db::driver("db.driver".to_string(),client.token.as_ref().unwrap().to_string(),hash);
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}
pub fn get_driver<T:DOwned>(client:Client) -> Result<T,E> {
    let mut body=Vec::new();
    let hash:HashMap<String,String>=HashMap::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::driver("db.driver".to_string(),client.token.as_ref().unwrap().to_string(),hash);
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}

pub fn connect<T:DOwned>(client:Client,driver:str,hash:HashMap<String,String>) -> Result<T,E> {
    let mut body=Vec::new();
    hash.insert("driver".to_string(),driver.to_string());
    let mut serialzier=Serializer::new(&mut body);
    let byte=req::db::connect("db.connect".to_string(),client.token.as_ref().unwrap().to_string(),hash);
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}

pub fn status<T:DOwned>(client:Client) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::status("db.status".to_string(),client.token.as_ref().unwrap().to_string());
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}

pub fn disconnect<T:DOwned>(client:Client) -> Result<T,E> {
    let mut body=Vec::new();
    let mut serializer=Serializer::new(&mut body);
    let byte=req::db::disconnect("db.disconnect".to_string(),client.token.as_ref().unwrap().to_string());
    byte.serialize(&mut serializer).unwrap();
    r#return(client.url,body);
}

fn r#return<T:DOwned>(url:String,body:Vec<u8>) -> Result<T,E> {
    let mut buf=vec![];
    let con=connect::connect(url,body,&mut buf);
    let new_buf=buf.clone();
    match con {
        Ok(_) => {
            let ret:Result<T,derror>=from_read(new_buf.as_slice());
            match ret {
                Ok(val) => {
                    Ok(val)
                },
                Err(_) => {
                    let ret2:Result<MsfError,derror>=from_read(new_buf.as_slice());
                    match ret2 {
                        Ok(e) => {
                            Err(E::MsfError(e))
                        },
                        Err(e) => {
                            Err(E::DError(e))
                        },
                    }
                },
            }
        },
        Err(e) => {
            Err(E::ConnectionError(e))
        },
    }
}
