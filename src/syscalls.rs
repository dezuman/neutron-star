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
use core::slice;

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

/************************************
**                                 **
**  Costack abstraction functions  **
**                                 **
************************************/

// pop_costack_XXX()

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

/// Pops an exact u8 value from the stack.
pub fn pop_costack_u8() -> Result<u8, RecoverableError> {
    pop_costack_typed!(u8)
}

/// Pops an exact u16 value from the stack.
pub fn pop_costack_u16() -> Result<u16, RecoverableError> {
    pop_costack_typed!(u16)
}

/// Pops an exact u32 value from the stack.
pub fn pop_costack_u32() -> Result<u32, RecoverableError> {
    pop_costack_typed!(u32)
}

/// Pops an exact u64 value from the stack.
pub fn pop_costack_u64() -> Result<u64, RecoverableError> {
    pop_costack_typed!(u64)
}

/// Pops an exact i8 value from the stack.
pub fn pop_costack_i8() -> Result<i8, RecoverableError> {
    pop_costack_typed!(i8)
}

/// Pops an exact i16 value from the stack.
pub fn pop_costack_i16() -> Result<i16, RecoverableError> {
    pop_costack_typed!(i16)
}

/// Pops an exact i32 value from the stack.
pub fn pop_costack_i32() -> Result<i32, RecoverableError> {
    pop_costack_typed!(i32)
}

/// Pops an exact i64 value from the stack.
pub fn pop_costack_i64() -> Result<i64, RecoverableError> {
    pop_costack_typed!(i64)
}

/// Pops an exact NeutronAddress value from the stack.
pub fn pop_costack_address() -> Result<NeutronAddress, RecoverableError> {
    pop_costack_typed!(NeutronAddress)
}

// pop_costack_fixed_array_XXX(array slice)

// TODO: This macro can probably be refactored to be less messy
// It basically casts a slice of a numeric array to a byte slice, then pops a costack value into it.
macro_rules! pop_costack_fixed_array_typed {
    ($SLICE:ident, $TYPE:tt) => {{
        const SIZE: usize = core::mem::size_of::<$TYPE>();

        let pointer = &mut$SLICE[0] as *mut $TYPE as *mut u8;
        let byte_slice = unsafe { slice::from_raw_parts_mut(pointer, $SLICE.len() * SIZE) };

        let result = match pop_costack_fixed(byte_slice) {
            Ok(v) => v,
            Err(_e) => return Err(RecoverableError::ItemDoesntExist),
        };

        // For these functions we only allow the exact expected byte count
        if result > ($SLICE.len() * SIZE) as u32 {
            return Err(RecoverableError::StackItemTooLarge);
        } else if result < ($SLICE.len() * SIZE) as u32 {
            return Err(RecoverableError::StackItemTooSmall);
        }

        return Ok(());
    }};
}

/// Pops a fixed size u8 array from the stack.
pub fn pop_costack_fixed_array_u8(slice: &mut [u8]) -> Result<(), RecoverableError> {
    pop_costack_fixed_array_typed!(slice, u8)
}

/// Pops a fixed size u16 array from the stack.
pub fn pop_costack_fixed_array_u16(slice: &mut [u16]) -> Result<(), RecoverableError> {
    pop_costack_fixed_array_typed!(slice, u16)
}

/// Pops a fixed size u32 array from the stack.
pub fn pop_costack_fixed_array_u32(slice: &mut [u32]) -> Result<(), RecoverableError> {
    pop_costack_fixed_array_typed!(slice, u32)
}

/// Pops a fixed size u64 array from the stack.
pub fn pop_costack_fixed_array_u64(slice: &mut [u64]) -> Result<(), RecoverableError> {
    pop_costack_fixed_array_typed!(slice, u64)
}

/// Pops a fixed size i8 array from the stack.
pub fn pop_costack_fixed_array_i8(slice: &mut [i8]) -> Result<(), RecoverableError> {
    pop_costack_fixed_array_typed!(slice, i8)
}

/// Pops a fixed size i16 array from the stack.
pub fn pop_costack_fixed_array_i16(slice: &mut [i16]) -> Result<(), RecoverableError> {
    pop_costack_fixed_array_typed!(slice, i16)
}

/// Pops a fixed size i32 array from the stack.
pub fn pop_costack_fixed_array_i32(slice: &mut [i32]) -> Result<(), RecoverableError> {
    pop_costack_fixed_array_typed!(slice, i32)
}

/// Pops a fixed size i64 array from the stack.
pub fn pop_costack_fixed_array_i64(slice: &mut [i64]) -> Result<(), RecoverableError> {
    pop_costack_fixed_array_typed!(slice, i64)
}

/// Pops a fixed size NeutronAddress array from the stack.
pub fn pop_costack_fixed_array_address(
    slice: &mut [NeutronAddress],
) -> Result<(), RecoverableError> {
    pop_costack_fixed_array_typed!(slice, NeutronAddress)
}

// push_costack_XXX(value)

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

/// Push an exact u8 value to the stack.
pub fn push_costack_u8(value: u8) {
    push_costack_typed!(value, u8);
}

/// Push an exact u16 value to the stack.
pub fn push_costack_u16(value: u16) {
    push_costack_typed!(value, u16);
}

/// Push an exact u32 value to the stack.
pub fn push_costack_u32(value: u32) {
    push_costack_typed!(value, u32);
}

/// Push an exact u64 value to the stack.
pub fn push_costack_u64(value: u64) {
    push_costack_typed!(value, u64);
}

/// Push an exact i8 value to the stack.
pub fn push_costack_i8(value: i8) {
    push_costack_typed!(value, i8);
}

/// Push an exact i16 value to the stack.
pub fn push_costack_i16(value: i16) {
    push_costack_typed!(value, i16);
}

/// Push an exact i32 value to the stack.
pub fn push_costack_i32(value: i32) {
    push_costack_typed!(value, i32);
}

/// Push an exact i64 value to the stack.
pub fn push_costack_i64(value: i64) {
    push_costack_typed!(value, i64);
}

/// Pushes an exact NeutronAddress to the stack.
pub fn push_costack_address(value: &NeutronAddress) {
    push_costack_typed!(*value, NeutronAddress);
}

// push_costack_array_XXX(array slice)

macro_rules! push_costack_array_typed {
    ($SLICE:ident, $TYPE:tt) => {{
        const SIZE: usize = core::mem::size_of::<$TYPE>();

        let pointer = &$SLICE[0] as *const $TYPE as *const u8;
        let byte_slice = unsafe { slice::from_raw_parts(pointer, $SLICE.len() * SIZE) };

        push_costack(byte_slice);
    }};
}

/// Pushes an u8 array to the stack.
pub fn push_costack_array_u8(value: &[u8]) {
    push_costack(value); // No need for the macro since the slice is already byte sized
}

/// Pushes an u16 array to the stack.
pub fn push_costack_array_u16(value: &[u16]) {
    push_costack_array_typed!(value, u16);
}

/// Pushes an u32 array to the stack.
pub fn push_costack_array_u32(value: &[u32]) {
    push_costack_array_typed!(value, u32);
}

/// Pushes an u64 array to the stack.
pub fn push_costack_array_u64(value: &[u64]) {
    push_costack_array_typed!(value, u64);
}

/// Pushes an i8 array to the stack.
pub fn push_costack_array_i8(value: &[i8]) {
    push_costack_array_typed!(value, i8);
}

/// Pushes an i16 array to the stack.
pub fn push_costack_array_i16(value: &[i16]) {
    push_costack_array_typed!(value, i16);
}

/// Pushes an i32 array to the stack.
pub fn push_costack_array_i32(value: &[i32]) {
    push_costack_array_typed!(value, i32);
}

/// Pushes an i64 array to the stack.
pub fn push_costack_array_i64(value: &[i64]) {
    push_costack_array_typed!(value, i64);
}

/// Pushes a NeutronAddress array to the stack.
pub fn push_costack_array_address(value: &[NeutronAddress]) {
    push_costack_array_typed!(value, NeutronAddress);
}

/*****************************************
**                                      **
**  Simple comap abstraction functions  **
**                                      **
*****************************************/

// ABI value constants
// TODO: Move to a more unified ABI helper library???

// All these are 1 byte headers -> only top byte is used, as following:
// Bits 7-6 are 0   -> 1 byte header
// Bit 5 is 0       -> numeric type
// Bit 4 is 0       -> not hex/bignum display
// Bit 3 is 0       -> not array
// Bits 2-0 determine the actual type

pub const ABI_VALUE_U8: u32 = 0b0000_0000;
pub const ABI_VALUE_I8: u32 = 0b0000_0100;
pub const ABI_VALUE_U16: u32 = 0b0000_0010;
pub const ABI_VALUE_I16: u32 = 0b0000_0110;
pub const ABI_VALUE_U32: u32 = 0b0000_0001;
pub const ABI_VALUE_I32: u32 = 0b0000_0101;
pub const ABI_VALUE_U64: u32 = 0b0000_0011;
pub const ABI_VALUE_I64: u32 = 0b0000_0111;

// OR (or add...) above type value with this to set byte indicating array value
pub const ABI_ARRAY_BIT: u32 = 0b0000_1000;

// write_comap_XXX(key, value)

macro_rules! write_comap_typed_with_abi {
    ($KEY:ident, $VALUE:ident, $TYPE:tt, $ABI_VALUE:expr) => {{
        unsafe {
            push_costack($KEY.as_bytes());
            push_costack_typed!($VALUE, $TYPE);
            __push_comap($ABI_VALUE);
        }
    }};
}

/// Write a u8 comap value
pub fn write_comap_u8(key: &str, value: u8) {
    write_comap_typed_with_abi!(key, value, u8, ABI_VALUE_U8)
}

/// Write a u16 comap value
pub fn write_comap_u16(key: &str, value: u16) {
    write_comap_typed_with_abi!(key, value, u16, ABI_VALUE_U16)
}

/// Write a u32 comap value
pub fn write_comap_u32(key: &str, value: u32) {
    write_comap_typed_with_abi!(key, value, u32, ABI_VALUE_U32)
}

/// Write a u64 comap value
pub fn write_comap_u64(key: &str, value: u64) {
    write_comap_typed_with_abi!(key, value, u64, ABI_VALUE_U64)
}

/// Write a i8 comap value
pub fn write_comap_i8(key: &str, value: i8) {
    write_comap_typed_with_abi!(key, value, i8, ABI_VALUE_I8)
}

/// Write a i16 comap value
pub fn write_comap_i16(key: &str, value: i16) {
    write_comap_typed_with_abi!(key, value, i16, ABI_VALUE_I16)
}

/// Write a i32 comap value
pub fn write_comap_i32(key: &str, value: i32) {
    write_comap_typed_with_abi!(key, value, i32, ABI_VALUE_I32)
}

/// Write a i64 comap value
pub fn write_comap_i64(key: &str, value: i64) {
    write_comap_typed_with_abi!(key, value, i64, ABI_VALUE_I64)
}

/// Write a NeutronAddress comap value
pub fn write_comap_address(key: &str, value: NeutronAddress) {
    write_comap_typed_with_abi!(key, value, NeutronAddress, ABI_VALUE_I8)
}

// write_comap_array_XXX(key, array slice)

macro_rules! write_comap_array_typed_with_abi {
    ($KEY:ident, $SLICE:ident, $TYPE:tt, $ABI_VALUE:expr) => {{
        unsafe {
            push_costack($KEY.as_bytes());
            push_costack_array_typed!($SLICE, $TYPE);
            __push_comap($ABI_VALUE);
        }
    }};
}

/// Write a u8 comap array
pub fn write_comap_array_u8(key: &str, value_slice: &[u8]) {
    write_comap_array_typed_with_abi!(key, value_slice, u8, ABI_VALUE_U8 + ABI_ARRAY_BIT)
}

/// Write a u16 comap array
pub fn write_comap_array_u16(key: &str, value_slice: &[u16]) {
    write_comap_array_typed_with_abi!(key, value_slice, u16, ABI_VALUE_U16 + ABI_ARRAY_BIT)
}

/// Write a u32 comap array
pub fn write_comap_array_u32(key: &str, value_slice: &[u32]) {
    write_comap_array_typed_with_abi!(key, value_slice, u32, ABI_VALUE_U32 + ABI_ARRAY_BIT)
}

/// Write a u64 comap array
pub fn write_comap_array_u64(key: &str, value_slice: &[u64]) {
    write_comap_array_typed_with_abi!(key, value_slice, u64, ABI_VALUE_U64 + ABI_ARRAY_BIT)
}

/// Write a i8 comap array
pub fn write_comap_array_i8(key: &str, value_slice: &[i8]) {
    write_comap_array_typed_with_abi!(key, value_slice, i8, ABI_VALUE_I8 + ABI_ARRAY_BIT)
}

/// Write a i16 comap array
pub fn write_comap_array_i16(key: &str, value_slice: &[i16]) {
    write_comap_array_typed_with_abi!(key, value_slice, i16, ABI_VALUE_I16 + ABI_ARRAY_BIT)
}

/// Write a i32 comap array
pub fn write_comap_array_i32(key: &str, value_slice: &[i32]) {
    write_comap_array_typed_with_abi!(key, value_slice, i32, ABI_VALUE_I32 + ABI_ARRAY_BIT)
}

/// Write a i64 comap array
pub fn write_comap_array_i64(key: &str, value_slice: &[i64]) {
    write_comap_array_typed_with_abi!(key, value_slice, i64, ABI_VALUE_I64 + ABI_ARRAY_BIT)
}

// read_comap_XXX(key)

// These two variants are identical save for reading either the input or result comap
macro_rules! read_comap_typed_with_abi {
    ($KEY:ident, $TYPE:tt, "input_map") => {{
        unsafe {
            push_costack($KEY.as_bytes());
            const BEGIN: usize = 0;
            const SIZE: usize = core::mem::size_of::<$TYPE>();
            __peek_comap(BEGIN, SIZE)
        }
    }};
    ($KEY:ident, $TYPE:tt, "result_map") => {{
        unsafe {
            push_costack($KEY.as_bytes());
            const BEGIN: usize = 0;
            const SIZE: usize = core::mem::size_of::<$TYPE>();
            __peek_result_comap(BEGIN, SIZE)
        }
    }};
}

/// Attempt to read a u8 input comap value
pub fn read_comap_u8(key: &str) -> Result<u8, RecoverableError> {
    match read_comap_typed_with_abi!(key, u8, "input_map") {
        ABI_VALUE_U8 => pop_costack_u8(),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

/// Attempt to read a u16 input comap value
pub fn read_comap_u16(key: &str) -> Result<u16, RecoverableError> {
    match read_comap_typed_with_abi!(key, u16, "input_map") {
        ABI_VALUE_U16 => pop_costack_u16(),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

/// Attempt to read a u32 input comap value
pub fn read_comap_u32(key: &str) -> Result<u32, RecoverableError> {
    match read_comap_typed_with_abi!(key, u32, "input_map") {
        ABI_VALUE_U32 => pop_costack_u32(),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

/// Attempt to read a u64 input comap value
pub fn read_comap_u64(key: &str) -> Result<u64, RecoverableError> {
    match read_comap_typed_with_abi!(key, u64, "input_map") {
        ABI_VALUE_U64 => pop_costack_u64(),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

/// Attempt to read a i8 input comap value
pub fn read_comap_i8(key: &str) -> Result<i8, RecoverableError> {
    match read_comap_typed_with_abi!(key, i8, "input_map") {
        ABI_VALUE_I8 => pop_costack_i8(),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

/// Attempt to read a i16 input comap value
pub fn read_comap_i16(key: &str) -> Result<i16, RecoverableError> {
    match read_comap_typed_with_abi!(key, i16, "input_map") {
        ABI_VALUE_I16 => pop_costack_i16(),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

/// Attempt to read a i32 input comap value
pub fn read_comap_i32(key: &str) -> Result<i32, RecoverableError> {
    match read_comap_typed_with_abi!(key, i32, "input_map") {
        ABI_VALUE_I32 => pop_costack_i32(),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

/// Attempt to read a i64 input comap value
pub fn read_comap_i64(key: &str) -> Result<i64, RecoverableError> {
    match read_comap_typed_with_abi!(key, i64, "input_map") {
        ABI_VALUE_I64 => pop_costack_i64(),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

// read_result_comap_XXX

/// Attempt to read a u8 result comap value
pub fn read_result_comap_u8(key: &str) -> Result<u8, RecoverableError> {
    match read_comap_typed_with_abi!(key, u8, "result_map") {
        ABI_VALUE_U8 => pop_costack_u8(),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

/// Attempt to read a u16 result comap value
pub fn read_result_comap_u16(key: &str) -> Result<u16, RecoverableError> {
    match read_comap_typed_with_abi!(key, u16, "result_map") {
        ABI_VALUE_U16 => pop_costack_u16(),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

/// Attempt to read a u32 result comap value
pub fn read_result_comap_u32(key: &str) -> Result<u32, RecoverableError> {
    match read_comap_typed_with_abi!(key, u32, "result_map") {
        ABI_VALUE_U32 => pop_costack_u32(),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

/// Attempt to read a u64 result comap value
pub fn read_result_comap_u64(key: &str) -> Result<u64, RecoverableError> {
    match read_comap_typed_with_abi!(key, u64, "result_map") {
        ABI_VALUE_U64 => pop_costack_u64(),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

/// Attempt to read a i8 result comap value
pub fn read_result_comap_i8(key: &str) -> Result<i8, RecoverableError> {
    match read_comap_typed_with_abi!(key, i8, "result_map") {
        ABI_VALUE_I8 => pop_costack_i8(),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

/// Attempt to read a i16 result comap value
pub fn read_result_comap_i16(key: &str) -> Result<i16, RecoverableError> {
    match read_comap_typed_with_abi!(key, i16, "result_map") {
        ABI_VALUE_I16 => pop_costack_i16(),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

/// Attempt to read a i32 result comap value
pub fn read_result_comap_i32(key: &str) -> Result<i32, RecoverableError> {
    match read_comap_typed_with_abi!(key, i32, "result_map") {
        ABI_VALUE_I32 => pop_costack_i32(),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

/// Attempt to read a i64 result comap value
pub fn read_result_comap_i64(key: &str) -> Result<i64, RecoverableError> {
    match read_comap_typed_with_abi!(key, i64, "result_map") {
        ABI_VALUE_I64 => pop_costack_i64(),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

// read_comap_fixed_array_XXX(key, array slice)

macro_rules! read_comap_fixed_array_typed_with_abi {
    ($KEY:ident, $SLICE_LENGTH:expr, $TYPE:tt, "input_map") => {{
        unsafe {
            push_costack($KEY.as_bytes());
            const BEGIN: usize = 0;
            const SIZE: usize = core::mem::size_of::<$TYPE>();
            __peek_comap(BEGIN, $SLICE_LENGTH * SIZE)
        }
    }};
    ($KEY:ident, $SLICE_LENGTH:expr, $TYPE:tt, "result_map") => {{
        unsafe {
            push_costack($KEY.as_bytes());
            const BEGIN: usize = 0;
            const SIZE: usize = core::mem::size_of::<$TYPE>();
            __peek_result_comap(BEGIN, $SLICE_LENGTH * SIZE)
        }
    }};
}

/// Attempt to read a u8 input comap array
pub fn read_comap_fixed_array_u8(
    key: &str,
    return_slice: &mut [u8],
) -> Result<(), RecoverableError> {
    const MATCH_VAL: u32 = ABI_VALUE_U8 + ABI_ARRAY_BIT;
    match read_comap_fixed_array_typed_with_abi!(key, return_slice.len(), u8, "input_map") {
        MATCH_VAL => pop_costack_fixed_array_u8(return_slice),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

/// Attempt to read a u16 input comap array
pub fn read_comap_fixed_array_u16(
    key: &str,
    return_slice: &mut [u16],
) -> Result<(), RecoverableError> {
    const MATCH_VAL: u32 = ABI_VALUE_U16 + ABI_ARRAY_BIT;
    match read_comap_fixed_array_typed_with_abi!(key, return_slice.len(), u16, "input_map") {
        MATCH_VAL => pop_costack_fixed_array_u16(return_slice),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

/// Attempt to read a u32 input comap array
pub fn read_comap_fixed_array_u32(
    key: &str,
    return_slice: &mut [u32],
) -> Result<(), RecoverableError> {
    const MATCH_VAL: u32 = ABI_VALUE_U32 + ABI_ARRAY_BIT;
    match read_comap_fixed_array_typed_with_abi!(key, return_slice.len(), u32, "input_map") {
        MATCH_VAL => pop_costack_fixed_array_u32(return_slice),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

/// Attempt to read a u64 input comap array
pub fn read_comap_fixed_array_u64(
    key: &str,
    return_slice: &mut [u64],
) -> Result<(), RecoverableError> {
    const MATCH_VAL: u32 = ABI_VALUE_U64 + ABI_ARRAY_BIT;
    match read_comap_fixed_array_typed_with_abi!(key, return_slice.len(), u64, "input_map") {
        MATCH_VAL => pop_costack_fixed_array_u64(return_slice),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

/// Attempt to read a i8 input comap array
pub fn read_comap_fixed_array_i8(
    key: &str,
    return_slice: &mut [i8],
) -> Result<(), RecoverableError> {
    const MATCH_VAL: u32 = ABI_VALUE_I8 + ABI_ARRAY_BIT;
    match read_comap_fixed_array_typed_with_abi!(key, return_slice.len(), i8, "input_map") {
        MATCH_VAL => pop_costack_fixed_array_i8(return_slice),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

/// Attempt to read a i16 input comap array
pub fn read_comap_fixed_array_i16(
    key: &str,
    return_slice: &mut [i16],
) -> Result<(), RecoverableError> {
    const MATCH_VAL: u32 = ABI_VALUE_I16 + ABI_ARRAY_BIT;
    match read_comap_fixed_array_typed_with_abi!(key, return_slice.len(), i16, "input_map") {
        MATCH_VAL => pop_costack_fixed_array_i16(return_slice),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

/// Attempt to read a i32 input comap array
pub fn read_comap_fixed_array_i32(
    key: &str,
    return_slice: &mut [i32],
) -> Result<(), RecoverableError> {
    const MATCH_VAL: u32 = ABI_VALUE_I32 + ABI_ARRAY_BIT;
    match read_comap_fixed_array_typed_with_abi!(key, return_slice.len(), i32, "input_map") {
        MATCH_VAL => pop_costack_fixed_array_i32(return_slice),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

/// Attempt to read a i64 input comap array
pub fn read_comap_fixed_array_i64(
    key: &str,
    return_slice: &mut [i64],
) -> Result<(), RecoverableError> {
    const MATCH_VAL: u32 = ABI_VALUE_I64 + ABI_ARRAY_BIT;
    match read_comap_fixed_array_typed_with_abi!(key, return_slice.len(), i64, "input_map") {
        MATCH_VAL => pop_costack_fixed_array_i64(return_slice),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

// read_result_comap_fixed_array_XXX(key, array slice)

/// Attempt to read a u8 result comap array
pub fn read_result_comap_fixed_array_u8(
    key: &str,
    return_slice: &mut [u8],
) -> Result<(), RecoverableError> {
    const MATCH_VAL: u32 = ABI_VALUE_U8 + ABI_ARRAY_BIT;
    match read_comap_fixed_array_typed_with_abi!(key, return_slice.len(), u8, "result_map") {
        MATCH_VAL => pop_costack_fixed_array_u8(return_slice),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

/// Attempt to read a u16 result comap array
pub fn read_result_comap_fixed_array_u16(
    key: &str,
    return_slice: &mut [u16],
) -> Result<(), RecoverableError> {
    const MATCH_VAL: u32 = ABI_VALUE_U16 + ABI_ARRAY_BIT;
    match read_comap_fixed_array_typed_with_abi!(key, return_slice.len(), u16, "result_map") {
        MATCH_VAL => pop_costack_fixed_array_u16(return_slice),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

/// Attempt to read a u32 result comap array
pub fn read_result_comap_fixed_array_u32(
    key: &str,
    return_slice: &mut [u32],
) -> Result<(), RecoverableError> {
    const MATCH_VAL: u32 = ABI_VALUE_U32 + ABI_ARRAY_BIT;
    match read_comap_fixed_array_typed_with_abi!(key, return_slice.len(), u32, "result_map") {
        MATCH_VAL => pop_costack_fixed_array_u32(return_slice),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

/// Attempt to read a u64 result comap array
pub fn read_result_comap_fixed_array_u64(
    key: &str,
    return_slice: &mut [u64],
) -> Result<(), RecoverableError> {
    const MATCH_VAL: u32 = ABI_VALUE_U64 + ABI_ARRAY_BIT;
    match read_comap_fixed_array_typed_with_abi!(key, return_slice.len(), u64, "result_map") {
        MATCH_VAL => pop_costack_fixed_array_u64(return_slice),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

/// Attempt to read a i8 result comap array
pub fn read_result_comap_fixed_array_i8(
    key: &str,
    return_slice: &mut [i8],
) -> Result<(), RecoverableError> {
    const MATCH_VAL: u32 = ABI_VALUE_I8 + ABI_ARRAY_BIT;
    match read_comap_fixed_array_typed_with_abi!(key, return_slice.len(), i8, "result_map") {
        MATCH_VAL => pop_costack_fixed_array_i8(return_slice),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

/// Attempt to read a i16 result comap array
pub fn read_result_comap_fixed_array_i16(
    key: &str,
    return_slice: &mut [i16],
) -> Result<(), RecoverableError> {
    const MATCH_VAL: u32 = ABI_VALUE_I16 + ABI_ARRAY_BIT;
    match read_comap_fixed_array_typed_with_abi!(key, return_slice.len(), i16, "result_map") {
        MATCH_VAL => pop_costack_fixed_array_i16(return_slice),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

/// Attempt to read a i32 result comap array
pub fn read_result_comap_fixed_array_i32(
    key: &str,
    return_slice: &mut [i32],
) -> Result<(), RecoverableError> {
    const MATCH_VAL: u32 = ABI_VALUE_I32 + ABI_ARRAY_BIT;
    match read_comap_fixed_array_typed_with_abi!(key, return_slice.len(), i32, "result_map") {
        MATCH_VAL => pop_costack_fixed_array_i32(return_slice),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
}

/// Attempt to read a i64 result comap array
pub fn read_result_comap_fixed_array_i64(
    key: &str,
    return_slice: &mut [i64],
) -> Result<(), RecoverableError> {
    const MATCH_VAL: u32 = ABI_VALUE_I64 + ABI_ARRAY_BIT;
    match read_comap_fixed_array_typed_with_abi!(key, return_slice.len(), i64, "result_map") {
        MATCH_VAL => pop_costack_fixed_array_i64(return_slice),
        _ => Err(RecoverableError::ItemDoesntExist), // TODO: Custom neutron-star error
    }
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
