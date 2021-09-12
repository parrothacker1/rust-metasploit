use metasploit::client::Client;
use metasploit::msf::core;

fn main() {
	let client=Client::new("127.0.0.1",55552,"user","password",true);
    println!("{:?}",core::module_status(client.clone()).unwrap());
    println!("{:?}",core::reload_module(client.clone()).unwrap());
    if core::save(client.clone()).unwrap() {
        println!("All saved");
    } else {
        println!("Failed to save");
    }
    if core::setg(client.clone(),"test","val").unwrap() {
        println!("Saved test:val");
    } else {
        println!("Failed to save");
    }
    if core::unsetg(client.clone(),"test").unwrap() {
        println!("Removed test:val");
    } else {
        println!("Failed to remove test");
    }
    let threadlist=core::list_thread(client.clone()).unwrap();
    println!("{:?}",threadlist);
    println!("{:?}",core::version(client.clone()).unwrap());
    if core::stop(client.clone()).unwrap() {
        println!("Stopped core");
    } else {
        println!("Failed to stop core");
    }
    if core::kill_thread(client.clone(),0).unwrap() {
        println!("Killed thread");
    } else {
        println!("Cannot kill thread");
    } 
}
