use metasploit::{client::Client,msf::plugins};
use std::collections::HashMap;

fn main() {
    let client=Client::new("127.0.0.1",55552,"user","password",true);
    println!("{:?}",plugins::loaded(client.clone()).unwrap());
    let mut hash=HashMap::new();
    hash.insert("Pass".to_string(),"password".to_string());
    hash.insert("ServerPort".to_string(),"75".to_string());
    let pl=plugins::load(client.clone(),"wiki".to_string(),hash);
    if pl.unwrap() {
        println!("Plugin loaded");
    } else {
        println!("Failed to load plugin");
    }
    if plugins::unload(client.clone(),"wiki".to_string()).unwrap() {
        println!("Unloaded plugin");
    } else {
        println!("Failed to unload plugin");
    }
}
