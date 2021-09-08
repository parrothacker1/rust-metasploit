use metasploit::client::Client;
use metasploit::msf::auth;

fn main() {
	let client=Client::new("127.0.0.1",55552,"user","password",true);
	if auth::token_add(client.clone(),"newtoken".to_string()).unwrap() {
		println!("New token added");
	} else {
		println!("Failed to add new token");
	}
	println!("{}",auth::token_generate(client.clone()).unwrap());
	let tokenlist=auth::token_list(client.clone()).unwrap();
	println!("The list of tokens are: {:?}",tokenlist);
	if auth::token_remove(client.clone(),"newtoken".to_string()).unwrap() {
		println!("Removed a token");
	} else {
		println!("Failed to remove token");
	}
	auth::logout(client.clone(),client.gettoken()).unwrap();
	println!("logged out of the RPC Server ");
}