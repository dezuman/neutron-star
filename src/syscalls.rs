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
use alloc::vec::*;

#[derive(core::fmt::Debug)]
pub enum SystemError{
    Generic(u32),
    UnexpectedSize,
}

pub fn push_sccs(data: &[u8]) -> Result<(), SystemError>{
    unsafe{
        let size = data.len();
        let ptr = data.as_ptr();
        let e = __push_sccs(ptr, size);
        if e == 0 {
            Ok(())
        }else{
            Err(SystemError::Generic(e))
        }
    }
}

pub fn pop_sccs_fixed(buffer: &mut [u8]) -> Result<u32, SystemError>{
    unsafe{
        let size = buffer.len();
        let ptr = buffer.as_mut_ptr();
        let actual_size = __pop_sccs(ptr, size) as u32;
        if actual_size > 0x8000_0000{
            Err(SystemError::Generic(actual_size))
        }else{
            Ok(actual_size)
        }
    }
}

pub fn pop_sccs() -> Result<Vec<u8>, SystemError>{
    unsafe{
        let actual_size = __peek_sccs(0 as *mut u8, 0, 0);
        let mut data = vec![];
        data.resize(actual_size, 0);
        let actual_size = __pop_sccs(data.as_mut_ptr(), actual_size) as u32;
        if actual_size > 0x8000_0000{
            Err(SystemError::Generic(actual_size))
        }else{
            Ok(data)
        }
    }
}

/// Pops an exactly u64 value from the stack. 
pub fn pop_sccs_u64() -> Result<u64, SystemError>{
    const SIZE:usize = 8;
    let mut buffer = [0; SIZE];
    let result = pop_sccs_fixed(&mut buffer)?;
    if result != SIZE as u32{
        return Err(SystemError::UnexpectedSize);
    }
    return Ok(unsafe { transmute::<[u8; SIZE], u64>(buffer) }.to_le());
}

/// Pops an exactly u32 value from the stack. 
pub fn pop_sccs_u32() -> Result<u32, SystemError>{
    const SIZE:usize = 4;
    let mut buffer = [0; SIZE];
    let result = pop_sccs_fixed(&mut buffer)?;
    if result != SIZE as u32{
        return Err(SystemError::UnexpectedSize);
    }
    return Ok(unsafe { transmute::<[u8; SIZE], u32>(buffer) }.to_le());
}

/// Pops an exactly u16 value from the stack. 
pub fn pop_sccs_u16() -> Result<u16, SystemError>{
    const SIZE:usize = 2;
    let mut buffer = [0; SIZE];
    let result = pop_sccs_fixed(&mut buffer)?;
    if result != SIZE as u32{
        return Err(SystemError::UnexpectedSize);
    }
    return Ok(unsafe { transmute::<[u8; SIZE], u16>(buffer) }.to_le());
}

/// Pops an exactly u8 value from the stack. 
pub fn pop_sccs_u8() -> Result<u8, SystemError>{
    const SIZE:usize = 1;
    let mut buffer = [0];
    let result = pop_sccs_fixed(&mut buffer)?;
    if result != SIZE as u32{
        return Err(SystemError::UnexpectedSize);
    }
    return Ok(buffer[0]);
}
/// Pops a NeutronShortAddress value from the stack. 
pub fn pop_sccs_address() -> Result<NeutronShortAddress, SystemError>{
    const SIZE:usize = core::mem::size_of::<NeutronShortAddress>();
    let mut buffer = [0; SIZE];
    let result = pop_sccs_fixed(&mut buffer)?;
    if result != SIZE as u32{
        return Err(SystemError::UnexpectedSize);
    }
    return Ok(unsafe { transmute::<[u8; SIZE], NeutronShortAddress>(buffer)});
}

/// Pushes a NeutronShortAddress to the stack. 
pub fn push_sccs_address(value: &NeutronShortAddress) -> Result<(), SystemError>{
    const SIZE:usize = core::mem::size_of::<NeutronShortAddress>();
    let t = unsafe{
        transmute::<NeutronShortAddress, [u8; SIZE]>(*value)
    };
    let result = push_sccs(&t);
    if result.is_err(){
        return Err(result.err().unwrap());
    }else{
        return Ok(());
    }
}

/// Pushes a u64 value to the stack. 
pub fn push_sccs_u64(value: u64) -> Result<(), SystemError>{
    const SIZE:usize = 8;
    let t = unsafe{
        transmute::<u64, [u8; SIZE]>(value)
    };
    let result = push_sccs(&t);
    if result.is_err(){
        return Err(result.err().unwrap());
    }else{
        return Ok(());
    }
}
/// Pushes a u32 value to the stack. 
pub fn push_sccs_u32(value: u32) -> Result<(), SystemError>{
    const SIZE:usize = 4;
    let t = unsafe{
        transmute::<u32, [u8; SIZE]>(value)
    };
    let result = push_sccs(&t);
    if result.is_err(){
        return Err(result.err().unwrap());
    }else{
        return Ok(());
    }
}

/// Pushes a u64 value to the stack. 
pub fn push_sccs_u16(value: u16) -> Result<(), SystemError>{
    const SIZE:usize = 2;
    let t = unsafe{
        transmute::<u16, [u8; SIZE]>(value)
    };
    let result = push_sccs(&t);
    if result.is_err(){
        return Err(result.err().unwrap());
    }else{
        return Ok(());
    }
}

/// Pushes a u64 value to the stack. 
pub fn push_sccs_u8(value: u8) -> Result<(), SystemError>{
    const SIZE:usize = 1;
    let t = unsafe{
        transmute::<u8, [u8; SIZE]>(value)
    };
    let result = push_sccs(&t);
    if result.is_err(){
        return Err(result.err().unwrap());
    }else{
        return Ok(());
    }
}

pub fn get_self_address() -> NeutronShortAddress{
    //TODO
    return NeutronShortAddress::default();
}

pub fn _system_call(feature: u32, function: u32) -> Result<u32, SystemError>{
    unsafe{
        let result = __system_call(feature, function);
        if result >= 0x8000_0000{
            Err(SystemError::Generic(result))
        }else{
            Ok(result)
        }
    }
}

