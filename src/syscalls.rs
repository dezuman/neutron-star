//! This is a collection of unsafe wrapper functions for raw system calls
//! These should only be used if the abstracted safe functions for some reason
//! do not meet your needs.
//! These functions only do the basic methods of abstraction, such as returning results and pairs
//! instead of taking mutable pointers for integers

extern crate neutron_star_rt;
extern crate neutron_star_constants;

use neutron_star_rt::*;
use neutron_star_constants::*;
use core::mem::transmute;

pub enum SystemError{
    Generic(u32),
    UnexpectedSize,
}

pub fn push_sccs(data: &[u8]) -> Result<(), SystemError>{
    unsafe{
        let size = data.len();
        let ptr = data.as_ptr();
        let e = __neutron_syscall_short(NeutronSyscalls::PushSCCS as u32, ptr as u32, size as u32, 0);
        if e != 0{
            Err(SystemError::Generic(e))
        }else{
            Ok(())
        }
    }
}

pub fn pop_sccs(buffer: &mut [u8]) -> Result<u32, SystemError>{
    unsafe{
        let mut actual_size:u32 = 0;
        let actual_size_pointer: *mut u32 = &mut actual_size;
        let size = buffer.len();
        let ptr = buffer.as_mut_ptr();
        let e = __neutron_syscall_short(NeutronSyscalls::PopSCCS as u32, ptr as u32, size as u32, actual_size_pointer as u32);
        if e != 0{
            Err(SystemError::Generic(e))
        }else{
            Ok(actual_size)
        }
    }
}

/// Pops an exactly u64 value from the stack. 
pub fn pop_sccs_u64() -> Result<u64, SystemError>{
    const SIZE:usize = 8;
    let mut buffer = [0; SIZE];
    let result = pop_sccs(&mut buffer)?;
    if result != SIZE as u32{
        return Err(SystemError::UnexpectedSize);
    }
    return Ok(unsafe { transmute::<[u8; SIZE], u64>(buffer) }.to_le());
}

/// Pops an exactly u32 value from the stack. 
pub fn pop_sccs_u32() -> Result<u32, SystemError>{
    const SIZE:usize = 4;
    let mut buffer = [0; SIZE];
    let result = pop_sccs(&mut buffer)?;
    if result != SIZE as u32{
        return Err(SystemError::UnexpectedSize);
    }
    return Ok(unsafe { transmute::<[u8; SIZE], u32>(buffer) }.to_le());
}

/// Pops an exactly u16 value from the stack. 
pub fn pop_sccs_u16() -> Result<u16, SystemError>{
    const SIZE:usize = 2;
    let mut buffer = [0; SIZE];
    let result = pop_sccs(&mut buffer)?;
    if result != SIZE as u32{
        return Err(SystemError::UnexpectedSize);
    }
    return Ok(unsafe { transmute::<[u8; SIZE], u16>(buffer) }.to_le());
}

/// Pops an exactly u8 value from the stack. 
pub fn pop_sccs_u8() -> Result<u8, SystemError>{
    const SIZE:usize = 1;
    let mut buffer = [0];
    let result = pop_sccs(&mut buffer)?;
    if result != SIZE as u32{
        return Err(SystemError::UnexpectedSize);
    }
    return Ok(buffer[0]);
}


