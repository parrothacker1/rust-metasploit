use serde_json::value::Value;
#[path="../structs/mod.rs"] mod structs;
use structs::*;
pub struct MsfError {
    error:bool,
    error_class:String,
    error_message:String,
}
pub enum Return_Type {
	Bool(bool),
	String(String),
    Int(i32),
	MsfErr(MsfError),
	Array(Vec<Value>),
    ConsoleCreate(console::create),
    ConsoleRead(console::read),
    CoreModule(core::modules),
}
