
#![feature(panic_info_message)]
#![feature(alloc)]
#![feature(alloc_error_handler)]
#![no_std]

extern crate linked_list_allocator;
extern crate neutron_star_rt;
#[macro_use]
extern crate alloc;
//extern crate alloc;
pub mod syscalls;
pub mod testing;
pub mod logging;
use core::alloc::Layout;

//nothing needed in this yet
#[no_mangle]
pub extern "C" fn __init_neutron() {}
use linked_list_allocator::LockedHeap;

pub fn init_heap() {
    let heap_start = 0x80020000;
    let heap_end = 0x8002FFFF;
    let heap_size = heap_end - heap_start;
    unsafe {
        ALLOCATOR.lock().init(heap_start, heap_size);
    }
}

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

// define what happens in an Out Of Memory (OOM) condition
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    unsafe{
        neutron_star_rt::__revert_execution(0);
    }

    loop {}
}


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