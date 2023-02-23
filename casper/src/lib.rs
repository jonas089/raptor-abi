use casper_types::{CLType};


pub fn test_type_import(){
	let x:CLType = CLType::Unit;
}

/*
pub struct EntryPoint{
    pub ident:String,
    pub args:String,
    pub ret:String,
    pub ty:String
}

pub fn build_entry_point(entrypoint: EntryPoint) -> TokenStream {
   let entrypoint_ident = &entrypoint.ident;
   let params = &entrypoint.args;
   let ret = &entrypoint.ret;
   let access = &entrypoint.ty;
   quote! {
       // declare a new variable
       let test_var = "hello".to_string();

       let params: Vec::from([#entrypoint_ident, #params, #ret, #access])
       // construct the entry point
       // ...
   }
}
*/
