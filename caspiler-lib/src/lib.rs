use casper_types::{Parameter, CLType, EntryPointAccess, EntryPointType, EntryPoint};
use code_generator::EpMacro;
use std::fmt;
use std::string::ToString;

#[derive(EpMacro)]
pub struct EntryPoint2{
    pub name: String,
	pub args: Vec<Parameter>,
    pub ret: CLType,
    pub access: EntryPointAccess,
    pub tp: EntryPointType
}
impl fmt::Display for EntryPoint2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {} Args:{:?} Ret:{:?} Access:{:?} Type:{:?}", self.name, self.args, self.ret, self.access, self.tp)
    }
}
