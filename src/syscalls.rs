//! This is a collection of unsafe wrapper functions for raw system calls
//! These should only be used if the abstracted safe functions for some reason
//! do not meet your needs.
//! These functions only do the basic methods of abstraction, such as returning results and pairs
//! instead of taking mutable pointers for integers

extern crate neutron_star_rt;
extern crate neutron_star_constants;

use neutron_star_rt::*;
use neutron_star_constants::*;

pub enum SyscallError{
    Error(u32)
}

pub unsafe fn push_sccs(data: *const u8, size: u32) -> Result<(), SyscallError>{
    let e = __neutron_syscall_short(NeutronSyscalls::PushSCCS as u32, data as u32, size,0);
    if e != 0{
        Err(SyscallError::Error(e))
    }else{
        Ok(())
    }
}

pub unsafe fn pop_sccs(buffer: *mut u8, size: u32) -> Result<u32, SyscallError>{
    let mut actual:u32 = 0;
    let actual_pointer: *mut u32 = &mut actual;
    let e = __neutron_syscall_short(NeutronSyscalls::PopSCCS as u32, buffer as u32, size, actual_pointer as u32);
    if e != 0{
        Err(SyscallError::Error(e))
    }else{
        Ok(actual)
    }
}



