use metasploit::client::Client as client;
use metasploit::msf::auth;
struct Test {
    host:String,
    test:String,
}
fn main() {
    let tet=client::new("127.0.0.1","msf","password",55552,false);
//    tet.print();
}
