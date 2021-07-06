use rust_metasploit::client::Client as client;
struct Test {
    host:String,
    test:String,
}
fn main() {
    let tet=client::new("127.0.0.1","msf","password",55552,true);
    tet.print();
}
