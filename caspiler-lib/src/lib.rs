use casper_types::{Parameter, CLType, EntryPointAccess, EntryPointType, EntryPoint};
use code_generator::{EpMacro};
use std::fmt;
use std::string::ToString;
// Note:
// Temporary use of a new struct EntryPoint2.
// This might be reverted, if the macro can be used on EntryPoint(1) as expected.
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

/*
Function:
What could be achieved with functional macro?
Function takes args (String, Vec, CLType, ...)
Need to find a way to extract these args

Struct:
Instance of struct -> attribute:vale, attribute:value...
Need to find a wat to extract values by paired attribute.

- can extract access them through an impl that is added at compiletime,
but that's the wrong scope.
*/
