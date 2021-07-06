use rs_metasploit::test;
struct Test {
    host:String,
    test:String,
}
fn main() {
    let tet=test::test1("127.0.0.1".to_string(),"testibg".to_string());
    tet.print();
}
