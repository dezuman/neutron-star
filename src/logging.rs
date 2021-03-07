use crate::syscalls::*;
/*
## Logging

ID: 2

Functions:

* log_debug(count, string, ...)
* log_info(count, string, ...)
* log_warning(count, string, ...)
* log_error(count, string, ...)

The exact order of printing messages is backward from what would be expected!
This is designed so that no allocator is required for doing `println!` functions within neutron-star.

The expense of reordering the strings etc is a cost on the CallSystem. This could potentially be somewhat expensive, 
but since logging is informative only and can easily be a no-op (other than needing to pop off appropriate number of stack items) this incurs no real risk.

Note in neutron-star, log_info is used by default for println!
*/

const LOGGING_ELEMENT: u32 = 4;

enum LoggingFunctions{
    LogDebug = 1,
    LogInfo,
    LogWarning,
    LogError
}


pub fn log_debug(msg: &str){
    push_costack(msg.as_bytes());
    push_costack(&[1]);
    _system_call(LOGGING_ELEMENT, LoggingFunctions::LogDebug as u32).unwrap();
}
pub fn log_info(msg: &str){
    push_costack(msg.as_bytes());
    push_costack(&[1]);
    _system_call(LOGGING_ELEMENT, LoggingFunctions::LogInfo as u32).unwrap();
}
pub fn log_warning(msg: &str){
    push_costack(msg.as_bytes());
    push_costack(&[1]);
    _system_call(LOGGING_ELEMENT, LoggingFunctions::LogWarning as u32).unwrap();
}
pub fn log_error(msg: &str){
    push_costack(msg.as_bytes());
    push_costack(&[1]);
    _system_call(LOGGING_ELEMENT, LoggingFunctions::LogError as u32).unwrap();
}

pub fn log_debug_from_costack(count: u8){
    push_costack(&[count]);
    _system_call(LOGGING_ELEMENT, LoggingFunctions::LogDebug as u32).unwrap();
}
pub fn log_info_from_costack(count: u8){
    push_costack(&[count]);
    _system_call(LOGGING_ELEMENT, LoggingFunctions::LogInfo as u32).unwrap();
}
pub fn log_warning_from_costack(count: u8){
    push_costack(&[count]);
    _system_call(LOGGING_ELEMENT, LoggingFunctions::LogWarning as u32).unwrap();
}
pub fn log_error_from_costack(count: u8){
    push_costack(&[count]);
    _system_call(LOGGING_ELEMENT, LoggingFunctions::LogError as u32).unwrap();
}

