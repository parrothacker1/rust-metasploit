#![allow(non_camel_case_types)]
use crate::client::Client;
use crate::error::Error as E;
use serde::de::DeserializeOwned as DOwned;
#[path="../blocking/sessions.rs"] mod sessions;

pub async fn list<T:DOwned>(client:Client) -> Result<T,E> {
    sessions::list(client.clone())
}
pub async fn stop<T:DOwned>(client:Client,sessionidstr:&str) -> Result<T,E> {
    sessions::stop(client.clone(),sessionidstr)
}
pub struct shell;
impl shell {
    pub async fn read<T:DOwned>(client:Client,sessionidstr:&str,readpointer:Option<i32>) -> Result<T,E> {
        sessions::shell::read(client.clone(),sessionidstr,readpointer)
    }
    pub async fn write<T:DOwned>(client:Client,sessionidstr:&str,datastr:&str) -> Result<T,E> {
        sessions::shell::write(client.clone(),sessionidstr,datastr)
    }
}
pub struct meterpreter {
    pub sessionid:String,
    pub client:Client,
}
impl meterpreter {
    pub fn new(client:Client,sessionidstr:&str) -> Self {
        meterpreter {
            sessionid:sessionidstr.to_string(),
            client:client,
        }
    }
    pub async fn write<T:DOwned>(&self,datastr:&str) -> Result<T,E> {
        let mtpr=sessions::meterpreter::new(self.client.clone(),&self.sessionid);
        mtpr.write(datastr)
    }
    pub async fn read<T:DOwned>(&self) -> Result<T,E> {
        let mtpr=sessions::meterpreter::new(self.client.clone(),&self.sessionid);
        mtpr.read()
    }
    pub async fn run_single<T:DOwned>(&self,commandstr:&str) -> Result<T,E> {
        let mtpr=sessions::meterpreter::new(self.client.clone(),&self.sessionid);
        mtpr.run_single(commandstr)
    }
    pub async fn script<T:DOwned>(&self,scriptnamestr:&str) -> Result<T,E> {
        let mtpr=sessions::meterpreter::new(self.client.clone(),&self.sessionid);
        mtpr.script(scriptnamestr)
    }
    pub async fn detach_session<T:DOwned>(&self) -> Result<T,E> {
        let mtpr=sessions::meterpreter::new(self.client.clone(),&self.sessionid);
        mtpr.detach_session()
    }
    pub async fn kill_session<T:DOwned>(&self) -> Result<T,E> {
        let mtpr=sessions::meterpreter::new(self.client.clone(),&self.sessionid);
        mtpr.kill_session()
    }
    pub async fn tabs<T:DOwned>(&self,inputlinestr:&str) -> Result<T,E> {
        let mtpr=sessions::meterpreter::new(self.client.clone(),&self.sessionid);
        mtpr.tabs(inputlinestr)
    }
    pub async fn compactible_modules<T:DOwned>(&self) -> Result<T,E> {
        let mtpr=sessions::meterpreter::new(self.client.clone(),&self.sessionid);
        mtpr.compactible_modules()
    }
}
pub async fn shell_upgrade<T:DOwned>(client:Client,sessionidstr:&str,connecthoststr:&str,connectport:i32) -> Result<T,E> {
    sessions::shell_upgrade(client.clone(),sessionidstr,connecthoststr,connectport)
}
pub struct ring {
    client:Client,
    sessionid:String,
}
impl ring {
    pub fn new(client:Client,sessionid:&str) -> Self {
        ring {
            client:client,
            sessionid:sessionid.to_string(),
        }
    }
    pub async fn clear<T:DOwned>(&self) -> Result<T,E> {
        let rng=sessions::ring::new(self.client.clone(),&self.sessionid);
        rng.clear()
    }
    pub async fn last<T:DOwned>(&self) -> Result<T,E> {
        let rng=sessions::ring::new(self.client.clone(),&self.sessionid);
        rng.last()
    }
    pub async fn put<T:DOwned>(&self,datastr:&str) -> Result<T,E> {
        let rng=sessions::ring::new(self.client.clone(),&self.sessionid);
        rng.put(datastr)
    }
}
