use crate::syscalls::*;
/*
## Logging

ID: 2

Functions:

* log_debug(string)
* log_info(string)
* log_warning(string)
* log_error(string)

Note in neutron-star, log_info is used by default for println!
*/

const LOGGING_FEATURE: u32 = 2;

enum LoggingFunctions{
    Available = 0, //reserved??
    LogDebug = 1,
    LogInfo,
    LogWarning,
    LogError
}


pub fn log_debug(msg: &str){
    push_sccs(msg.as_bytes()).unwrap();
    _system_call(LOGGING_FEATURE, LoggingFunctions::LogDebug as u32).unwrap();
}
pub fn log_info(msg: &str){
    push_sccs(msg.as_bytes()).unwrap();
    _system_call(LOGGING_FEATURE, LoggingFunctions::LogInfo as u32).unwrap();
}
pub fn log_warning(msg: &str){
    push_sccs(msg.as_bytes()).unwrap();
    _system_call(LOGGING_FEATURE, LoggingFunctions::LogWarning as u32).unwrap();
}
pub fn log_error(msg: &str){
    push_sccs(msg.as_bytes()).unwrap();
    _system_call(LOGGING_FEATURE, LoggingFunctions::LogError as u32).unwrap();
}

