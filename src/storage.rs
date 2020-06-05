use crate::syscalls::*;
use alloc::vec::*;
/*
## Global Storage

ID: 1

Functions:

* store_state(key, value) -> ()
* load_state(key) -> (value)
* key_exists(key) -> (bool)
*/

const GLOBAL_STORAGE_FEATURE: u32 = 1;

pub enum GlobalStorageFunctions{
    Available = 0, //reserved??
    StoreState = 1,
    LoadState,
    KeyExists
}

pub fn store_state(key: &[u8], value: &[u8]) -> Result<(), SystemError>{
    push_sccs(value)?;
    push_sccs(key)?;
    _system_call(GLOBAL_STORAGE_FEATURE, GlobalStorageFunctions::StoreState as u32)?;
    Ok(())
}

pub fn load_state_fixed(key: &[u8], value: &mut [u8]) -> Result<usize, SystemError>{
    push_sccs(key)?;
    _system_call(GLOBAL_STORAGE_FEATURE, GlobalStorageFunctions::StoreState as u32)?;
    Ok(pop_sccs_fixed(value)? as usize)
}

pub fn load_state(key: &[u8]) -> Result<Vec<u8>, SystemError>{
    push_sccs(key)?;
    _system_call(GLOBAL_STORAGE_FEATURE, GlobalStorageFunctions::StoreState as u32)?;
    pop_sccs()
}