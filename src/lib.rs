#![no_std]
pub mod syscalls;
pub mod testing;

//nothing needed in this yet
#[no_mangle]
pub extern "C" fn __init_qtum() {}
