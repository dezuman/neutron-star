
#![no_std]

extern crate neutron_star_rt;
pub mod syscalls;
#[macro_use]
pub mod testing;
pub mod logging;
pub mod storage;

#[no_mangle]
pub extern "C" fn __init_neutron() {
}
/*
#[panic_handler]
pub fn _neutron_panic_handler(info: &core::panic::PanicInfo) -> ! {
    use neutron_star_rt::__revert_execution;
    unsafe{
        //return fault + error + revert
        let m = info.message();
        println!("CONTRACT PANIC!");
        if let Some(location) = info.location() {
            println!("panic occurred in file '{}' at line {}", location.file(), location.line());
        } else {
            println!("panic occurred but can't get location information...");
        }
        if m.is_some(){
            println!("{}", *m.unwrap());
        }else{
            println!("Contract panic! no reason given");
        }
        __revert_execution(8 + 1 + 2);
    }
}
*/