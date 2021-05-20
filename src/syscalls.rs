//! This is a collection of unsafe wrapper functions for raw system calls
//! These should only be used if the abstracted safe functions for some reason
//! do not meet your needs.
//! These functions only do the basic methods of abstraction, such as returning results and pairs
//! instead of taking mutable pointers for integers

// Comap macros using costack macros doesn't compile without nested unsafe blocks, so block warning
#![allow(unused_unsafe)]

extern crate neutron_common;
extern crate neutron_star_rt;

use core::mem::transmute;
use neutron_common::*;
use neutron_star_rt::*;

#[derive(core::fmt::Debug)]
pub enum SystemError {
    Generic(u32),
    UnexpectedSize,
}

/// Minimal wrapping around a raw hypervisor call to push_costack. (Avoid unless strictly necessary)
pub fn push_costack(data: &[u8]) {
    unsafe {
        let size = data.len();
        let ptr = data.as_ptr();
        __push_costack(ptr, size);
    }
}

/// Minimal wrapping around a raw hypervisor call to pop_costack. (Avoid unless strictly necessary)
pub fn pop_costack_fixed(buffer: &mut [u8]) -> Result<u32, SystemError> {
    unsafe {
        let size = buffer.len();
        let ptr = buffer.as_mut_ptr();
        let actual_size = __pop_costack(ptr, size) as u32;
        if actual_size > 0x8000_0000 {
            Err(SystemError::Generic(actual_size))
        } else {
            Ok(actual_size)
        }
    }
}

/// Pop and discard a value from the stack.
pub fn discard_costack() {
    unsafe {
        __pop_costack(core::ptr::null_mut(), 0);
    }
}

macro_rules! pop_costack_typed {
    ($TYPE:tt) => {{
        const SIZE: usize = core::mem::size_of::<$TYPE>();
        let mut buffer = [0; SIZE];
        let result = match pop_costack_fixed(&mut buffer) {
            Ok(v) => v,
            Err(_e) => return Err(RecoverableError::ItemDoesntExist),
        };

        // For these functions we only allow the exact expected byte count
        if result > SIZE as u32 {
            return Err(RecoverableError::StackItemTooLarge);
        } else if result < SIZE as u32 {
            return Err(RecoverableError::StackItemTooSmall);
        }

        Ok(unsafe { transmute::<[u8; SIZE], $TYPE>(buffer) })
    }};
}

/// Pops an exact u64 value from the stack.
pub fn pop_costack_u64() -> Result<u64, RecoverableError> {
    pop_costack_typed!(u64)
}

/// Pops an exact u32 value from the stack.
pub fn pop_costack_u32() -> Result<u32, RecoverableError> {
    pop_costack_typed!(u32)
}

/// Pops an exact u16 value from the stack.
pub fn pop_costack_u16() -> Result<u16, RecoverableError> {
    pop_costack_typed!(u16)
}

/// Pops an exact u8 value from the stack.
pub fn pop_costack_u8() -> Result<u8, RecoverableError> {
    pop_costack_typed!(u8)
}

/// Pops an exact i64 value from the stack.
pub fn pop_costack_i64() -> Result<i64, RecoverableError> {
    pop_costack_typed!(i64)
}

/// Pops an exact i32 value from the stack.
pub fn pop_costack_i32() -> Result<i32, RecoverableError> {
    pop_costack_typed!(i32)
}

/// Pops an exact i16 value from the stack.
pub fn pop_costack_i16() -> Result<i16, RecoverableError> {
    pop_costack_typed!(i16)
}

/// Pops an exact i8 value from the stack.
pub fn pop_costack_i8() -> Result<i8, RecoverableError> {
    pop_costack_typed!(i8)
}

/// Pops an exact NeutronAddress value from the stack.
pub fn pop_costack_address() -> Result<NeutronAddress, RecoverableError> {
    pop_costack_typed!(NeutronAddress)
}

macro_rules! push_costack_typed {
    ($VALUE:ident, $TYPE:tt) => {{
        const SIZE: usize = core::mem::size_of::<$TYPE>();
        let bytes = unsafe { transmute::<$TYPE, [u8; SIZE]>($VALUE) };
        push_costack(&bytes);
    }};
    // Handle reference-type parameters (Only NeutronAddress currently)
    (*$VALUE:ident, $TYPE:tt) => {{
        const SIZE: usize = core::mem::size_of::<$TYPE>();
        let bytes = unsafe { transmute::<$TYPE, [u8; SIZE]>(*$VALUE) };
        push_costack(&bytes);
    }};
}

/// Push an exact u64 value to the stack.
pub fn push_costack_u64(value: u64) {
    push_costack_typed!(value, u64);
}

/// Push an exact u32 value to the stack.
pub fn push_costack_u32(value: u32) {
    push_costack_typed!(value, u32);
}

/// Push an exact u16 value to the stack.
pub fn push_costack_u16(value: u16) {
    push_costack_typed!(value, u16);
}

/// Push an exact u8 value to the stack.
pub fn push_costack_u8(value: u8) {
    push_costack_typed!(value, u8);
}

/// Push an exact i64 value to the stack.
pub fn push_costack_i64(value: i64) {
    push_costack_typed!(value, i64);
}

/// Push an exact i32 value to the stack.
pub fn push_costack_i32(value: i32) {
    push_costack_typed!(value, i32);
}

/// Push an exact i16 value to the stack.
pub fn push_costack_i16(value: i16) {
    push_costack_typed!(value, i16);
}

/// Push an exact i8 value to the stack.
pub fn push_costack_i8(value: i8) {
    push_costack_typed!(value, i8);
}

/// Pushes an exact NeutronAddress to the stack.
pub fn push_costack_address(value: &NeutronAddress) {
    push_costack_typed!(*value, NeutronAddress);
}

// All these are 1 byte headers -> only top byte is used, as following:
// Bits 7-6 are 0   -> 1 byte header
// Bit 5 is 0       -> numeric type
// Bit 4 is 0       -> not hex/bignum display
// Bit 3 is 0       -> not array
// Bits 2-0 determine the actual type

// TODO: Move to a more unified ABI helper library???
pub const ABI_VALUE_U8: u32 = 0b00000000;
pub const ABI_VALUE_I8: u32 = 0b00000100;
pub const ABI_VALUE_U16: u32 = 0b00000010;
pub const ABI_VALUE_I16: u32 = 0b00000110;
pub const ABI_VALUE_U32: u32 = 0b00000001;
pub const ABI_VALUE_I32: u32 = 0b00000101;
pub const ABI_VALUE_U64: u32 = 0b00000011;
pub const ABI_VALUE_I64: u32 = 0b00000111;

// OR above type value with this to set byte indicating array value
//const ABI_ARRAY_BIT: u32    = 0b00001000 << 24;

macro_rules! write_comap_typed_with_abi {
    ($KEY:ident, $VALUE:ident, $TYPE:tt, $ABI_VALUE:expr) => {{
        unsafe {
            push_costack($KEY.as_bytes());
            push_costack_typed!($VALUE, $TYPE);
            __push_comap($ABI_VALUE);
        }
    }};
}

/// Write a u64 comap value
pub fn write_comap_u64(key: &str, value: u64) {
    write_comap_typed_with_abi!(key, value, u64, ABI_VALUE_U64)
}

/// Write a u32 comap value
pub fn write_comap_u32(key: &str, value: u32) {
    write_comap_typed_with_abi!(key, value, u32, ABI_VALUE_U32)
}

/// Write a u16 comap value
pub fn write_comap_u16(key: &str, value: u16) {
    write_comap_typed_with_abi!(key, value, u16, ABI_VALUE_U16)
}

/// Write a u8 comap value
pub fn write_comap_u8(key: &str, value: u8) {
    write_comap_typed_with_abi!(key, value, u8, ABI_VALUE_U8)
}

/// Write a i64 comap value
pub fn write_comap_i64(key: &str, value: i64) {
    write_comap_typed_with_abi!(key, value, i64, ABI_VALUE_I64)
}

/// Write a i32 comap value
pub fn write_comap_i32(key: &str, value: i32) {
    write_comap_typed_with_abi!(key, value, i32, ABI_VALUE_I32)
}

/// Write a i16 comap value
pub fn write_comap_i16(key: &str, value: i16) {
    write_comap_typed_with_abi!(key, value, i16, ABI_VALUE_I16)
}

/// Write a i8 comap value
pub fn write_comap_i8(key: &str, value: i8) {
    write_comap_typed_with_abi!(key, value, i8, ABI_VALUE_I8)
}



pub fn get_self_address() -> NeutronAddress {
    //TODO
    return NeutronAddress::default();
}

pub fn _system_call(element: u32, function: u32) -> Result<u32, SystemError> {
    unsafe {
        let result = __system_call(element, function);
        if result >= 0x8000_0000 {
            Err(SystemError::Generic(result))
        } else {
            Ok(result)
        }
    }
}
