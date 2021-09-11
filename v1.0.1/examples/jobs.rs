use metasploit::client::Client;
use metasploit::msf::jobs;

fn main() {
    let client=Client::new("127.0.0.1",55552,"user","password",true);
    let list=jobs::list(client.clone()).unwrap();
    println!("{:?}",list);
    println!("{:?}",jobs::info(client.clone(),"0".to_string()).unwrap());
    if jobs::stop(client.clone(),"0".to_string()).unwrap() {
        println!("Killed job 0");
    } else {
        println!("Cannot kill 0");
    }
}
