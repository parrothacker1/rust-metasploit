use metasploit::client::Client as client;
use metasploit::msf::*;

fn main() {
    let tet=client::new("127.0.0.1","user","password",55552,true);
    tet.print();
    let test=auth::token_list(tet.clone()).unwrap();
    println!("{:?}",test);
    let test1=core::version(tet.clone()).unwrap();
    println!("{:?}",test1);
    let test2=auth::logout(tet.clone(),test[0].clone()).unwrap();
    println!("{:?}",test2);
    println!("{}",auth::logout(tet.clone(),test[0].clone()).unwrap());
}
