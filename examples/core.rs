use metasploit::client::Client;
use metasploit::msf::core;

fn main() {
	let client=Client::new("127.0.0.1",55552,"user","password",true);
	
}