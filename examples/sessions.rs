use metasploit::{client::Client,msf::sessions};

fn main() {
    let client=Client::new("127.0.0.1",55552,"msf","password",false);
    let list=sessions::list(client.clone()).unwrap();
    println!("{:?}",list);
    /*if sessions::stop(client.clone(),"1".to_string()).unwrap() {
        println!("Session stopped");
    } else {
        println!("Failed to stop session");
    }*/
    let mut id:i32=1;
    for (k,v) in list {
        id=k;
    }
    //println!("{:?}",sessions::shell::read(client.clone(),id.to_string(),None).unwrap());
    //panic sometimes
    //println!("{:?}",sessions::shell::write(client.clone(),"2".to_string(),"help".to_string()).unwrap());
    let mut meterpreter=sessions::meterpreter::new(client.clone(),id.to_string());
    meterpreter.write("help\n".to_string()).unwrap();
    if meterpreter.run_single("help\n".to_string()).unwrap() {
        println!("Success");
    } else {
        println!("Failed");
    }
    println!("{:?}",meterpreter.tabs("hel".to_string()).unwrap());
    println!("{:?}",meterpreter.compactible_modules().unwrap());
    println!("{:?}",meterpreter.read().unwrap());
    match sessions::shell_upgrade(client.clone(),id.to_string(),"127.0.0.1".to_string(),8080) {
    	Ok(val) => {
    		println!("{:?}",val);
    	},
    	Err(err) => {
    		println!("{}",err.error_string);
    	},
    }
    let rings=sessions::ring::new(client.clone(),id.to_string());
    println!("{:?}",rings.put("help\n".to_string()));
    println!("{:?}",rings.last());
    println!("{:?}",rings.clear());
}
