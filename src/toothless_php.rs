use std::ffi::{CStr, CString};
use std::os::raw::c_char;

extern "C" {

    fn init_php();

    fn shutdown_php();

    fn execute_php_script(

    );
}

pub struct PhpInterpreter;

impl PhpInterpreter {

    pub fn new() {

    }
}