#![no_std]
pub mod syscalls;
pub mod testing;

//nothing needed in this yet
#[no_mangle]
pub extern "C" fn __init_neutron() {}


extern crate neutron_star_rt;

#[panic_handler]
pub fn _neutron_panic_handler(_info: &core::panic::PanicInfo) -> ! {
    use neutron_star_rt::__exit;
    unsafe{
        //return fault + error + revert
        __exit(8 + 1 + 2)
    }
}