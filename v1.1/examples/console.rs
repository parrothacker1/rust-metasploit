use metasploit::client::Client;
use metasploit::msf::console;

fn main() {
	let client=Client::new("127.0.0.1",55552,"user","password",true);
	let new_console=console::create(client.clone()).unwrap();
	println!("{:?}",new_console);
	let consolelist=console::list(client.clone()).unwrap();
	println!("The list of consoles are {:?}",consolelist);
	console::write(client.clone(),new_console.id.clone(),"use exploit/multi/handler\n".to_string()).unwrap();
	println!("{:?}",console::read(client.clone(),new_console.id.clone()));
	println!("{:?}",console::tabs(client.clone(),new_console.id.clone(),"ver".to_string()).unwrap());
	if console::session_detach(client.clone(),new_console.id.clone()).unwrap() {
		println!("Detached");
	} else {
		println!("Failed to detach");
	}
	if console::session_kill(client.clone(),new_console.id.clone()).unwrap() {
		println!("Session killed");
	} else {
		println!("Failed to kill session");
	}
}