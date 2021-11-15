use metasploit::client::Client;
use metasploit::msf::auth;

fn main() {
	let client=Client::new("127.0.0.1",55552,"user","password",true);
	if auth::add_token(client.clone(),"newtoken").unwrap() {
		println!("New token added");
	} else {
		println!("Failed to add new token");
	}
	println!("{}",auth::generate_token(client.clone()).unwrap());
	let tokenlist=auth::list_token(client.clone()).unwrap();
	println!("The list of tokens are: {:?}",tokenlist);
	if auth::remove_token(client.clone(),"newtoken").unwrap() {
		println!("Removed a token");
	} else {
		println!("Failed to remove token");
	}
	auth::logout(client.clone()).unwrap();
	println!("logged out of the RPC Server ");
}
